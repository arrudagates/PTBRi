use std::{cell::RefCell, collections::HashMap, io};

use crate::{AstNode, Expression, InputType, Value};

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

    pub fn get_var(&self, ident: String) -> Value {
        let variables = self.variables.borrow();
        variables
            .get(&ident)
            .expect("Variable not defined")
            .to_owned()
    }

    pub fn interpret_program(&self, program: Vec<Box<AstNode>>) -> Return {
        let program = program.into_iter();

        for step in program {
            match self.interpret_ast(*step) {
                Return::None => (),
                Return::Value(value) => return Return::Value(value),
            }
        }
        Return::None
    }

    pub fn interpret_fn(&self, ident: String, variables: Vec<Value>) -> Value {
        let mut me = self;
        loop {
            let block = match me.functions.borrow_mut().get_mut(&ident) {
                None => None,
                Some(func) => {
                    if variables.len() != func.args.len() {
                        panic!("Wrong number of args supplied to function");
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
                let val = match scope.interpret_program(block) {
                    Return::None => Value::Void,
                    Return::Value(val) => val,
                };
                break val;
            } else if let Some(parent) = &me.parent {
                me = &parent;
            } else {
                panic!("Function not defined");
            }
        }
    }

    pub fn interpret_expr(&self, expr: Expression) -> Value {
        macro_rules! interpret_operation {
            ($left:expr, $right:expr, $op:tt) => {
                Value::from(self.interpret_expr($left) $op self.interpret_expr($right))
            }
        }
        match expr {
            Expression::Variable(ident) => self.get_var(ident),
            Expression::Value(value) => value,
            Expression::Sum(left, right) => interpret_operation!(*left, *right, +),
            Expression::Sub(left, right) => interpret_operation!(*left, *right, -),
            Expression::Mult(left, right) => interpret_operation!(*left, *right, *),
            Expression::Div(left, right) => interpret_operation!(*left, *right, /),
            Expression::Is(left, right) => interpret_operation!(*left, *right, ==),
            Expression::IsNot(left, right) => interpret_operation!(*left, *right, !=),
            Expression::Smlr(left, right) => interpret_operation!(*left, *right, <),
            Expression::Bigr(left, right) => interpret_operation!(*left, *right, >),
            Expression::SmlrEq(left, right) => interpret_operation!(*left, *right, <=),
            Expression::BigrEq(left, right) => interpret_operation!(*left, *right, >=),
            Expression::FnCall(ident, vars) => self.interpret_fn(
                ident,
                vars.into_iter()
                    .map(|var| self.interpret_expr(var))
                    .collect(),
            ),
            Expression::Entrada(input_type) => {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                match input_type {
                    InputType::Number => Value::Float(
                        input
                            .trim()
                            .parse::<f32>()
                            .expect("Failed to parse f32 from input"),
                    ),
                    InputType::String => Value::String(input.trim().to_string()),
                }
            }
        }
    }

    pub fn interpret_ast(&self, ast: AstNode) -> Return {
        match ast {
            AstNode::Print(exprs) => {
                let mut print_string = String::new();
                exprs.into_iter().for_each(|expr| {
                    print_string.push_str(format!(" {}", &self.interpret_expr(expr)).as_str())
                });

                println!("{}", print_string.trim());
            }
            AstNode::Val(_) => {}
            AstNode::Definition { ident, expr } => {
                let value = self.interpret_expr(expr);
                {
                    self.variables.borrow_mut().insert(ident, value);
                }
            }
            AstNode::If { comp, block, senao } => {
                if let Value::Bool(boolean) = self.interpret_expr(comp) {
                    if boolean {
                        return self.interpret_program(block);
                    } else if let Some(block) = senao {
                        return self.interpret_program(block);
                    }
                }
            }
            AstNode::While { comp, block } => {
                while let Value::Bool(true) = self.interpret_expr(comp.clone()) {
                    self.interpret_program(block.clone());
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
                        .map(|expr| self.interpret_expr(expr))
                        .collect(),
                );
            }
            AstNode::Return(expr) => return Return::Value(self.interpret_expr(expr)),
            AstNode::Expression(_) => {}
        }
        Return::None
    }
}
