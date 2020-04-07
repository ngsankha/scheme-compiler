use nom::{IResult, character::streaming::digit1,
    sequence::terminated, character::complete::multispace0,
    combinator::map_res};

use crate::ast::Expr;
use crate::ast::Expr::*;

pub fn parse(input: &str) -> IResult<&str, Expr> {
    terminated(parse_int, multispace0)(input)
}

fn int_from_str(input: &str) -> Result<Expr, std::num::ParseIntError> {
    input.parse::<u64>().map(|n| EInt(n))
}

fn parse_int(input: &str) -> IResult<&str, Expr> {
    map_res(digit1, int_from_str)(input)
}
