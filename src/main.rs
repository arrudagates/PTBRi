use std::{env::args, fs};

use anyhow::Result;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

mod interpreter;
pub use interpreter::*;
mod types;
pub use types::*;
mod ast;
pub use ast::*;
mod error;
pub use error::*;

#[derive(Parser)]
#[grammar = "ptbr.pest"]
struct PTBRParser;

pub fn main() -> Result<()> {
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

    run(program)
}

pub fn run(program: String) -> Result<()> {
    let mut ast = vec![];

    let pairs = PTBRParser::parse(Rule::program, &program).expect("Failed to parse");
    for pair in pairs {
        match pair.as_rule() {
            Rule::line => {
                ast.push(Box::new(build_ast_from_expr(pair)));
            }
            _ => {}
        }
    }
    Scope::new().interpret_program(ast)?;
    Ok(())
}

fn build_expr(pair: pest::iterators::Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::ident => Expression::Variable(String::from(pair.as_str())),
        Rule::value => build_expr(pair.into_inner().next().unwrap()),
        Rule::integer => Expression::Value(Value::Integer(
            pair.as_str().parse().expect("Failed to parse i32"),
        )),
        Rule::float => Expression::Value(Value::Float(
            pair.as_str().parse().expect("Failed to parse f32"),
        )),
        Rule::string => Expression::Value(Value::String(String::from(
            pair.as_str()
                .strip_prefix("\"")
                .expect("prefix not present")
                .strip_suffix("\"")
                .expect("suffix not present"),
        ))),
        Rule::boolean => match pair.into_inner().next().unwrap().as_rule() {
            Rule::verdadeiro => Expression::Value(Value::Bool(true)),
            Rule::falso => Expression::Value(Value::Bool(false)),
            _ => panic!("Seriously how did this happen?"),
        },
        Rule::expression => build_expr(pair.into_inner().next().unwrap()),
        Rule::sum_expr => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            Expression::Sum(Box::new(build_expr(left)), Box::new(build_expr(right)))
        }

        Rule::subtraction_expr => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            Expression::Sub(Box::new(build_expr(left)), Box::new(build_expr(right)))
        }

        Rule::multiply_expr => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            Expression::Mult(Box::new(build_expr(left)), Box::new(build_expr(right)))
        }

        Rule::divide_expr => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            Expression::Div(Box::new(build_expr(left)), Box::new(build_expr(right)))
        }

        Rule::function_call => {
            let mut pair = pair.into_inner();
            let ident = String::from(pair.next().unwrap().as_str());
            let mut vars: Vec<Expression> = vec![];
            if let Some(signature_pair) = pair.peek() {
                if let Rule::function_signature = signature_pair.as_rule() {
                    for pair in pair.next().unwrap().into_inner() {
                        vars.push(build_expr(pair));
                    }
                }
            }
            Expression::FnCall(ident, vars)
        }

        Rule::entrada => {
            let pair = pair.into_inner().next().unwrap();
            match pair.as_rule() {
                Rule::numero => Expression::Entrada(InputType::Number),
                Rule::texto => Expression::Entrada(InputType::String),
                _ => panic!("Entrada não é texto ou númeor"),
            }
        }

        Rule::comp_expr => {
            let mut pair = pair.into_inner();
            let left = Box::new(build_expr(pair.next().unwrap()));
            let op = pair.next().unwrap();
            let right = Box::new(build_expr(pair.next().unwrap()));
            match op.as_rule() {
                Rule::is_op | Rule::for_op => Expression::Is(left, right),
                Rule::is_not_op | Rule::nao_for_op => Expression::IsNot(left, right),
                Rule::bigger_than | Rule::not_smaller_than_eq => Expression::Bigr(left, right),
                Rule::smaller_than | Rule::not_bigger_than_eq => Expression::Smlr(left, right),
                Rule::bigger_than_eq | Rule::not_smaller_than => Expression::BigrEq(left, right),
                Rule::smaller_than_eq | Rule::not_bigger_than => Expression::SmlrEq(left, right),
                _ => panic!("Only operators accepted for se are 'é' and 'não é'"),
            }
        }

        _ => {
            println!("expr: {:#?}", pair);
            panic!("Not an expression")
        }
    }
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::line => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::se => {
            let mut pair = pair.into_inner();
            let comp_expr = pair.next().unwrap();
            AstNode::If {
                comp: build_expr(comp_expr),
                block: pair
                    .next()
                    .unwrap()
                    .into_inner()
                    .into_iter()
                    .map(|pair| Box::new(build_ast_from_expr(pair)))
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
                                .map(|pair| Box::new(build_ast_from_expr(pair)))
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
            let comp_expr = pair.next().unwrap();
            AstNode::While {
                comp: build_expr(comp_expr),
                block: pair
                    .next()
                    .unwrap()
                    .into_inner()
                    .into_iter()
                    .map(|pair| Box::new(build_ast_from_expr(pair)))
                    .collect(),
            }
        }
        Rule::define_variable => {
            let mut pair = pair.into_inner();
            let ident = String::from(pair.next().unwrap().as_str());
            let expr = build_expr(pair.next().unwrap());
            AstNode::Definition { ident, expr }
        }
        Rule::mostre => {
            let mut vec: Vec<Expression> = Vec::new();
            for pair in pair.clone().into_inner() {
                vec.push(build_expr(pair));
            }
            AstNode::Print(vec)
        }
        Rule::function => {
            let mut pair = pair.into_inner();
            let ident = String::from(pair.next().unwrap().as_str());
            let mut args: Vec<String> = vec![];
            if let Rule::function_signature = pair.peek().unwrap().as_rule() {
                for pair in pair.next().unwrap().into_inner() {
                    args.push(String::from(pair.as_str()));
                }
            }
            let block = pair
                .next()
                .unwrap()
                .into_inner()
                .into_iter()
                .map(|pair| Box::new(build_ast_from_expr(pair)))
                .collect();
            AstNode::Function { ident, args, block }
        }
        Rule::function_call => {
            let mut pair = pair.into_inner();
            let ident = String::from(pair.next().unwrap().as_str());
            let mut vars: Vec<Expression> = vec![];
            if let Some(signature_pair) = pair.peek() {
                if let Rule::function_signature = signature_pair.as_rule() {
                    for pair in pair.next().unwrap().into_inner() {
                        vars.push(build_expr(pair));
                    }
                }
            }
            AstNode::FnCall { ident, vars }
        }
        Rule::retorne => AstNode::Return(build_expr(pair.into_inner().next().unwrap())),
        _ => {
            println!("pair not implemented: {:#?}", pair);
            todo!()
        }
    }
}
