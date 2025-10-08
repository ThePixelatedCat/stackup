#![warn(clippy::pedantic)]
#![warn(clippy::style)]
use std::env;
use std::error::Error;
use std::fs;

use stackup::ExprParser;
use stackup::eval_full;

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env::args().nth(1).expect("missing source file path");
    let expr_parser = ExprParser::new();
    let program = fs::read_to_string(filepath)?
        .split_whitespace()
        .map(|e| expr_parser.parse(e).unwrap())
        .collect();
    eval_full(program)?;
    Ok(())
}


