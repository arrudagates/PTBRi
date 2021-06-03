use std::{cell::RefCell, collections::HashMap, io};

use anyhow::Result;
use thiserror::Error;

use crate::{AstNode, Expression, InputType, Value};

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Variable \"{0}\" not defined")]
    UndefinedVariable(String),
    #[error("Function \"{0}\" not defined")]
    UndefinedFunction(String),
    #[error("Couldn't parse {0} as a {1}")]
    ParseError(String, String),
    #[error("Function {0} expected {1} arguments but {2} {3} supplied")]
    WrongNumberOfArgs(String, usize, usize, String),
}

pub enum Return {
    None,
    Value(Value),
}

pub struct Function {
    pub args: Vec<String>,
    pub block: Vec<Box<AstNode>>,
}

impl Function {
    pub fn new(args: Vec<String>, block: Vec<Box<AstNode>>) -> Self {
        Self { args, block }
    }
}

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
    ) -> Result<Return, InterpreterError> {
        let program = program.into_iter();

        for step in program {
            match self.interpret_ast(*step)? {
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
    ) -> Result<Value, InterpreterError> {
        let mut me = self;
        loop {
            let block = match me.functions.borrow_mut().get_mut(&ident) {
                None => None,
                Some(func) => {
                    if variables.len() != func.args.len() {
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
                        ));
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
                let val = match scope.interpret_program(block)? {
                    Return::None => Value::Void,
                    Return::Value(val) => val,
                };
                break Ok(val);
            } else if let Some(parent) = &me.parent {
                me = &parent;
            } else {
                return Err(InterpreterError::UndefinedFunction(ident));
            }
        }
    }

    pub fn interpret_expr(&self, expr: Expression) -> Result<Value, InterpreterError> {
        macro_rules! interpret_operation {
            ($left:expr, $right:expr, $op:tt) => {
                Value::from(self.interpret_expr($left)? $op self.interpret_expr($right)?)
            }
        }
        match expr {
            Expression::Variable(ident) => match self.get_var(ident.clone()) {
                Some(value) => Ok(value),
                None => Err(InterpreterError::UndefinedVariable(ident)),
            },
            Expression::Value(value) => Ok(value),
            Expression::Sum(left, right) => Ok(interpret_operation!(*left, *right, +)),
            Expression::Sub(left, right) => Ok(interpret_operation!(*left, *right, -)),
            Expression::Mult(left, right) => Ok(interpret_operation!(*left, *right, *)),
            Expression::Div(left, right) => Ok(interpret_operation!(*left, *right, /)),
            Expression::Is(left, right) => Ok(interpret_operation!(*left, *right, ==)),
            Expression::IsNot(left, right) => Ok(interpret_operation!(*left, *right, !=)),
            Expression::Smlr(left, right) => Ok(interpret_operation!(*left, *right, <)),
            Expression::Bigr(left, right) => Ok(interpret_operation!(*left, *right, >)),
            Expression::SmlrEq(left, right) => Ok(interpret_operation!(*left, *right, <=)),
            Expression::BigrEq(left, right) => Ok(interpret_operation!(*left, *right, >=)),
            Expression::FnCall(ident, vars) => self.interpret_fn(
                ident,
                vars.into_iter()
                    .map(|var| -> Value { self.interpret_expr(var).unwrap() })
                    .collect(),
            ),
            Expression::Entrada(input_type) => {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                match input_type {
                    InputType::Number => match input.trim().parse::<f32>() {
                        Ok(number) => Ok(Value::Float(number)),
                        Err(_) => Err(InterpreterError::ParseError(
                            input.trim().to_string(),
                            "float".to_string(),
                        )),
                    },
                    InputType::String => Ok(Value::String(input.trim().to_string())),
                }
            }
        }
    }

    pub fn interpret_ast(&self, ast: AstNode) -> Result<Return, InterpreterError> {
        match ast {
            AstNode::Print(exprs) => {
                let mut print_string = String::new();
                for expr in exprs.into_iter() {
                    print_string.push_str(format!(" {}", &self.interpret_expr(expr)?).as_str());
                }

                println!("{}", print_string.trim());
            }
            AstNode::Val(_) => {}
            AstNode::Definition { ident, expr } => {
                let value = self.interpret_expr(expr)?;
                {
                    self.variables.borrow_mut().insert(ident, value);
                }
            }
            AstNode::If { comp, block, senao } => {
                if let Value::Bool(boolean) = self.interpret_expr(comp)? {
                    if boolean {
                        return self.interpret_program(block);
                    } else if let Some(block) = senao {
                        return self.interpret_program(block);
                    }
                }
            }
            AstNode::While { comp, block } => {
                while let Value::Bool(true) = self.interpret_expr(comp.clone())? {
                    self.interpret_program(block.clone())?;
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
                        .map(|expr| self.interpret_expr(expr).unwrap())
                        .collect(),
                )?;
            }
            AstNode::Return(expr) => return Ok(Return::Value(self.interpret_expr(expr)?)),
            AstNode::Expression(_) => {}
        }
        Ok(Return::None)
    }
}
