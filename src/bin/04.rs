use advent_of_code::helpers::{is_char_digit, parse_u32};
use nom::{
    bytes::complete::take_while1,
    character::complete::line_ending,
    combinator::opt,
    multi::many1,
    sequence::{separated_pair, terminated},
    Finish, IResult, Parser,
};

pub fn part_one(input: &str) -> Option<u32> {
    let (_, assignments) = parse(input).finish().unwrap();

    Some(
        u32::try_from(
            assignments
                .into_iter()
                .map(sections_intersect_full)
                .filter(|x| *x)
                .count(),
        )
        .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, assignments) = parse(input).finish().unwrap();

    Some(
        u32::try_from(
            assignments
                .into_iter()
                .map(sections_intersect_partial)
                .filter(|x| *x)
                .count(),
        )
        .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn sections_intersect_full(assignment: ((u32, u32), (u32, u32))) -> bool {
    let (fst, snd) = assignment;

    fst.0 >= snd.0 && fst.1 <= snd.1 || snd.0 >= fst.0 && snd.1 <= fst.1
}

fn sections_intersect_partial(assignment: ((u32, u32), (u32, u32))) -> bool {
    let (fst, snd) = assignment;

    fst.1 >= snd.0 && fst.1 <= snd.1 || snd.1 >= fst.0 && snd.1 <= fst.1
}

fn parse(input: &str) -> IResult<&str, Vec<((u32, u32), (u32, u32))>> {
    many1(terminated(
        separated_pair(
            parse_sections,
            nom::character::complete::char(','),
            parse_sections,
        ),
        opt(line_ending),
    ))
    .parse(input)
}

fn parse_sections(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(parse_u32, nom::character::complete::char('-'), parse_u32).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
