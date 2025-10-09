use std::ptr;

use crate::ast::Expr;

use crate::dict::Dict;
use crate::err::{OpErr, OpResult};
use crate::stack::Stack;

fn eval_opcall(name: &str, stack: &mut Stack, dict: &mut Dict) -> OpResult<()> {
    let fun_ptr = ptr::from_ref(dict.get(name).ok_or(OpErr::UnknownOp(name.to_owned()))?);
    unsafe { (*fun_ptr)(stack, dict) }
}

pub fn eval_expr(expr: &Expr, stack: &mut Stack, dict: &mut Dict) -> OpResult<()> {
    match expr {
        Expr::Opname(name) => eval_opcall(name, stack, dict)?,
        Expr::Value(v) => stack.push(v.clone()),
    }
    Ok(())
}
