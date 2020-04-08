use nom::{IResult,
    character::streaming::digit1,
    character::streaming::multispace0,
    character::streaming::multispace1,
    character::streaming::char,
    sequence::terminated,
    sequence::preceded,
    sequence::tuple,
    bytes::streaming::tag,
    branch::alt,
    combinator::map_res};

use crate::ast::Expr;
use crate::ast::Expr::*;

pub fn parse(input: &str) -> IResult<&str, Expr> {
    terminated(parse_expr, nom::character::complete::multispace0)(input)
}

fn to_int(input: &str) -> Result<Expr, std::num::ParseIntError> {
    input.parse::<u64>().map(|n| EInt(n))
}

fn parse_sexpr(input: &str) -> IResult<&str, Expr> {
    preceded(tag("("),
        preceded(multispace0,
            terminated(
                terminated(
                    alt((parse_add1,
                         parse_sub1,
                         parse_zeroh,
                         parse_if)),
                    multispace0),
                tag(")"))))(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_int, parse_bool, parse_sexpr))(input)
}

fn parse_true(input: &str) -> IResult<&str, Expr> {
    map_res(char('t'), to_bool)(input)
}

fn parse_false(input: &str) -> IResult<&str, Expr> {
    map_res(char('f'), to_bool)(input)
}

fn parse_bool(input: &str) -> IResult<&str, Expr> {
    preceded(tag("#"), alt((parse_true, parse_false)))(input)
}

fn to_bool(input: char) -> Result<Expr, std::num::ParseIntError> {
    Ok(match input {
        't' => EBool(true),
        'f' => EBool(false),
        _ => unreachable!()
    })
}

fn to_add1(e: Expr) -> Result<Expr, ()> {
    Ok(EAdd1(Box::new(e)))
}

fn to_sub1(e: Expr) -> Result<Expr, ()> {
    Ok(ESub1(Box::new(e)))
}

fn to_if(e: (Expr, Expr, Expr)) -> Result<Expr, ()> {
    Ok(EIf(Box::new(e.0), Box::new(e.1), Box::new(e.2)))
}

fn parse_add1(input: &str) -> IResult<&str, Expr> {
    map_res(preceded(tag("add1"),
        preceded(multispace0,
            parse_expr)), to_add1)(input)
}

fn parse_sub1(input: &str) -> IResult<&str, Expr> {
    map_res(preceded(tag("sub1"),
        preceded(multispace0,
            parse_expr)), to_sub1)(input)
}

fn parse_zeroh(input: &str) -> IResult<&str, Expr> {
    map_res(preceded(tag("zero?"),
        preceded(multispace0,
            parse_expr)), to_zeroh)(input)
}

fn parse_int(input: &str) -> IResult<&str, Expr> {
    map_res(digit1, to_int)(input)
}

fn to_zeroh(e: Expr) -> Result<Expr, ()> {
    Ok(EZeroh(Box::new(e)))
}

fn parse_if(input: &str) -> IResult<&str, Expr> {
    map_res(preceded(tag("if"),
        preceded(multispace1, tuple((parse_expr,
                preceded(multispace1, parse_expr),
                preceded(multispace1, parse_expr))))), to_if)(input)
}

