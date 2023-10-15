/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::num::ParseIntError;

use nom::{IResult, combinator::map_res, bytes::complete::{take_while1, take}, Parser};

pub fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(take_while1(is_char_digit), str_to_u32).parse(input)
}

pub fn str_to_u32(input: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(input, 10)
}

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(take_while1(is_char_digit), str_to_usize).parse(input)
}

pub fn str_to_usize(input: &str) -> Result<usize, ParseIntError> {
    usize::from_str_radix(input, 10)
}

pub fn parse_char(input: &str) -> IResult<&str, char> {
    map_res(take(1usize), str_to_char).parse(input)
}

pub fn str_to_char(input: &str) -> Result<char, Box<dyn std::error::Error>> {
    match input.chars().next() {
        Some(x) => Ok(x),
        _ => Err("empty input".into())
    }
}

pub fn is_char_digit(c: char) -> bool {
    c.is_digit(10)
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}