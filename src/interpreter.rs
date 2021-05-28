use std::{collections::HashMap, io};

use crate::{AstNode, Mod, Op, Se, Value};

impl AstNode {
    fn get_value(
        self,
        mut variables: &mut HashMap<String, Value>,
        mut functions: &mut HashMap<String, (Vec<String>, Vec<Box<AstNode>>, Option<Value>)>,
    ) -> Option<Value> {
        match self {
            AstNode::Print(_)
            | AstNode::Definition { .. }
            | AstNode::If { .. }
            | AstNode::While { .. }
            | AstNode::IfEnd => None,
            AstNode::Operation { op, left, right } => {
                let l = left
                    .get_value(&mut variables, &mut functions)
                    .expect("Can't get value from left");
                let r = right
                    .get_value(&mut variables, &mut functions)
                    .expect("Can't get value from right");
                Some(match op {
                    Op::Sum => l + r,
                    Op::Sub => l - r,
                    Op::Mult => l * r,
                    Op::Div => l / r,
                })
            }
            AstNode::Ident(ident) => Some(
                variables
                    .clone()
                    .get(&ident)
                    .expect("Not defined")
                    .to_owned(),
            ),
            AstNode::Val(value) => Some(value),
            AstNode::Function { .. } => None,
            AstNode::FunctionCall { ident, vars } => {
                let function = functions.get(&ident).expect("Function not found");

                interpret(function.clone().1, &mut variables, &mut functions.clone());
                Some(variables.get(&ident.clone()).expect("void").to_owned())
            }
            AstNode::Entrada(input_type) => {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                match input_type {
                    crate::InputType::Number => Some(Value::Float(
                        input
                            .trim()
                            .parse::<f32>()
                            .expect("Failed to parse f32 from input"),
                    )),
                    crate::InputType::String => Some(Value::String(input.trim().to_string())),
                }
            }
            AstNode::FunctionReturn { .. } => todo!(),
        }
    }
}

macro_rules! compare {
    ($left:expr, $right: expr, $op:path) => {
        $op($left, $right)
    };
}

pub fn interpret(
    program: Vec<Box<AstNode>>,
    mut variables: &mut HashMap<String, Value>,
    mut functions: &mut HashMap<String, (Vec<String>, Vec<Box<AstNode>>, Option<Value>)>,
) {
    let iterator = program.into_iter();

    for step in iterator {
        match *step {
            AstNode::Print(node) => {
                let print_values: Vec<String> = node
                    .into_iter()
                    .map(|inner_node| {
                        format!(
                            "{}",
                            inner_node
                                .get_value(&mut variables, &mut functions)
                                .expect("void")
                        )
                    })
                    .collect();
                println!("{}", print_values.join(" "))
            }
            AstNode::Operation { .. } => {}
            AstNode::Definition { ident, expr } => {
                variables.insert(
                    ident,
                    expr.get_value(&mut variables.clone(), &mut functions)
                        .expect("void"),
                );
            }
            AstNode::Ident(_) => {}
            AstNode::If {
                se,
                modifier,
                left,
                right,
                block,
                senao,
            } => {
                let comp = if let Some(modifier) = modifier {
                    match modifier {
                        Mod::Smlr => match se {
                            Se::Is => std::cmp::PartialOrd::lt,
                            Se::Isnt => std::cmp::PartialOrd::gt,
                        },
                        Mod::Bigr => match se {
                            Se::Is => std::cmp::PartialOrd::gt,
                            Se::Isnt => std::cmp::PartialOrd::lt,
                        },
                        Mod::SmlrEq => match se {
                            Se::Is => std::cmp::PartialOrd::le,
                            Se::Isnt => std::cmp::PartialOrd::gt,
                        },
                        Mod::BigrEq => match se {
                            Se::Is => std::cmp::PartialOrd::ge,
                            Se::Isnt => std::cmp::PartialOrd::lt,
                        },
                    }
                } else {
                    match se {
                        Se::Is => std::cmp::PartialEq::eq,
                        Se::Isnt => std::cmp::PartialEq::ne,
                    }
                };
                if compare!(
                    &left
                        .clone()
                        .get_value(&mut variables, &mut functions)
                        .unwrap(),
                    &right
                        .clone()
                        .get_value(&mut variables, &mut functions)
                        .unwrap(),
                    comp
                ) {
                    interpret(block, &mut variables, &mut functions);
                } else {
                    if let Some(block) = senao {
                        interpret(block, &mut variables, &mut functions)
                    } else {
                        ()
                    }
                };
            }
            AstNode::IfEnd => {}
            AstNode::While {
                se,
                modifier,
                left,
                right,
                block,
            } => {
                let comp = if let Some(modifier) = modifier {
                    match modifier {
                        Mod::Smlr => match se {
                            Se::Is => std::cmp::PartialOrd::lt,
                            Se::Isnt => std::cmp::PartialOrd::gt,
                        },
                        Mod::Bigr => match se {
                            Se::Is => std::cmp::PartialOrd::gt,
                            Se::Isnt => std::cmp::PartialOrd::lt,
                        },
                        Mod::SmlrEq => match se {
                            Se::Is => std::cmp::PartialOrd::le,
                            Se::Isnt => std::cmp::PartialOrd::gt,
                        },
                        Mod::BigrEq => match se {
                            Se::Is => std::cmp::PartialOrd::ge,
                            Se::Isnt => std::cmp::PartialOrd::lt,
                        },
                    }
                } else {
                    match se {
                        Se::Is => std::cmp::PartialEq::eq,
                        Se::Isnt => std::cmp::PartialEq::ne,
                    }
                };

                while compare!(
                    &left
                        .clone()
                        .get_value(&mut variables, &mut functions)
                        .unwrap(),
                    &right
                        .clone()
                        .get_value(&mut variables, &mut functions)
                        .unwrap(),
                    comp
                ) {
                    interpret(block.clone(), &mut variables, &mut functions)
                }
            }
            AstNode::Val(_) => {}
            AstNode::Function { ident, vars, block } => {
                functions.insert(ident, (vars, block, None));
            }
            AstNode::FunctionCall { ident, .. } => {
                let function = functions.get(&ident).expect("Function not found");
                interpret(function.1.clone(), variables, &mut functions);
            }
            AstNode::Entrada(_) => {}
            AstNode::FunctionReturn { ident, expr } => {
                variables.insert(
                    ident.clone(),
                    expr.clone()
                        .get_value(&mut variables.clone(), &mut functions)
                        .expect("void"),
                );
            }
        }
    }
}
