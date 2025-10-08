use std::fmt::Display;

use crate::{
    ast::{Expr, Val},
    eval::{OpErr, OpResult, Type},
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
        self.0.pop().ok_or(OpErr::MissingItems {
            expected: 1,
            found: 0,
        })
    }

    pub fn pop_num(&mut self) -> OpResult<f64> {
        let temp = self.pop()?;

        if let Val::Number(n) = temp {
            Ok(n)
        } else {
            Err(OpErr::TypeMismatch {
                expected: Type::Number,
                found: (&temp).into(),
            })
        }
    }

    pub fn pop_text(&mut self) -> OpResult<String> {
        let temp = self.pop()?;

        if let Val::Text(t) = temp {
            Ok(t)
        } else {
            Err(OpErr::TypeMismatch {
                expected: Type::Number,
                found: (&temp).into(),
            })
        }
    }

    pub fn pop_block(&mut self) -> OpResult<Vec<Expr>> {
        let temp = self.pop()?;

        if let Val::Block(b) = temp {
            Ok(b)
        } else {
            Err(OpErr::TypeMismatch {
                expected: Type::Block,
                found: (&temp).into(),
            })
        }
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
