use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    InterpreterError(InterpreterError),
    TypeError(TypeError),
    ParserError(ParserError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InterpreterError(error) => write!(f, "{}", error),
            Error::TypeError(error) => write!(f, "{}", error),
            Error::ParserError(error) => write!(f, "{}", error),
        }
    }
}

impl From<InterpreterError> for Error {
    fn from(error: InterpreterError) -> Self {
        Self::InterpreterError(error)
    }
}

impl From<TypeError> for Error {
    fn from(error: TypeError) -> Self {
        Self::TypeError(error)
    }
}

impl From<ParserError> for Error {
    fn from(error: ParserError) -> Self {
        Self::ParserError(error)
    }
}

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
    #[error("Failed to read input")]
    InputError,
}

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Cannot perform {0}, between types {1} and {2}")]
    IllegalOperation(String, String, String),
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Parsing failed, reason:\n{0}")]
    PestError(String),
    #[error("Not an expression:\n{0}")]
    NotAnExpression(String),
    #[error("Incorrect syntax:\n{0}")]
    NotAST(String),
}
