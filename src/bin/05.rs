use advent_of_code::helpers::{parse_char, parse_usize, transpose};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::line_ending,
    combinator::{opt, value},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple, Tuple},
    Finish, IResult, Parser,
};

pub fn part_one(input: &str) -> Option<String> {
    let (_, (crates, moves)) = parse(input).finish().unwrap();

    let mut stacks: Vec<Vec<char>> = transpose(crates);
    for stack in &mut stacks {
        stack.retain(|&c| c != ' ');
        stack.reverse();
    }

    for (cnt, from, to) in moves {
        for _ in 0..cnt {
            let val = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(val);
        }
    }

    let mut result: Vec<char> = Vec::with_capacity(stacks.len());
    for stack in &stacks {
        result.push(*stack.last().unwrap());
    }

    Some(result.into_iter().collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, (crates, moves)) = parse(input).finish().unwrap();

    let mut stacks: Vec<Vec<char>> = transpose(crates);
    for stack in &mut stacks {
        stack.retain(|&c| c != ' ');
        stack.reverse();
    }

    for (cnt, from, to) in moves {
        let len_before = stacks[to - 1].len();
        for _ in 0..cnt {
            let val = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(val);
        }

        stacks[to - 1][len_before..].reverse()
    }

    let mut result: Vec<char> = Vec::with_capacity(stacks.len());
    for stack in &stacks {
        result.push(*stack.last().unwrap());
    }

    Some(result.into_iter().collect::<String>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

type Move = (usize, usize, usize);

fn parse(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Move>)> {
    (
        many1(terminated(
            separated_list1(nom::character::complete::char(' '), parse_crate),
            line_ending,
        )),
        preceded(
            tuple((take_till(|c| c == '\n'), line_ending, line_ending)),
            many1(parse_move),
        ),
    )
        .parse(input)
}

fn parse_crate(input: &str) -> IResult<&str, char> {
    alt((
        delimited(tag("["), parse_char, tag("]")),
        value(' ', tag("   ")),
    ))
    .parse(input)
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    terminated(
        tuple((
            preceded(tag("move "), parse_usize),
            preceded(tag(" from "), parse_usize),
            preceded(tag(" to "), parse_usize),
        )),
        opt(line_ending),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".into()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".into()));
    }
}
