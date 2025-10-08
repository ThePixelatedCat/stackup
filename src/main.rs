#![warn(clippy::pedantic)]
#![warn(clippy::style)]
use std::env;
use std::error::Error;
use std::fs;

use lalrpop_util::lalrpop_mod;

mod ast;
mod eval;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar
);

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = env::args().nth(1).unwrap_or("test.su".to_owned());
    let program = fs::read_to_string(filepath)?;
    let ast = grammar::ProgParser::new().parse(&program).unwrap();
    eval::eval_full(ast)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::grammar::{ValueParser, TextParser, NumParser, BlockParser, ExprParser, OpnameParser, ExprsParser};
    use crate::ast::{Expr, Val};

    #[test]
    fn test_text() {
        let text = "hello".to_string();
        assert_eq!(text, TextParser::new().parse(r#""hello""#).unwrap());
        assert_eq!(text, TextParser::new().parse(r#"'hello'"#).unwrap());
        assert!(TextParser::new().parse(r#"'hello""#).is_err());
        assert!(TextParser::new().parse(r#""hello'"#).is_err());
    }

    #[test]
    fn test_num() {
        assert_eq!(0., NumParser::new().parse(r#"0"#).unwrap());
        assert_eq!(-1.65, NumParser::new().parse(r#"-01.650"#).unwrap());
        assert!(NumParser::new().parse(r#"o232"#).is_err());
    }

    #[test]
    fn test_val() {
        assert_eq!(Val::Number(0.), ValueParser::new().parse("0").unwrap());
        assert_eq!(Val::Text("Hello, World!".to_string()), ValueParser::new().parse(r#""Hello, World!""#).unwrap());
    }

    #[test]
    fn test_opname() {
        assert_eq!("fib", OpnameParser::new().parse("fib").unwrap());
        assert_eq!("Weirder_namE", OpnameParser::new().parse("Weirder_namE").unwrap());
        assert!(OpnameParser::new().parse("bad,name32:").is_err());
    }

    #[test]
    fn test_expr() {
        assert_eq!(Expr::Value(Val::Number(0.)), ExprParser::new().parse("0").unwrap());
        assert_eq!(Expr::Value(Val::Text("Hello, World!".to_string())), ExprParser::new().parse(r#""Hello, World!""#).unwrap());
        assert_eq!(Expr::Opname("test_op".to_owned()), ExprParser::new().parse("test_op").unwrap());
    }

    #[test]
    fn test_exprs() {
        let block = vec![Expr::Value(Val::Number(1.0)), Expr::Value(Val::Number(3.0)), Expr::Value(Val::Number(4.0))];
        assert_eq!(block, ExprsParser::new().parse(r#"1 3 4"#).unwrap());
    }

    #[test]
    fn test_block() {
        let block = vec![Expr::Value(Val::Number(1.0)), Expr::Value(Val::Number(3.0)), Expr::Value(Val::Number(4.0))];
        assert_eq!(block, BlockParser::new().parse(r#"{1 3 4}"#).unwrap());
        assert_eq!(block, BlockParser::new().parse(r#"{ 1 3 4 }"#).unwrap());
        assert!(NumParser::new().parse(r#"{}"#).is_err());
    }
}