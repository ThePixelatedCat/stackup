use std::{error::Error, fmt::Display};

use crate::ast::{Ast, Expr, Val};

mod env;

use env::Env;

#[derive(Debug, PartialEq)]
pub enum Type {
    Text,
    Number,
    AnonOp,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Text => "Text",
            Self::Number => "Number",
            Self::AnonOp => "AnonOp",
        };
        write!(f, "{name}")
    }
}

impl From<&Val> for Type {
    fn from(value: &Val) -> Self {
        match value {
            Val::Number(_) => Self::Number,
            Val::Text(_) => Self::Text,
            Val::AnonOp(_) => Self::AnonOp,
        }
    }
}

#[derive(Debug)]
pub enum OpErr {
    TypeMismatch { expected: Type, found: Type },
    MissingVals { expected: u32, found: u32 },
    UnknownOp(String),
    InvalidVal { val: Val, desc: String },
}

impl Display for OpErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeMismatch { expected, found } => {
                write!(f, "Expected a {expected}, found a {found}")
            }
            Self::MissingVals { expected, found } => write!(
                f,
                "Expected at least {expected} items on the stack, found {found}"
            ),
            Self::UnknownOp(n) => write!(f, "Operation {n} not bound"),
            Self::InvalidVal { val, desc } => write!(f, "Invalid value {val}, {desc}"),
        }
    }
}
impl Error for OpErr {}

type OpResult<T> = Result<T, OpErr>;

fn eval_opcall(env: &mut Env, name: &str) -> OpResult<()> {
    if let Some(e) = env.find_op(name)? {
        for expr in e.clone() {
            eval_expr(expr, env)?;
        }
    }
    Ok(())
}

pub fn eval_expr(expr: Expr, env: &mut Env) -> OpResult<()> {
    match expr {
        Expr::Opname(name) => eval_opcall(env, &name)?,
        Expr::Value(v) => env.push(v),
    }

    Ok(())
}

pub fn eval_full(ast: Ast) -> OpResult<()> {
    let mut env = Env::new();

    for expr in ast {
        eval_expr(expr, &mut env)?;
    }
    Ok(())
}
