use std::fmt::Display;

pub type Ast = Vec<Expr>;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Value(Val),
    Opname(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Number(f64),
    Text(String),
    AnonOp(Vec<Expr>),
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Text(t) => write!(f, "{t}"),
            Self::AnonOp(e) => write!(f, "{e:?}"),
        }
    }
}
