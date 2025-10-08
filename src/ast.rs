use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Value(Val),
    Opname(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Number(f64),
    Text(String),
    Block(Vec<Expr>),
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Text(t) => write!(f, "{t}"),
            Self::Block(e) => write!(f, "{e:?}"),
        }
    }
}
