use std::collections::HashMap;

use crate::{AstNode, Mod, Op, Se};

impl AstNode {
    fn get_value(self, variables: &HashMap<String, String>) -> Option<String> {
        match self {
            AstNode::Print(_)
            | AstNode::Definition { .. }
            | AstNode::If { .. }
            | AstNode::IfEnd => None,
            AstNode::Integer(integer) => Some(format!("{}", integer)),
            AstNode::Operation { op, left, right } => {
                let l = left
                    .get_value(&variables)
                    .expect("Can't get value from left")
                    .parse::<i32>()
                    .expect("Failed to parse");
                let r = right
                    .get_value(&variables)
                    .expect("Can't get value from right")
                    .parse::<i32>()
                    .expect("Failed to parse");
                Some(format!(
                    "{}",
                    match op {
                        Op::Sum => l + r,
                        Op::Sub => l - r,
                        Op::Mult => l * r,
                        Op::Div => l / r,
                    }
                ))
            }
            AstNode::Ident(ident) => Some(variables.get(&ident).expect("void").into()),
            AstNode::String(string) => Some(string),
        }
    }
}

pub fn interpret(program: Vec<Box<AstNode>>) {
    let mut variables = HashMap::new();
    let iterator = program.into_iter();

    for step in iterator.enumerate() {
        match *step.1 {
            AstNode::Print(node) => {
                let print_values: Vec<String> = node
                    .into_iter()
                    .map(|inner_node| inner_node.get_value(&variables).expect("void"))
                    .collect();
                println!("{}", print_values.join(" "))
            }
            AstNode::Integer(_) => {}
            AstNode::Operation { .. } => {}
            AstNode::Definition { ident, expr } => {
                variables.insert(ident, expr.get_value(&variables).expect("void"));
            }
            AstNode::Ident(_) => {}
            AstNode::String(_) => {}
            AstNode::If {
                se,
                modifier,
                left,
                right,
                block,
            } => {
                let evaluate = if let Some(modifier) = modifier {
                    match modifier {
                        Mod::Smlr => match se {
                            Se::Is => left.get_value(&variables) < right.get_value(&variables),
                            Se::Isnt => left.get_value(&variables) > right.get_value(&variables),
                        },
                        Mod::Bigr => match se {
                            Se::Is => left.get_value(&variables) > right.get_value(&variables),
                            Se::Isnt => left.get_value(&variables) < right.get_value(&variables),
                        },
                        Mod::SmlrEq => match se {
                            Se::Is => left.get_value(&variables) <= right.get_value(&variables),
                            Se::Isnt => left.get_value(&variables) > right.get_value(&variables),
                        },
                        Mod::BigrEq => match se {
                            Se::Is => left.get_value(&variables) >= right.get_value(&variables),
                            Se::Isnt => left.get_value(&variables) < right.get_value(&variables),
                        },
                    }
                } else {
                    match se {
                        Se::Is => left.get_value(&variables) == right.get_value(&variables),
                        Se::Isnt => left.get_value(&variables) != right.get_value(&variables),
                    }
                };
                if evaluate {
                    interpret(block);
                } else {
                    ()
                };
            }
            AstNode::IfEnd => {}
        }
    }
}
