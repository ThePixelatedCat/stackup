use std::collections::HashMap;
use std::ops::{Add, Sub, Mul, Div};
use super::Env;
use super::super::{OpResult, OpErr};
use crate::ast::Val;

type V = Box<dyn FnMut(&mut Env) -> OpResult<()>>;
pub struct Builtins (HashMap<String, V>);

impl Builtins {
    pub fn new() -> Self {
        let mut nself = Self (HashMap::new());

        nself.put("add", |e| arith(e, f64::add));
        nself.put("sub", |e| arith(e, f64::sub));
        nself.put("mul", |e| arith(e, f64::mul));
        nself.put("div", |e| arith(e, f64::div));

        nself.put("pop", |e| e.pop().map(|_| ()));
        nself.put("dup", dup);
        nself.put("swp", swp);

        nself.put("def", def);
        
        nself
    }

    pub fn find(&self, k: String) -> Option<&V> {
        self.0.get(&k)
    }

    fn put<F: FnMut(&mut Env) -> OpResult<()> + 'static>(&mut self, name: &str, f: F) {
        self.0.insert(name.to_string(), Box::new(f));
    }
}

fn arith<F: Fn(f64, f64) -> f64>(env: &mut Env, op: F) -> OpResult<()> {
    let n1 = env.pop_num().map_err(|e| if let OpErr::MissingVals {..} = e {OpErr::MissingVals { expected: 2, found: 0 }} else {e})?;
    let n2 = env.pop_num().map_err(|e| if let OpErr::MissingVals {..} = e {OpErr::MissingVals { expected: 2, found: 1 }} else {e})?;
    Ok(env.push(Val::Number(op(n1, n2))))
}

fn dup(env: &mut Env) -> OpResult<()> {
    let temp = env.pop()?;
    env.push(temp.clone());
    env.push(temp);
    Ok(())
}

fn swp(env: &mut Env) -> OpResult<()> {
    let temp1 = env.pop().map_err(|_| OpErr::MissingVals { expected: 2, found: 0 })?;
    let temp2 = env.pop().map_err(|_| OpErr::MissingVals { expected: 2, found: 1 })?;
    env.push(temp1);
    env.push(temp2);
    Ok(())
}

fn def(env: &mut Env) -> OpResult<()> {
    let body = env.pop_anonop()?;
    let name = env.pop_text()?;

    if name.contains(&[' ', '\t', '\n', '\r', '{', '}']) {
        return Err(OpErr::InvalidVal { val: Val::Text(name), desc: "name contains invalid characters".to_string() })
    };

    Ok(env.bind(name, body))
}