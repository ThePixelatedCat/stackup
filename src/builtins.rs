use crate::ast::Val;
use crate::eval;
use crate::{
    dict::Dict,
    err::{OpErr, OpResult},
    stack::Stack,
};

pub fn arith<F: Fn(f64, f64) -> f64>(stack: &mut Stack, op: F) -> OpResult<()> {
    let n1 = stack.pop_num().map_err(|e| {
        if let OpErr::MissingItems { .. } = e {
            OpErr::MissingItems {
                expected: 2,
                found: 0,
            }
        } else {
            e
        }
    })?;
    let n2 = stack.pop_num().map_err(|e| {
        if let OpErr::MissingItems { .. } = e {
            OpErr::MissingItems {
                expected: 2,
                found: 1,
            }
        } else {
            e
        }
    })?;
    stack.push(Val::Number(op(n2, n1)));
    Ok(())
}

pub fn comp<F: Fn(&f64, &f64) -> bool>(stack: &mut Stack, op: F) -> OpResult<()> {
    let n1 = stack.pop_num().map_err(|e| {
        if let OpErr::MissingItems { .. } = e {
            OpErr::MissingItems {
                expected: 2,
                found: 0,
            }
        } else {
            e
        }
    })?;
    let n2 = stack.pop_num().map_err(|e| {
        if let OpErr::MissingItems { .. } = e {
            OpErr::MissingItems {
                expected: 2,
                found: 1,
            }
        } else {
            e
        }
    })?;
    let result = if op(&n1, &n2) { 1. } else { 0. };
    stack.push(Val::Number(result));
    Ok(())
}

pub fn log<F: Fn(bool, bool) -> bool>(stack: &mut Stack, op: F) -> OpResult<()> {
    let n1 = stack.pop_num().map(|n| n > 0.).map_err(|e| {
        if let OpErr::MissingItems { .. } = e {
            OpErr::MissingItems {
                expected: 2,
                found: 0,
            }
        } else {
            e
        }
    })?;
    let n2 = stack.pop_num().map(|n| n > 0.).map_err(|e| {
        if let OpErr::MissingItems { .. } = e {
            OpErr::MissingItems {
                expected: 2,
                found: 1,
            }
        } else {
            e
        }
    })?;
    let result = if op(n1, n2) { 1. } else { 0. };
    stack.push(Val::Number(result));
    Ok(())
}

pub fn not(stack: &mut Stack, _: &mut Dict) -> OpResult<()> {
    let n = stack.pop_num().map(|n| n > 0.).map_err(|e| {
        if let OpErr::MissingItems { .. } = e {
            OpErr::MissingItems {
                expected: 2,
                found: 0,
            }
        } else {
            e
        }
    })?;
    let result = if n { 0. } else { 1. };
    stack.push(Val::Number(result));
    Ok(())
}

pub fn if_(stack: &mut Stack, dict: &mut Dict) -> OpResult<()> {
    let el = stack.pop_block()?;
    let th = stack.pop_block()?;
    let cond = stack.pop_num().map(|n| n > 0.)?;

    let to_run = if cond { th } else { el };
    for expr in to_run {
        eval::eval_expr(&expr, stack, dict)?;
    }
    Ok(())
}

pub fn dup(stack: &mut Stack, _: &mut Dict) -> OpResult<()> {
    let temp = stack.pop()?;
    stack.push(temp.clone());
    stack.push(temp);
    Ok(())
}

pub fn swp(stack: &mut Stack, _: &mut Dict) -> OpResult<()> {
    let temp1 = stack.pop().map_err(|_| OpErr::MissingItems {
        expected: 2,
        found: 0,
    })?;
    let temp2 = stack.pop().map_err(|_| OpErr::MissingItems {
        expected: 2,
        found: 1,
    })?;
    stack.push(temp1);
    stack.push(temp2);
    Ok(())
}

pub fn def(stack: &mut Stack, dict: &mut Dict) -> OpResult<()> {
    let body = stack.pop_block()?;
    let name = stack.pop_text()?;

    if name.contains([' ', '\t', '\n', '\r', '{', '}']) {
        return Err(OpErr::InvalidVal {
            val: Val::Text(name),
            desc: "name contains invalid characters".to_string(),
        });
    }

    let op = Box::new(move |s: &mut Stack, d: &mut Dict| {
        for expr in &body {
            eval::eval_expr(expr, s, d)?;
        }
        Ok(())
    });

    dict.bind(name, op);
    Ok(())
}

pub fn prt(stack: &mut Stack, _: &mut Dict) -> OpResult<()> {
    println!("{}", stack.pop()?);
    Ok(())
}
