use crate::types::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Variable(String),
    Value(Value),
    Sum(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Is(Box<Expression>, Box<Expression>),
    IsNot(Box<Expression>, Box<Expression>),
    Smlr(Box<Expression>, Box<Expression>),
    Bigr(Box<Expression>, Box<Expression>),
    SmlrEq(Box<Expression>, Box<Expression>),
    BigrEq(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    FnCall(String, Vec<Expression>),
    Entrada(InputType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    Print(Vec<Expression>),
    Val(Value),
    Definition {
        ident: String,
        expr: Expression,
    },
    If {
        comp: Expression,
        block: Vec<Box<AstNode>>,
        senao: Option<Vec<Box<AstNode>>>,
    },
    While {
        comp: Expression,
        block: Vec<Box<AstNode>>,
    },
    Function {
        ident: String,
        args: Vec<String>,
        block: Vec<Box<AstNode>>,
    },
    FnCall {
        ident: String,
        vars: Vec<Expression>,
    },
    Return(Expression),
    Expression(Expression),
}
