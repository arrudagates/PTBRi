use anyhow::Result;

use crate::{AstNode, Error, Expression, InputType, ParserError, Scope, Value};

use pest::Parser;

#[derive(Parser)]
#[grammar = "ptbr.pest"]
struct PTBRParser;

pub fn run(program: String) -> Result<()> {
    let mut ast = vec![];

    for pair in match PTBRParser::parse(Rule::program, &program) {
        Ok(pairs) => Ok(pairs),
        Err(error) => Err(Error::from(ParserError::PestError(format!("{}", error)))),
    }? {
        match pair.as_rule() {
            Rule::line => {
                ast.push(Box::new(build_ast_from_expr(pair)?));
            }
            _ => {}
        }
    }
    Scope::new().interpret_program(ast)?;
    Ok(())
}

fn build_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expression, Error> {
    match pair.as_rule() {
        Rule::ident => Ok(Expression::Variable(String::from(pair.as_str()))),
        Rule::integer => Ok(Expression::Value(Value::Integer(
            pair.as_str().parse().expect("Failed to parse i32"),
        ))),
        Rule::float => Ok(Expression::Value(Value::Float(
            pair.as_str().parse().expect("Failed to parse f32"),
        ))),
        Rule::string => Ok(Expression::Value(Value::String(String::from(
            pair.as_str()
                .strip_prefix("\"")
                .expect("prefix not present")
                .strip_suffix("\"")
                .expect("suffix not present"),
        )))),

        Rule::verdadeiro => Ok(Expression::Value(Value::Bool(true))),

        Rule::falso => Ok(Expression::Value(Value::Bool(false))),

        Rule::sum_expr => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            Ok(Expression::Sum(
                Box::new(build_expr(left)?),
                Box::new(build_expr(right)?),
            ))
        }

        Rule::subtraction_expr => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            Ok(Expression::Sub(
                Box::new(build_expr(left)?),
                Box::new(build_expr(right)?),
            ))
        }

        Rule::multiply_expr => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            Ok(Expression::Mult(
                Box::new(build_expr(left)?),
                Box::new(build_expr(right)?),
            ))
        }

        Rule::divide_expr => {
            let mut pair = pair.into_inner();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            Ok(Expression::Div(
                Box::new(build_expr(left)?),
                Box::new(build_expr(right)?),
            ))
        }

        Rule::function_call => {
            let mut pair = pair.into_inner();
            let ident = String::from(pair.next().unwrap().as_str());
            let mut vars: Vec<Expression> = vec![];
            if let Some(signature_pair) = pair.peek() {
                if let Rule::function_signature = signature_pair.as_rule() {
                    for pair in pair.next().unwrap().into_inner() {
                        vars.push(build_expr(pair)?);
                    }
                }
            }
            Ok(Expression::FnCall(ident, vars))
        }

        Rule::entrada_numero => Ok(Expression::Entrada(InputType::Number)),

        Rule::entrada_texto => Ok(Expression::Entrada(InputType::String)),

        Rule::comp_expr => {
            let mut pair = pair.into_inner();
            let left = Box::new(build_expr(pair.next().unwrap())?);
            let op = pair.next().unwrap();
            let right = Box::new(build_expr(pair.next().unwrap())?);
            match op.as_rule() {
                Rule::is_op | Rule::for_op => Ok(Expression::Is(left, right)),
                Rule::is_not_op | Rule::nao_for_op => Ok(Expression::IsNot(left, right)),
                Rule::bigger_than | Rule::not_smaller_than_eq => Ok(Expression::Bigr(left, right)),
                Rule::smaller_than | Rule::not_bigger_than_eq => Ok(Expression::Smlr(left, right)),
                Rule::bigger_than_eq | Rule::not_smaller_than => {
                    Ok(Expression::BigrEq(left, right))
                }
                Rule::smaller_than_eq | Rule::not_bigger_than => {
                    Ok(Expression::SmlrEq(left, right))
                }
                _ => panic!("Only operators accepted for se are 'é' and 'não é'"),
            }
        }

        _ => {
            //println!("expr: {:#?}", pair);
            Err(ParserError::NotAnExpression(pair.as_str().to_string()).into())
        }
    }
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Result<AstNode, Error> {
    match pair.as_rule() {
        Rule::line => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::se => {
            let mut pair = pair.into_inner();
            let comp_expr = pair.next().unwrap();
            Ok(AstNode::If {
                comp: build_expr(comp_expr)?,
                block: pair
                    .next()
                    .unwrap()
                    .into_inner()
                    .into_iter()
                    .map(|pair| Box::new(build_ast_from_expr(pair).unwrap()))
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
                                .map(|pair| Box::new(build_ast_from_expr(pair).unwrap()))
                                .collect(),
                        )
                    } else {
                        None
                    }
                },
            })
        }
        Rule::enquanto => {
            let mut pair = pair.into_inner();
            let comp_expr = pair.next().unwrap();
            Ok(AstNode::While {
                comp: build_expr(comp_expr)?,
                block: pair
                    .next()
                    .unwrap()
                    .into_inner()
                    .into_iter()
                    .map(|pair| Box::new(build_ast_from_expr(pair).unwrap()))
                    .collect(),
            })
        }
        Rule::define_variable => {
            let mut pair = pair.into_inner();
            let ident = String::from(pair.next().unwrap().as_str());
            let expr = build_expr(pair.next().unwrap())?;
            Ok(AstNode::Definition { ident, expr })
        }
        Rule::mostre => {
            let mut vec: Vec<Expression> = Vec::new();
            for pair in pair.clone().into_inner() {
                vec.push(build_expr(pair)?);
            }
            Ok(AstNode::Print(vec))
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
                .map(|pair| Box::new(build_ast_from_expr(pair).unwrap()))
                .collect();
            Ok(AstNode::Function { ident, args, block })
        }
        Rule::function_call => {
            let mut pair = pair.into_inner();
            let ident = String::from(pair.next().unwrap().as_str());
            let mut vars: Vec<Expression> = vec![];
            if let Some(signature_pair) = pair.peek() {
                if let Rule::function_signature = signature_pair.as_rule() {
                    for pair in pair.next().unwrap().into_inner() {
                        vars.push(build_expr(pair)?);
                    }
                }
            }
            Ok(AstNode::FnCall { ident, vars })
        }
        Rule::retorne => Ok(AstNode::Return(build_expr(
            pair.into_inner().next().unwrap(),
        )?)),

        _ => {
            //println!("pair not implemented: {:#?}", pair);
            Err(ParserError::NotAST(pair.as_str().to_string()).into())
        }
    }
}
