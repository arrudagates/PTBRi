use std::{collections::HashMap, env::args, fs};

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

mod interpreter;
pub use interpreter::*;
mod types;
pub use types::*;

#[derive(Parser)]
#[grammar = "ptbr.pest"]
struct PTBRParser;

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Sum,
    Sub,
    Mult,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Se {
    Is,
    Isnt,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mod {
    Smlr,
    Bigr,
    SmlrEq,
    BigrEq,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    Print(Vec<Box<AstNode>>),
    Val(Value),
    Operation {
        op: Op,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Definition {
        ident: String,
        expr: Box<AstNode>,
    },
    Ident(String),
    If {
        se: Se,
        modifier: Option<Mod>,
        left: Box<AstNode>,
        right: Box<AstNode>,
        block: Vec<Box<AstNode>>,
        senao: Option<Vec<Box<AstNode>>>,
    },
    While {
        se: Se,
        modifier: Option<Mod>,
        left: Box<AstNode>,
        right: Box<AstNode>,
        block: Vec<Box<AstNode>>,
    },
    Function {
        ident: String,
        vars: Vec<String>,
        block: Vec<Box<AstNode>>,
    },
    FunctionCall {
        ident: String,
        vars: Vec<String>,
    },
    FunctionReturn {
        ident: String,
        expr: Box<AstNode>,
    },
    IfEnd,
    Entrada(InputType),
}

pub fn main() {
    let mut ast = vec![];

    let mut program = String::new();

    let args: Vec<String> = args().collect();

    if args.len() > 0 {
        if std::path::Path::new(&args[1]).is_file() {
            program = String::from_utf8_lossy(
                &fs::read(std::path::Path::new(&args[1])).expect("Failed to read file"),
            )
            .to_string();
        }
    }

    let pairs = PTBRParser::parse(Rule::program, &program).expect("Failed to parse");
    for pair in pairs {
        match pair.as_rule() {
            Rule::line => {
                ast.push(Box::new(build_ast_from_expr(pair, None)));
            }
            _ => {}
        }
    }
    let mut variables: HashMap<String, Value> = HashMap::new();
    let mut functions: HashMap<String, (Vec<String>, Vec<Box<AstNode>>, Option<Value>)> =
        HashMap::new();
    //println!("ast: {:#?}", ast);
    interpret(ast, &mut variables, &mut functions);
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>, scope: Option<String>) -> AstNode {
    match pair.as_rule() {
        Rule::line => build_ast_from_expr(pair.into_inner().next().unwrap(), scope.clone()),
        Rule::sum_expr => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            AstNode::Operation {
                op: Op::Sum,
                left: Box::new(build_ast_from_expr(left, None)),
                right: Box::new(build_ast_from_expr(right, None)),
            }
        }
        Rule::subtraction_expr => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            AstNode::Operation {
                op: Op::Sub,
                left: Box::new(build_ast_from_expr(left, None)),
                right: Box::new(build_ast_from_expr(right, None)),
            }
        }
        Rule::multiply_expr => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            AstNode::Operation {
                op: Op::Mult,
                left: Box::new(build_ast_from_expr(left, None)),
                right: Box::new(build_ast_from_expr(right, None)),
            }
        }
        Rule::divide_expr => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            AstNode::Operation {
                op: Op::Div,
                left: Box::new(build_ast_from_expr(left, None)),
                right: Box::new(build_ast_from_expr(right, None)),
            }
        }
        Rule::se => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let mut op_inner = pair.next().unwrap().into_inner();
            let op = op_inner.next().unwrap();
            let modifier = op_inner.next();
            let right = pair.next().unwrap();
            AstNode::If {
                se: match op.as_rule() {
                    Rule::is_op => Se::Is,
                    Rule::is_not_op => Se::Isnt,
                    _ => panic!("Only operators accepted for se are 'é' and 'não é'"),
                },
                modifier: if let Some(modifier) = modifier {
                    match modifier.as_rule() {
                        Rule::bigger_than => Some(Mod::Bigr),
                        Rule::smaller_than => Some(Mod::Smlr),
                        Rule::bigger_than_eq => Some(Mod::BigrEq),
                        Rule::smaller_than_eq => Some(Mod::SmlrEq),
                        _ => panic!("Only modifiers accepted are 'menor que' and 'maior que'"),
                    }
                } else {
                    None
                },
                left: Box::new(build_ast_from_expr(left, None)),
                right: Box::new(build_ast_from_expr(right, None)),
                block: pair
                    .next()
                    .unwrap()
                    .into_inner()
                    .into_iter()
                    .map(|pair| Box::new(build_ast_from_expr(pair, scope.clone())))
                    .collect(),
                senao: {
                    if let Some(senao_block) = pair.next() {
                        Some(
                            senao_block
                                .into_inner()
                                .next()
                                .unwrap()
                                .into_inner()
                                .into_iter()
                                .map(|pair| Box::new(build_ast_from_expr(pair, scope.clone())))
                                .collect(),
                        )
                    } else {
                        None
                    }
                },
            }
        }
        Rule::enquanto => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let mut op_inner = pair.next().unwrap().into_inner();
            let op = op_inner.next().unwrap();
            let modifier = op_inner.next();
            let right = pair.next().unwrap();
            AstNode::While {
                se: match op.as_rule() {
                    Rule::for_op => Se::Is,
                    Rule::nao_for_op => Se::Isnt,
                    _ => panic!("Only operators accepted for enquanto are 'for' and 'não for'"),
                },
                modifier: if let Some(modifier) = modifier {
                    match modifier.as_rule() {
                        Rule::bigger_than => Some(Mod::Bigr),
                        Rule::smaller_than => Some(Mod::Smlr),
                        Rule::bigger_than_eq => Some(Mod::BigrEq),
                        Rule::smaller_than_eq => Some(Mod::SmlrEq),
                        _ => panic!("Only modifiers accepted are 'menor que' and 'maior que'"),
                    }
                } else {
                    None
                },
                left: Box::new(build_ast_from_expr(left, None)),
                right: Box::new(build_ast_from_expr(right, None)),
                block: pair
                    .next()
                    .unwrap()
                    .into_inner()
                    .into_iter()
                    .map(|pair| Box::new(build_ast_from_expr(pair, None)))
                    .collect(),
            }
        }
        Rule::se_end => AstNode::IfEnd,
        Rule::define_variable => {
            let mut pair = pair.into_inner();
            let ident = String::from(pair.next().unwrap().as_str());
            let expr = Box::new(build_ast_from_expr(pair.next().unwrap(), None));
            AstNode::Definition { ident, expr }
        }
        Rule::value => build_ast_from_expr(pair.into_inner().next().unwrap(), None),
        Rule::expression => build_ast_from_expr(pair.into_inner().next().unwrap(), None),
        Rule::integer => AstNode::Val(Value::Integer(
            pair.as_str().parse().expect("Failed to parse i32"),
        )),
        Rule::float => AstNode::Val(Value::Float(
            pair.as_str().parse().expect("Failed to parse f32"),
        )),
        Rule::ident => {
            let ident: String = String::from(pair.as_str());
            AstNode::Ident(ident)
        }
        Rule::boolean => match pair.into_inner().next().unwrap().as_rule() {
            Rule::verdadeiro => AstNode::Val(Value::Bool(true)),
            Rule::falso => AstNode::Val(Value::Bool(false)),
            _ => panic!("Seriously how did this happen?"),
        },
        Rule::string => AstNode::Val(Value::String(String::from(
            pair.as_str()
                .strip_prefix("\"")
                .expect("prefix not present")
                .strip_suffix("\"")
                .expect("suffix not present"),
        ))),
        Rule::mostre => {
            let mut vec: Vec<Box<AstNode>> = Vec::new();
            for pair in pair.clone().into_inner() {
                vec.push(Box::new(build_ast_from_expr(pair, None)));
            }
            AstNode::Print(vec)
        }
        Rule::function => {
            let mut pair = pair.into_inner();
            let ident = String::from(pair.next().unwrap().as_str());
            let mut vars: Vec<String> = vec![];
            if let Rule::function_signature = pair.peek().unwrap().as_rule() {
                for pair in pair.next().unwrap().into_inner() {
                    vars.push(String::from(pair.as_str()));
                }
            }
            let block = pair
                .next()
                .unwrap()
                .into_inner()
                .into_iter()
                .map(|pair| match pair.as_rule() {
                    Rule::retorne => Box::new(AstNode::FunctionReturn {
                        ident: ident.clone(),
                        expr: Box::new(build_ast_from_expr(
                            pair.into_inner().next().unwrap(),
                            scope.clone(),
                        )),
                    }),
                    _ => Box::new(build_ast_from_expr(pair, Some(ident.clone()))),
                })
                .collect();
            AstNode::Function { ident, vars, block }
        }
        Rule::function_call => {
            let mut pair = pair.into_inner();
            let ident = String::from(pair.next().unwrap().as_str());
            let mut vars: Vec<String> = vec![];
            if let Rule::function_signature = pair.peek().unwrap().as_rule() {
                for pair in pair.next().unwrap().into_inner() {
                    vars.push(String::from(pair.as_str()));
                }
            }
            AstNode::FunctionCall { ident, vars }
        }
        Rule::retorne => AstNode::FunctionReturn {
            ident: scope.clone().expect("No scope"),
            expr: Box::new(build_ast_from_expr(
                pair.into_inner().next().unwrap(),
                scope.clone(),
            )),
        },
        Rule::entrada => {
            let pair = pair.into_inner().next().unwrap();
            match pair.as_rule() {
                Rule::numero => AstNode::Entrada(InputType::Number),
                Rule::texto => AstNode::Entrada(InputType::String),
                _ => panic!("Entrada não é texto ou númeor"),
            }
        }
        _ => {
            println!("pair not implemented: {:#?}", pair);
            todo!()
        }
    }
}
