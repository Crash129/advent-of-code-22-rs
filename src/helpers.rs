/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::num::ParseIntError;

pub fn str_to_u32(input: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(input, 10)
}

pub fn is_char_digit(c: char) -> bool {
    c.is_digit(10)
}

