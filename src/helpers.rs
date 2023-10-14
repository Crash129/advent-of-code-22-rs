/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::num::ParseIntError;

use nom::{IResult, combinator::map_res, bytes::complete::take_while1, Parser};

pub fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(take_while1(is_char_digit), str_to_u32).parse(input)
}

pub fn str_to_u32(input: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(input, 10)
}

pub fn is_char_digit(c: char) -> bool {
    c.is_digit(10)
}

