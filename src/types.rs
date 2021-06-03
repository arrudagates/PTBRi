use core::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

use anyhow::Result;

use crate::{Error, TypeError};

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Value {
    Void,
    String(String),
    Integer(i32),
    Float(f32),
    Bool(bool),
    //List(Vec<Value>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum InputType {
    Number,
    String,
}

impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Self::Bool(boolean)
    }
}

impl Add for Value {
    type Output = Result<Self, Error>;

    fn add(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            // (Value::List(_), _) | (_, Value::List(_)) => todo!(),
            (Value::Void, Value::Void) => Ok(Value::Void),
            (Value::Void, other) | (other, Value::Void) => Ok(other),
            (_value, Value::Bool(_bool)) | (Value::Bool(_bool), _value) => Err(
                TypeError::IllegalOperation("addition".into(), "any".into(), "bool".into()).into(),
            ),
            (value, Value::String(string)) | (Value::String(string), value) => {
                Ok(Value::String(format!("{}{}", value, string)))
            }
            (Value::Integer(integer), Value::Float(float))
            | (Value::Float(float), Value::Integer(integer)) => {
                Ok(Value::Float(integer as f32 + float))
            }
            (Value::Float(floatl), Value::Float(floatr)) => Ok(Value::Float(floatl + floatr)),
            (Value::Integer(integerl), Value::Integer(integerr)) => {
                Ok(Value::Integer(integerl + integerr))
            }
        }
    }
}

impl Sub for Value {
    type Output = Result<Self, Error>;

    fn sub(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            // (Value::List(_), _) | (_, Value::List(_)) => todo!(),
            (Value::Void, Value::Void) => Ok(Value::Void),
            (Value::Void, other) | (other, Value::Void) => Ok(other),
            (_value, Value::Bool(_bool)) | (Value::Bool(_bool), _value) => Err(
                TypeError::IllegalOperation("subtraction".into(), "any".into(), "bool".into())
                    .into(),
            ),
            (_value, Value::String(_string)) | (Value::String(_string), _value) => Err(
                TypeError::IllegalOperation("subtraction".into(), "any".into(), "string".into())
                    .into(),
            ),
            (Value::Integer(integer), Value::Float(float)) => {
                Ok(Value::Float(integer as f32 - float))
            }
            (Value::Float(float), Value::Integer(integer)) => {
                Ok(Value::Float(float - integer as f32))
            }
            (Value::Float(floatl), Value::Float(floatr)) => Ok(Value::Float(floatl - floatr)),
            (Value::Integer(integerl), Value::Integer(integerr)) => {
                Ok(Value::Integer(integerl - integerr))
            }
        }
    }
}

impl Mul for Value {
    type Output = Result<Self, Error>;

    fn mul(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            // (Value::List(_), _) | (_, Value::List(_)) => todo!(),
            (Value::Void, Value::Void) => Ok(Value::Void),
            (Value::Void, other) | (other, Value::Void) => Ok(other),
            (_value, Value::Bool(_bool)) | (Value::Bool(_bool), _value) => Err(
                TypeError::IllegalOperation("multiplication".into(), "any".into(), "bool".into())
                    .into(),
            ),
            (_value, Value::String(_string)) | (Value::String(_string), _value) => Err(
                TypeError::IllegalOperation("multiplication".into(), "any".into(), "string".into())
                    .into(),
            ),
            (Value::Integer(integer), Value::Float(float))
            | (Value::Float(float), Value::Integer(integer)) => {
                Ok(Value::Float(integer as f32 * float))
            }
            (Value::Float(floatl), Value::Float(floatr)) => Ok(Value::Float(floatl * floatr)),
            (Value::Integer(integerl), Value::Integer(integerr)) => {
                Ok(Value::Integer(integerl * integerr))
            }
        }
    }
}

impl Div for Value {
    type Output = Result<Self, Error>;

    fn div(self, other: Self) -> Result<Self, Error> {
        match (self, other) {
            // (Value::List(_), _) | (_, Value::List(_)) => todo!(),
            (Value::Void, Value::Void) => Ok(Value::Void),
            (Value::Void, other) | (other, Value::Void) => Ok(other),
            (_value, Value::Bool(_bool)) | (Value::Bool(_bool), _value) => Err(
                TypeError::IllegalOperation("division".into(), "any".into(), "bool".into()).into(),
            ),
            (_value, Value::String(_string)) | (Value::String(_string), _value) => Err(
                TypeError::IllegalOperation("division".into(), "any".into(), "string".into())
                    .into(),
            ),
            (Value::Integer(integer), Value::Float(float)) => {
                Ok(Value::Float(integer as f32 / float))
            }
            (Value::Float(float), Value::Integer(integer)) => {
                Ok(Value::Float(float / integer as f32))
            }
            (Value::Float(floatl), Value::Float(floatr)) => Ok(Value::Float(floatl / floatr)),
            (Value::Integer(integerl), Value::Integer(integerr)) => {
                Ok(Value::Integer(integerl / integerr))
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            // Value::List(list) => {
            //     let mut result = String::new();
            //     for value in list {
            //         result.push_str(format!("{}, ", value).as_str());
            //     }
            //     write!(f, "{}", &result.as_str()[0..result.len() - 2])
            // }
            Value::Void => write!(f, ""),
            Value::Bool(boolean) => write!(f, "{}", boolean),
            Value::String(string) => write!(f, "{}", string),
            Value::Integer(integer) => write!(f, "{}", integer),
            Value::Float(float) => write!(f, "{}", float),
        }
    }
}
