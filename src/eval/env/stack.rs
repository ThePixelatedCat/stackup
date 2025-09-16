use crate::{ast::Val, eval::{OpErr, OpResult}};

pub struct Stack (Vec<Val>);

impl Stack {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    
    pub fn push(&mut self, item: Val) {
        self.0.push(item);
    }

    pub fn pop(&mut self) -> OpResult<Val> {
        self.0.pop().ok_or(OpErr::MissingVals { expected: 1, found: 0 })
    }
}