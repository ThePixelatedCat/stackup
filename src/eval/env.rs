use std::collections::HashMap;

use crate::ast::{Expr, Val};
use super::{OpErr, Type, OpResult};
use builtins::Builtins;
use stack::Stack;

mod builtins;
mod stack;

pub struct Env {
    stack: Stack,
    dict: HashMap<String, Vec<Expr>>,
    builtins: Builtins
}

impl Env {
    pub fn new() -> Self {
        Self {stack: Stack::new(), dict: HashMap::new(), builtins: Builtins::new()}
    }

    pub fn bind(&mut self, name: String, body: Vec<Expr>) {
        self.dict.insert(name, body);
    }

    pub fn push(&mut self, item: Val) {
        self.stack.push(item);
    }

    pub fn pop(&mut self) -> OpResult<Val> {
        self.stack.pop()
    }

    pub fn pop_num(&mut self) -> OpResult<f64> {
        let temp = self.stack.pop()?;

        if let Val::Number(n) = temp {
            Ok(n)
        } else {
            Err(OpErr::TypeMismatch { expected: Type::Number, found: (&temp).into() })
        }
    }

    pub fn pop_text(&mut self) -> OpResult<String> {
        let temp = self.stack.pop()?;
        
        if let Val::Text(t) = temp {
            Ok(t)
        } else {
            Err(OpErr::TypeMismatch { expected: Type::Number, found: (&temp).into() })
        }
    }

    pub fn pop_anonop(&mut self) -> OpResult<Vec<Expr>> {
        let temp = self.stack.pop()?;
        
        if let Val::AnonOp(b) = temp {
            Ok(b)
        } else {
            Err(OpErr::TypeMismatch { expected: Type::AnonOp, found: (&temp).into() })
        }
    }
}