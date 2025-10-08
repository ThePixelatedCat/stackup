use super::{Dict, OpErr, OpResult, Stack};
use crate::ast::Val;
use crate::eval;
use std::ops::{Add, Div, Mul, Sub};

pub fn insert(dict: &mut super::Dict) {
    dict.bind("add".to_string(), Box::new(|s, _| arith(s, f64::add)));
    dict.bind("sub".to_string(), Box::new(|s, _| arith(s, f64::sub)));
    dict.bind("mul".to_string(), Box::new(|s, _| arith(s, f64::mul)));
    dict.bind("div".to_string(), Box::new(|s, _| arith(s, f64::div)));

    dict.bind("eq".to_string(), Box::new(|s, _| comp(s, f64::eq)));
    dict.bind("lt".to_string(), Box::new(|s, _| comp(s, f64::lt)));
    dict.bind("gt".to_string(), Box::new(|s, _| comp(s, f64::gt)));
    dict.bind("le".to_string(), Box::new(|s, _| comp(s, f64::le)));
    dict.bind("ge".to_string(), Box::new(|s, _| comp(s, f64::ge)));

    dict.bind("and".to_string(), Box::new(|s, _| log(s, |a, b| a && b)));
    dict.bind("or".to_string(), Box::new(|s, _| log(s, |a, b| a || b)));
    dict.bind("not".to_string(), Box::new(not));

    dict.bind("if".to_string(), Box::new(if_));

    dict.bind("pop".to_string(), Box::new(|s, _| s.pop().map(|_| ())));
    dict.bind("dup".to_string(), Box::new(dup));
    dict.bind("swp".to_string(), Box::new(swp));

    dict.bind("def".to_string(), Box::new(def));

    dict.bind("prt".to_string(), Box::new(prt));
    dict.bind(
        "stk".to_string(),
        Box::new(|s, _| {
            println!("{s}");
            Ok(())
        }),
    );
}

fn arith<F: Fn(f64, f64) -> f64>(stack: &mut Stack, op: F) -> OpResult<()> {
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

fn comp<F: Fn(&f64, &f64) -> bool>(stack: &mut Stack, op: F) -> OpResult<()> {
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

fn log<F: Fn(bool, bool) -> bool>(stack: &mut Stack, op: F) -> OpResult<()> {
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

fn not(stack: &mut Stack, _: *mut Dict) -> OpResult<()> {
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

fn if_(stack: &mut Stack, dict: *mut Dict) -> OpResult<()> {
    let el = stack.pop_block()?;
    let th = stack.pop_block()?;
    let cond = stack.pop_num().map(|n| n > 0.)?;

    let to_run = if cond { th } else { el };
    for expr in to_run {
        eval::eval_expr(expr, stack, unsafe { &mut *dict })?;
    }
    Ok(())
}

fn dup(stack: &mut Stack, _: *mut Dict) -> OpResult<()> {
    let temp = stack.pop()?;
    stack.push(temp.clone());
    stack.push(temp);
    Ok(())
}

fn swp(stack: &mut Stack, _: *mut Dict) -> OpResult<()> {
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

fn def(stack: &mut Stack, dict: *mut Dict) -> OpResult<()> {
    let body = stack.pop_block()?;
    let name = stack.pop_text()?;

    if name.contains([' ', '\t', '\n', '\r', '{', '}']) {
        return Err(OpErr::InvalidVal {
            val: Val::Text(name),
            desc: "name contains invalid characters".to_string(),
        });
    }

    unsafe { (*dict).bind(name, Box::new(eval::build_op(body))) };
    Ok(())
}

fn prt(stack: &mut Stack, _: *mut Dict) -> OpResult<()> {
    println!("{}", stack.pop()?);
    Ok(())
}
