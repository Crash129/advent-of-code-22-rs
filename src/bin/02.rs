use nom::{
    character::complete::{alpha1, line_ending, space1},
    combinator::{map_res, opt},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult, Parser, Finish,
};

pub fn part_one(input: &str) -> Option<u32> {
    let (_, strat) = parse(input).finish().unwrap();

    Some(strat.into_iter().map(round_score).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, strat) = parse(input).finish().unwrap();

    Some(strat.into_iter().map(mutate_round).map(round_score).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

macro_rules! winning_shape {
    (Rock) => { Shape::Paper };
    (Paper) => { Shape::Scissors };
    (Scissors) => { Shape::Rock };
    ($shape:ident) => { compile_error!("Invalid shape") };
}

macro_rules! losing_shape {
    (Rock) => { Shape::Scissors };
    (Paper) => { Shape::Rock };
    (Scissors) => { Shape::Paper };
    ($shape:ident) => { compile_error!("Invalid shape") };
}

fn winning_shape(shape: Shape) -> Shape {
    match shape {
        Shape::Rock => winning_shape!(Rock),
        Shape::Paper => winning_shape!(Paper),
        Shape::Scissors => winning_shape!(Scissors),
    }
}

fn losing_shape(shape: Shape) -> Shape {
    match shape {
        Shape::Rock => losing_shape!(Rock),
        Shape::Paper => losing_shape!(Paper),
        Shape::Scissors => losing_shape!(Scissors),
    }
}

fn mutate_round(round: (Shape, Shape)) -> (Shape, Shape) {
    let (elf, you) = round;

    match you {
        Shape::Rock => (elf, losing_shape(elf)), // lose
        Shape::Paper => (elf, elf), // draw
        Shape::Scissors => (elf, winning_shape(elf)), // win
    }
}

fn round_score(round: (Shape, Shape)) -> u32 {
    hand_score(round.1) + outcome_score(round)
}

fn hand_score(selected_shape: Shape) -> u32 {
    match selected_shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn outcome_score(round: (Shape, Shape)) -> u32 {
    let (elf, you) = round;

    if you == elf {
        3 // draw
    } else if you == winning_shape(elf)
    {
        6 // you won
    } else {
        0 // elf won
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(Shape, Shape)>> {
    many1(terminated(
        separated_pair(parse_hand, space1, parse_hand),
        opt(line_ending),
    ))
    .parse(input)
}

fn str_to_hand(input: &str) -> Result<Shape, Box<dyn std::error::Error>> {
    match input {
        "A" | "X" => Ok(Shape::Rock),
        "B" | "Y" => Ok(Shape::Paper),
        "C" | "Z" => Ok(Shape::Scissors),
        _ => Err("unknown hand".into()),
    }
}

fn parse_hand(input: &str) -> IResult<&str, Shape> {
    map_res(alpha1, str_to_hand).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
