use nom::{IResult,
    character::streaming::digit1,
    character::streaming::multispace0,
    character::streaming::multispace1,
    character::is_digit,
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

fn int_from_str(input: &str) -> Result<Expr, std::num::ParseIntError> {
    input.parse::<u64>().map(|n| EInt(n))
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    if is_digit(input.as_bytes()[0]) {
        parse_int(input)
    } else {
        preceded(tag("("),
            preceded(multispace0,
                terminated(
                    terminated(
                        alt((parse_add1,
                             parse_sub1,
                             parse_if)),
                        multispace0),
                    tag(")"))))(input)
    }
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

fn parse_int(input: &str) -> IResult<&str, Expr> {
    map_res(digit1, int_from_str)(input)
}

fn to_zeroh(e: Expr) -> Result<Expr, ()> {
    Ok(e)
}

fn parse_zeroh(input: &str) -> IResult<&str, Expr> {
    map_res(preceded(tag("zero?"),
        preceded(multispace0,
            parse_expr)), to_zeroh)(input)
}

fn parse_if(input: &str) -> IResult<&str, Expr> {
    map_res(preceded(tag("if"),
        preceded(multispace1,
                tuple((preceded(tag("("),
                        preceded(multispace0,
                            terminated(
                                terminated(parse_zeroh,
                                multispace0),
                            tag(")")))),
                        preceded(multispace1, parse_expr),
                        preceded(multispace1, parse_expr))))), to_if)(input)
}

