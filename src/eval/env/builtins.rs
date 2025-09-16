use super::super::{OpErr, OpResult};
use super::Env;
use crate::ast::Val;
use crate::eval;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

type V = Box<dyn Fn(&mut Env) -> OpResult<()>>;
pub struct Builtins(HashMap<String, V>);

impl Builtins {
    pub fn new() -> Self {
        let mut nself = Self(HashMap::new());

        nself.put("add", |e| arith(e, f64::add));
        nself.put("sub", |e| arith(e, f64::sub));
        nself.put("mul", |e| arith(e, f64::mul));
        nself.put("div", |e| arith(e, f64::div));

        nself.put("eq", |e| comp(e, f64::eq));
        nself.put("lt", |e| comp(e, f64::lt));
        nself.put("gt", |e| comp(e, f64::gt));
        nself.put("le", |e| comp(e, f64::le));
        nself.put("ge", |e| comp(e, f64::ge));

        nself.put("and", |e| log(e, |a, b| a && b));
        nself.put("or", |e| log(e, |a, b| a || b));
        nself.put("not", not);

        nself.put("if", if_);

        nself.put("pop", |e| e.pop().map(|_| ()));
        nself.put("dup", dup);
        nself.put("swp", swp);

        nself.put("def", def);
        nself.put("evl", evl);

        nself.put("prt", prt);
        nself.put("stk", |e| Ok(println!("{}", e.stack)));

        nself
    }

    pub fn find(&self, k: &str) -> Option<*const V> {
        self.0.get(k).map(|v| &raw const *v)
    }

    fn put<F: Fn(&mut Env) -> OpResult<()> + 'static>(&mut self, name: &str, f: F) {
        self.0.insert(name.to_string(), Box::new(f));
    }
}

fn arith<F: Fn(f64, f64) -> f64>(env: &mut Env, op: F) -> OpResult<()> {
    let n1 = env.pop_num().map_err(|e| {
        if let OpErr::MissingVals { .. } = e {
            OpErr::MissingVals {
                expected: 2,
                found: 0,
            }
        } else {
            e
        }
    })?;
    let n2 = env.pop_num().map_err(|e| {
        if let OpErr::MissingVals { .. } = e {
            OpErr::MissingVals {
                expected: 2,
                found: 1,
            }
        } else {
            e
        }
    })?;
    env.push(Val::Number(op(n2, n1)));
    Ok(())
}

fn comp<F: Fn(&f64, &f64) -> bool>(env: &mut Env, op: F) -> OpResult<()> {
    let n1 = env.pop_num().map_err(|e| {
        if let OpErr::MissingVals { .. } = e {
            OpErr::MissingVals {
                expected: 2,
                found: 0,
            }
        } else {
            e
        }
    })?;
    let n2 = env.pop_num().map_err(|e| {
        if let OpErr::MissingVals { .. } = e {
            OpErr::MissingVals {
                expected: 2,
                found: 1,
            }
        } else {
            e
        }
    })?;
    let result = if op(&n1, &n2) {1.} else {0.};
    env.push(Val::Number(result));
    Ok(())
}

fn log<F: Fn(bool, bool) -> bool>(env: &mut Env, op: F) -> OpResult<()> {
    let n1 = env.pop_num().map(|n| n > 0.).map_err(|e| {
        if let OpErr::MissingVals { .. } = e {
            OpErr::MissingVals {
                expected: 2,
                found: 0,
            }
        } else {
            e
        }
    })?;
    let n2 = env.pop_num().map(|n| n > 0.).map_err(|e| {
        if let OpErr::MissingVals { .. } = e {
            OpErr::MissingVals {
                expected: 2,
                found: 1,
            }
        } else {
            e
        }
    })?;
    let result = if op(n1, n2) {1.} else {0.};
    env.push(Val::Number(result));
    Ok(())
}

fn not(env: &mut Env) -> OpResult<()> {
    let n = env.pop_num().map(|n| n > 0.).map_err(|e| {
        if let OpErr::MissingVals { .. } = e {
            OpErr::MissingVals {
                expected: 2,
                found: 0,
            }
        } else {
            e
        }
    })?;
    let result = if n {0.} else {1.};
    env.push(Val::Number(result));
    Ok(())
}

fn if_(env: &mut Env) -> OpResult<()> {
    let el = env.pop_anonop()?;
    let th = env.pop_anonop()?;
    let cond = env.pop_num().map(|n| n > 0.)?;

    let to_run = if cond {th} else {el};
    for expr in to_run {
        eval::eval_expr(expr, env)?;
    }
    Ok(())
}

fn dup(env: &mut Env) -> OpResult<()> {
    let temp = env.pop()?;
    env.push(temp.clone());
    env.push(temp);
    Ok(())
}

fn swp(env: &mut Env) -> OpResult<()> {
    let temp1 = env.pop().map_err(|_| OpErr::MissingVals {
        expected: 2,
        found: 0,
    })?;
    let temp2 = env.pop().map_err(|_| OpErr::MissingVals {
        expected: 2,
        found: 1,
    })?;
    env.push(temp1);
    env.push(temp2);
    Ok(())
}

fn def(env: &mut Env) -> OpResult<()> {
    let body = env.pop_anonop()?;
    let name = env.pop_text()?;

    if name.contains([' ', '\t', '\n', '\r', '{', '}']) {
        return Err(OpErr::InvalidVal {
            val: Val::Text(name),
            desc: "name contains invalid characters".to_string(),
        });
    }

    env.bind_op(name, body);
    Ok(())
}

fn evl(env: &mut Env) -> OpResult<()> {
    let body = env.pop_anonop()?;
    for expr in body {
        eval::eval_expr(expr, env)?;
    }
    Ok(())
}

fn prt(env: &mut Env) -> OpResult<()> {
    println!("{}", env.pop()?);
    Ok(())
}
