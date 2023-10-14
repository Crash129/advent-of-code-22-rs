use std::ops::{BitAnd};

use bitmaps::Bitmap;
use nom::{
    bytes::complete::take_while1,
    character::{complete::line_ending, is_alphabetic},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    Finish, IResult, Parser,
};

pub fn part_one(input: &str) -> Option<u32> {
    let (_, rucksacks) = parse(input.as_bytes()).finish().unwrap();

    let x: usize = rucksacks
        .into_iter()
        .map(chars_to_priorities)
        .map(split_vec_half)
        .map(|(x, y)| (bitmap_from_u8s(x), bitmap_from_u8s(y)))
        .map(|(x, y)| x.bitand(y))
        .fold(0 as usize, |acc: usize, bmp: Bitmap<128>| {
            acc + bmp.into_iter().map(|x| x + 1).sum::<usize>()
        });

    Some(u32::try_from(x).unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, rucksacks) = parse(input.as_bytes()).finish().unwrap();

    let x: usize = rucksacks
        .into_iter()
        .map(chars_to_priorities)
        .map(bitmap_from_u8s)
        .collect::<Vec<_>>()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .into_iter()
                .cloned()
                .reduce(|x, y| x.bitand(y))
                .unwrap()
                .first_index()
                .unwrap()
        })
        .map(|x| x + 1)
        .sum();

    Some(u32::try_from(x).unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn bitmap_from_u8s(xs: Vec<u8>) -> Bitmap<128> {
    let mut bmp: Bitmap<128> = Bitmap::new();
    for x in xs.into_iter() {
        bmp.set(x.into(), true);
    }

    bmp
}

fn chars_to_priorities(cs: &[u8]) -> Vec<u8> {
    cs.into_iter()
        .map(|c| {
            if c.is_ascii_uppercase() {
                26 + c - ('A' as u8)
            } else {
                c - ('a' as u8)
            }
        })
        .collect()
}

fn split_vec_half<T>(xs: Vec<T>) -> (Vec<T>, Vec<T>)
where
    T: Clone,
{
    assert_eq!(xs.len() % 2, 0);

    (xs[..xs.len() / 2].to_vec(), xs[xs.len() / 2..].to_vec())
}

fn parse(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many1(terminated(take_while1(is_alphabetic), opt(line_ending))).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
