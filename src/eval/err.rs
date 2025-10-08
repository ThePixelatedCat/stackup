use crate::{ast::Val};
use std::{error::Error, fmt::Display};

pub type OpResult<T> = Result<T, OpErr>;

#[derive(Debug, PartialEq)]
pub enum Type {
    Text,
    Number,
    Block,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Text => "Text",
            Self::Number => "Number",
            Self::Block => "Block",
        };
        write!(f, "{name}")
    }
}

impl From<&Val> for Type {
    fn from(value: &Val) -> Self {
        match value {
            Val::Number(_) => Self::Number,
            Val::Text(_) => Self::Text,
            Val::Block(_) => Self::Block,
        }
    }
}

#[derive(Debug)]
pub enum OpErr {
    TypeMismatch { expected: Type, found: Type },
    MissingItems { expected: u32, found: u32 },
    UnknownOp(String),
    InvalidVal { val: Val, desc: String },
}

impl Display for OpErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeMismatch { expected, found } => {
                write!(f, "Expected a {expected}, found a {found}")
            }
            Self::MissingItems { expected, found } => write!(
                f,
                "Expected at least {expected} items on the stack, found {found}"
            ),
            Self::UnknownOp(n) => write!(f, "Operation {n} not bound"),
            Self::InvalidVal { val, desc } => write!(f, "Invalid value {val}, {desc}"),
        }
    }
}

impl Error for OpErr {}
