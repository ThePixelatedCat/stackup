use std::fmt::Display;

use crate::{
    ast::Val,
    eval::{OpErr, OpResult},
};

#[derive(Debug)]
pub struct Stack(Vec<Val>);

impl Stack {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, item: Val) {
        self.0.push(item);
    }

    pub fn pop(&mut self) -> OpResult<Val> {
        self.0.pop().ok_or(OpErr::MissingVals {
            expected: 1,
            found: 0,
        })
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|v| v.to_string() + ", ").collect::<String>()) 
    }
}
