#![warn(clippy::pedantic)]
#![warn(clippy::style)]
use std::env;
use std::error::Error;
use std::fs;

mod ast;
mod eval;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env::args().nth(1).unwrap();
    let program = fs::read_to_string(filepath)?;
    let (_, ast) = parser::full_parse(&program).unwrap();
    //println!("{ast:?}");
    eval::eval_full(ast)?;
    Ok(())
}
