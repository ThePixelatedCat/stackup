use crate::ast::{Expr, Val, Ast};
use nom::branch::alt;
use nom::character::complete::{char, multispace0, multispace1, none_of, one_of, satisfy};
use nom::multi::{many0, many1, separated_list1};
use nom::number::complete::double;
use nom::sequence::delimited;
use nom::{AsChar, IResult, Input, Parser};

fn wrap_space<I, F>(
    f: F,
) -> impl Parser<I, Output = <F as Parser<I>>::Output, Error = <F as Parser<I>>::Error>
where
    I: Clone + Input,
    F: Parser<I>,
    <I as Input>::Item: AsChar,
{
    delimited(multispace0, f, multispace0)
}

fn number(input: &str) -> IResult<&str, Val> {
    double.map(Val::Number).parse(input)
}

fn text(input: &str) -> IResult<&str, Val> {
    let delims = "'\"";
    delimited(one_of(delims), many0(none_of(delims)), one_of(delims))
        .map(|r| Val::Text(r.iter().collect()))
        .parse(input)
}

fn anon_op(input: &str) -> IResult<&str, Val> {
    delimited(char('{'), wrap_space(exprs), char('}'))
        .map(Val::AnonOp)
        .parse(input)
}

fn val(input: &str) -> IResult<&str, Expr> {
    alt((text, number, anon_op)).map(Expr::Value).parse(input)
}

fn op_call(input: &str) -> IResult<&str, Expr> {
    many1(satisfy(|c| c.is_alphabetic() || c == '_'))
        .map(|r| Expr::Opcall(r.iter().collect()))
        .parse(input)
}

fn expr(input: &str) -> IResult<&str, Expr> {
    alt((val, op_call)).parse(input)
}

fn exprs(input: &str) -> IResult<&str, Ast> {
    separated_list1(multispace1, expr).parse(input)
}

pub fn full_parse(input: &str) -> IResult<&str, Ast> {
    exprs.parse(input)
}
