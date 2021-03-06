use std::{cell::RefCell, collections::HashMap, convert::TryFrom, io};

use anyhow::Result;

use crate::{AstNode, Error, Expression, InputType, InterpreterError, TypeError, Value};

pub enum Return {
    None,
    Value(Value),
}

#[derive(Debug)]
pub struct Function {
    pub args: Vec<String>,
    pub block: Vec<Box<AstNode>>,
}

impl Function {
    pub fn new(args: Vec<String>, block: Vec<Box<AstNode>>) -> Self {
        Self { args, block }
    }
}

#[derive(Debug)]
pub struct Global {
    pub recursion: usize,
    pub recursion_limit: usize,
}

impl Default for Global {
    fn default() -> Self {
        Self {
            recursion: 0,
            #[cfg(not(debug_assertions))]
            recursion_limit: 4000,
            #[cfg(debug_assertions)]
            recursion_limit: 200,
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Error> {
        match value {
            Value::Void => Err(TypeError::ToBoolError("Void".into()).into()),
            Value::String(_) => Err(TypeError::ToBoolError("String".into()).into()),
            Value::Integer(_) => Err(TypeError::ToBoolError("Integer".into()).into()),
            Value::Float(_) => Err(TypeError::ToBoolError("Float".into()).into()),
            Value::Bool(boolean) => Ok(boolean),
        }
    }
}

#[derive(Debug)]
pub struct Scope<'a> {
    pub variables: RefCell<HashMap<String, Value>>,
    pub functions: RefCell<HashMap<String, Function>>,
    pub parent: Option<&'a Scope<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Self {
            variables: RefCell::new(HashMap::new()),
            functions: RefCell::new(HashMap::new()),
            parent: None,
        }
    }

    pub fn go_down(&'a self, vars: HashMap<String, Value>) -> Self {
        Self {
            variables: RefCell::new(vars),
            functions: RefCell::new(HashMap::new()),
            parent: Some(self),
        }
    }

    pub fn get_var(&self, ident: String) -> Option<Value> {
        let variables = self.variables.borrow();
        match variables.get(&ident) {
            Some(variable) => Some(variable.to_owned()),
            None => None,
        }
    }

    pub fn interpret_program(
        &self,
        program: Vec<Box<AstNode>>,
        global: &mut Global,
    ) -> Result<Return, Error> {
        let program = program.into_iter();

        for step in program {
            match self.interpret_ast(*step, global)? {
                Return::None => (),
                Return::Value(value) => return Ok(Return::Value(value)),
            }
        }
        Ok(Return::None)
    }

    pub fn interpret_fn(
        &self,
        ident: String,
        variables: Vec<Value>,
        global: &mut Global,
    ) -> Result<Value, Error> {
        if global.recursion > global.recursion_limit {
            let peak = global.recursion.clone();
            global.recursion = 0;
            return Err(InterpreterError::RecursionLimit(peak - 1).into());
        }
        global.recursion += 1;

        let mut me = self;
        loop {
            let block = match me.functions.borrow_mut().get_mut(&ident) {
                None => None,
                Some(func) => {
                    if variables.len() != func.args.len() {
                        //global.recursion = 0;
                        return Err(InterpreterError::WrongNumberOfArgs(
                            ident,
                            func.args.len(),
                            variables.len(),
                            {
                                if variables.len() == 1 {
                                    "was"
                                } else {
                                    "were"
                                }
                            }
                            .to_string(),
                        )
                        .into());
                    }

                    let mut vars = HashMap::new();
                    for (i, arg) in variables.iter().enumerate() {
                        vars.insert(func.args[i].clone(), arg.clone());
                    }
                    Some((func.block.clone(), vars))
                }
            };
            if let Some((block, vars)) = block {
                let scope = me.go_down(vars);
                let val = match scope.interpret_program(block, global)? {
                    Return::None => Value::Void,
                    Return::Value(val) => val,
                };
                global.recursion -= 1;
                break Ok(val);
            } else if let Some(parent) = &me.parent {
                me = &parent;
            } else {
                global.recursion = 0;
                return Err(InterpreterError::UndefinedFunction(ident).into());
            }
        }
    }

    pub fn interpret_expr(&self, expr: Expression, global: &mut Global) -> Result<Value, Error> {
        macro_rules! interpret_operation {
            ($left:expr, $right:expr, $op:tt) => {

                self.interpret_expr($left, global)? $op self.interpret_expr($right, global)?
            }
        }

        macro_rules! interpret_bool {
            ($left:expr, $right:expr, $op:tt) => {

                bool::try_from(self.interpret_expr($left, global)?)? $op bool::try_from(self.interpret_expr($right, global)?)?
            }
        }

        match expr {
            Expression::Variable(ident) => match self.get_var(ident.clone()) {
                Some(value) => Ok(value),
                None => Err(InterpreterError::UndefinedVariable(ident).into()),
            },
            Expression::Value(value) => Ok(value),
            Expression::Sum(left, right) => Ok(interpret_operation!(*left, *right, +)?),
            Expression::Sub(left, right) => Ok(interpret_operation!(*left, *right, -)?),
            Expression::Mult(left, right) => Ok(interpret_operation!(*left, *right, *)?),
            Expression::Div(left, right) => Ok(interpret_operation!(*left, *right, /)?),
            Expression::Is(left, right) => Ok(interpret_operation!(*left, *right, ==).into()),
            Expression::IsNot(left, right) => Ok(interpret_operation!(*left, *right, !=).into()),
            Expression::Smlr(left, right) => Ok(interpret_operation!(*left, *right, <).into()),
            Expression::Bigr(left, right) => Ok(interpret_operation!(*left, *right, >).into()),
            Expression::SmlrEq(left, right) => Ok(interpret_operation!(*left, *right, <=).into()),
            Expression::BigrEq(left, right) => Ok(interpret_operation!(*left, *right, >=).into()),
            Expression::And(left, right) => Ok(interpret_bool!(*left, *right, &&).into()),
            Expression::Or(left, right) => Ok(interpret_bool!(*left, *right, ||).into()),
            Expression::FnCall(ident, vars) => self.interpret_fn(
                ident,
                vars.into_iter()
                    .map(|var| -> Value { self.interpret_expr(var, global).unwrap() })
                    .collect(),
                global,
            ),
            Expression::Entrada(input_type) => {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => match input_type {
                        InputType::Number => match input.trim().parse::<f32>() {
                            Ok(number) => Ok(Value::Float(number)),
                            Err(_) => Err(InterpreterError::ParseError(
                                input.trim().to_string(),
                                "float".to_string(),
                            )
                            .into()),
                        },
                        InputType::String => Ok(Value::String(input.trim().to_string())),
                    },
                    Err(_) => Err(InterpreterError::InputError.into()),
                }
            }
        }
    }

    pub fn interpret_ast(&self, ast: AstNode, global: &mut Global) -> Result<Return, Error> {
        match ast {
            AstNode::Print(exprs) => {
                let mut print_string = String::new();
                for expr in exprs.into_iter() {
                    print_string
                        .push_str(format!(" {}", &self.interpret_expr(expr, global)?).as_str());
                }

                println!("{}", print_string.trim());
            }
            AstNode::Val(_) => {}
            AstNode::Definition { ident, expr } => {
                let value = self.interpret_expr(expr, global)?;
                {
                    self.variables.borrow_mut().insert(ident, value);
                }
            }
            AstNode::If { comp, block, senao } => {
                if let Value::Bool(boolean) = self.interpret_expr(comp, global)? {
                    if boolean {
                        return self.interpret_program(block, global);
                    } else if let Some(block) = senao {
                        return self.interpret_program(block, global);
                    }
                }
            }
            AstNode::While { comp, block } => {
                while let Value::Bool(true) = self.interpret_expr(comp.clone(), global)? {
                    self.interpret_program(block.clone(), global)?;
                }
            }
            AstNode::Function { ident, args, block } => {
                self.functions
                    .borrow_mut()
                    .insert(ident, Function::new(args, block));
            }
            AstNode::FnCall { ident, vars } => {
                self.interpret_fn(
                    ident,
                    vars.into_iter()
                        .map(|expr| self.interpret_expr(expr, global).unwrap())
                        .collect(),
                    global,
                )?;
            }
            AstNode::Return(expr) => return Ok(Return::Value(self.interpret_expr(expr, global)?)),
            AstNode::Expression(_) => {}
        }
        Ok(Return::None)
    }
}
