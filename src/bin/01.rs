use std::collections::BinaryHeap;

use advent_of_code::helpers::{is_char_digit, str_to_u32};

use nom::{
    bytes::complete::take_while1,
    character::complete::line_ending,
    combinator::{map_res, opt},
    multi::{many1},
    sequence::terminated,
    IResult, Parser,
};

pub fn part_one(input: &str) -> Option<u32> {
    let (_, numbers) = parse(input).unwrap();

    numbers
        .into_iter()
        .map(|xs| xs.into_iter().sum())
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, numbers) = parse(input).unwrap();

    let heap: BinaryHeap<u32> = numbers
        .into_iter()
        .map(|xs| xs.into_iter().sum())
        .collect();

    Some(heap.into_iter().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    many1(terminated(
        many1(terminated(
            map_res(take_while1(is_char_digit), str_to_u32),
            opt(line_ending),
        )),
        opt(line_ending),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
