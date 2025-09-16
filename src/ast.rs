use std::fmt::Display;

pub type Ast = Vec<Expr>;

#[derive(Clone)]
#[derive(Debug)]
pub enum Expr {
    Value(Val),
    Opcall(String),
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Val {
    Number(f64),
    Text(String),
    AnonOp(Vec<Expr>),
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {Self::Number(_) => "Number", Self::Text(_) => "Text", Self::AnonOp(_) => "AnonOp"};
        write!(f, "{name}")
    }
}
