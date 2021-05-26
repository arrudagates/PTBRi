use std::collections::HashMap;

use crate::{AstNode, Mod, Op, Se, Value};

impl AstNode {
    fn get_value(self, variables: &HashMap<String, Value>) -> Option<Value> {
        match self {
            AstNode::Print(_)
            | AstNode::Definition { .. }
            | AstNode::If { .. }
            | AstNode::While { .. }
            | AstNode::IfEnd => None,
            AstNode::Operation { op, left, right } => {
                let l = left
                    .get_value(&variables)
                    .expect("Can't get value from left");
                let r = right
                    .get_value(&variables)
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
            AstNode::FunctionCall { .. } => None,
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
    mut functions: &mut HashMap<String, (Vec<String>, Vec<Box<AstNode>>)>,
) {
    let iterator = program.into_iter();

    for step in iterator {
        match *step {
            AstNode::Print(node) => {
                let print_values: Vec<String> = node
                    .into_iter()
                    .map(|inner_node| {
                        format!("{}", inner_node.get_value(&variables).expect("void"))
                    })
                    .collect();
                println!("{}", print_values.join(" "))
            }
            AstNode::Operation { .. } => {}
            AstNode::Definition { ident, expr } => {
                variables.insert(ident, expr.get_value(&variables).expect("void"));
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
                    &left.clone().get_value(&variables).unwrap(),
                    &right.clone().get_value(&variables).unwrap(),
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
                    &left.clone().get_value(&variables).unwrap(),
                    &right.clone().get_value(&variables).unwrap(),
                    comp
                ) {
                    interpret(block.clone(), &mut variables, &mut functions)
                }
            }
            AstNode::Val(_) => {}
            AstNode::Function { ident, vars, block } => {
                functions.insert(ident, (vars, block));
            }
            AstNode::FunctionCall { ident, vars } => {
                let function = functions.get(&ident).expect("Function not found");
                let mut scope_variables: HashMap<String, Value> = HashMap::new();
                vars.into_iter().for_each(|ident| {
                    scope_variables
                        .insert(
                            ident.clone(),
                            variables
                                .get(&ident)
                                .expect("Variable not found")
                                .to_owned(),
                        )
                        .expect("Failed to insert variable into scope");
                });

                interpret(function.1.clone(), variables, functions);
            }
        }
    }
}
