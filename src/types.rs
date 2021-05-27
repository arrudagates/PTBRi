use core::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Value {
    String(String),
    Integer(i32),
    Float(f32),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum InputType {
    Number,
    String,
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (_value, Value::Bool(_bool)) | (Value::Bool(_bool), _value) => todo!(),
            (value, Value::String(string)) | (Value::String(string), value) => {
                Value::String(format!("{}{}", value, string))
            }
            (Value::Integer(integer), Value::Float(float))
            | (Value::Float(float), Value::Integer(integer)) => {
                Value::Float(integer as f32 + float)
            }
            (Value::Float(floatl), Value::Float(floatr)) => Value::Float(floatl + floatr),
            (Value::Integer(integerl), Value::Integer(integerr)) => {
                Value::Integer(integerl + integerr)
            }
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (_value, Value::Bool(_bool)) | (Value::Bool(_bool), _value) => todo!(),
            (_value, Value::String(_string)) | (Value::String(_string), _value) => todo!(),
            (Value::Integer(integer), Value::Float(float)) => Value::Float(integer as f32 - float),
            (Value::Float(float), Value::Integer(integer)) => Value::Float(float - integer as f32),
            (Value::Float(floatl), Value::Float(floatr)) => Value::Float(floatl - floatr),
            (Value::Integer(integerl), Value::Integer(integerr)) => {
                Value::Integer(integerl - integerr)
            }
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (_value, Value::Bool(_bool)) | (Value::Bool(_bool), _value) => todo!(),
            (_value, Value::String(_string)) | (Value::String(_string), _value) => todo!(),
            (Value::Integer(integer), Value::Float(float))
            | (Value::Float(float), Value::Integer(integer)) => {
                Value::Float(integer as f32 * float)
            }
            (Value::Float(floatl), Value::Float(floatr)) => Value::Float(floatl * floatr),
            (Value::Integer(integerl), Value::Integer(integerr)) => {
                Value::Integer(integerl * integerr)
            }
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (_value, Value::Bool(_bool)) | (Value::Bool(_bool), _value) => todo!(),
            (_value, Value::String(_string)) | (Value::String(_string), _value) => todo!(),
            (Value::Integer(integer), Value::Float(float)) => Value::Float(integer as f32 / float),
            (Value::Float(float), Value::Integer(integer)) => Value::Float(float / integer as f32),
            (Value::Float(floatl), Value::Float(floatr)) => Value::Float(floatl / floatr),
            (Value::Integer(integerl), Value::Integer(integerr)) => {
                Value::Integer(integerl / integerr)
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(boolean) => write!(f, "{}", boolean),
            Value::String(string) => write!(f, "{}", string),
            Value::Integer(integer) => write!(f, "{}", integer),
            Value::Float(float) => write!(f, "{}", float),
        }
    }
}
