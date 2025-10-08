use std::{collections::HashMap,rc::Rc};

use crate::ast::{Expr};

mod builtins;
mod err;
mod stack;

use err::{OpErr, OpResult, Type};
use stack::Stack;

type OpFn = dyn Fn(&mut Stack, *mut Dict) -> OpResult<()>;
#[derive(Clone)]
struct Dict(HashMap<String, Rc<OpFn>>);

impl Dict {
    fn get(&self, name: &str) -> Option<&OpFn> {
        self.0.get(name).map(|v| &**v)
    }

    fn bind(&mut self, name: String, fun: Box<OpFn>) {
        self.0.insert(name, Rc::new(fun));
    }
}

fn build_op(body: Vec<Expr>) -> impl Fn(&mut Stack, *mut Dict) -> OpResult<()> {
    move |s, d| {
        for expr in &body {
            eval_expr(expr.clone(), s, unsafe { &mut *d })?;
        }
        Ok(())
    }
}

fn eval_opcall(name: &str, stack: &mut Stack, dict: &mut Dict) -> OpResult<()> {
    let dict_ptr = &raw mut *dict;
    let fun = dict.get(name).ok_or(OpErr::UnknownOp(name.to_owned()))?;
    fun(stack, dict_ptr)
}

fn eval_expr(expr: Expr, stack: &mut Stack, dict: &mut Dict) -> OpResult<()> {
    match expr {
        Expr::Opname(name) => eval_opcall(&name, stack, dict)?,
        Expr::Value(v) => stack.push(v),
    }
    Ok(())
}

pub fn eval_full(ast: Vec<Expr>) -> OpResult<()> {
    let mut stack = Stack::new();
    let mut dict: Dict = Dict(HashMap::new());
    builtins::insert(&mut dict);

    for expr in ast {
        eval_expr(expr, &mut stack, &mut dict)?;
    }
    Ok(())
}
