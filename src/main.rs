#![warn(clippy::pedantic)]
#![warn(clippy::style)]
use std::env;
use std::error::Error;
use std::fs;

mod ast;
mod parser;
mod eval;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env::args().nth(1).unwrap();
    let program = fs::read_to_string(filepath)?;
    let result = parser::full_parse(&program).unwrap();
    print!("{result:?}");
    Ok(())
}
