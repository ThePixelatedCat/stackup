use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

use crate::{builtins::*, err::OpResult, stack::Stack};

type OpFn = dyn Fn(&mut Stack, &mut Dict) -> OpResult<()>;
pub struct Dict(HashMap<String, Box<OpFn>>);

impl Dict {
    pub fn get(&self, name: &str) -> Option<&OpFn> {
        self.0.get(name).map(|v| &**v)
    }

    pub fn bind(&mut self, name: String, fun: Box<OpFn>) {
        self.0.insert(name, fun);
    }
}

impl Default for Dict {
    fn default() -> Self {
        let mut temp = Self(HashMap::new());

        temp.bind("add".to_string(), Box::new(|s, _| arith(s, f64::add)));
        temp.bind("sub".to_string(), Box::new(|s, _| arith(s, f64::sub)));
        temp.bind("mul".to_string(), Box::new(|s, _| arith(s, f64::mul)));
        temp.bind("div".to_string(), Box::new(|s, _| arith(s, f64::div)));

        temp.bind("eq".to_string(), Box::new(|s, _| comp(s, f64::eq)));
        temp.bind("lt".to_string(), Box::new(|s, _| comp(s, f64::lt)));
        temp.bind("gt".to_string(), Box::new(|s, _| comp(s, f64::gt)));
        temp.bind("le".to_string(), Box::new(|s, _| comp(s, f64::le)));
        temp.bind("ge".to_string(), Box::new(|s, _| comp(s, f64::ge)));

        temp.bind("and".to_string(), Box::new(|s, _| log(s, |a, b| a && b)));
        temp.bind("or".to_string(), Box::new(|s, _| log(s, |a, b| a || b)));
        temp.bind("not".to_string(), Box::new(not));

        temp.bind("if".to_string(), Box::new(if_));

        temp.bind("pop".to_string(), Box::new(|s, _| s.pop().map(|_| ())));
        temp.bind("dup".to_string(), Box::new(dup));
        temp.bind("swp".to_string(), Box::new(swp));

        temp.bind("def".to_string(), Box::new(def));
        temp.bind("evl".to_string(), Box::new(evl));

        temp.bind("prt".to_string(), Box::new(prt));
        temp.bind(
            "stk".to_string(),
            Box::new(|s, _| {
                println!("{s}");
                Ok(())
            }),
        );

        temp
    }
}
