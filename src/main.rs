#![warn(clippy::pedantic)]
#![warn(clippy::style)]
use std::{env, io::Write};
use std::error::Error;
use std::fs;

use stackup::{ExprParser, Program};

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env::args().nth(1);

    match filepath {
        Some(path) => interp_file(path),
        None => repl(),
    }
}

fn interp_file(filepath: String) -> Result<(), Box<dyn Error>> {
    let expr_parser = ExprParser::new();
    let program: Program = fs::read_to_string(filepath)?
        .split_whitespace()
        .map(|e| expr_parser.parse(e).unwrap())
        .collect();

    let (mut s, mut d) = stackup::init_env();
    for expr in &program {
        stackup::eval_expr(expr, &mut s, &mut d)?;
    }

    Ok(())
}

fn repl() -> Result<(), Box<dyn Error>> {
    let (mut s, mut d) = stackup::init_env();
    let mut buf = String::new();
    let expr_parser = ExprParser::new();

    let stdin = std::io::stdin();
    print!("Begin");
    loop {
        buf.clear();
        print!("\n> ");
        std::io::stdout().flush()?;
        if stdin.read_line(&mut buf).is_ok() {
            if buf.trim() == "#quit" {
                break Ok(());
            }
            for expr in buf.split_whitespace() {
                if let Ok(parsed) = expr_parser.parse(expr) {
                    if let Err(e) = stackup::eval_expr(&parsed, &mut s, &mut d) {
                        println!("{e}")
                    }
                }
            }
        }
    }
}
