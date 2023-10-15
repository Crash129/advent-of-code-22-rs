use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    match find_unique_sequence(input, 4) {
        Some(i) => Some(u32::try_from(i).unwrap()),
        _ => None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    match find_unique_sequence(input, 14) {
        Some(i) => Some(u32::try_from(i).unwrap()),
        _ => None
    }
}

fn find_unique_sequence(input: &str, seq_len: usize) -> Option<usize> {
    let mut set: HashSet<char> = HashSet::with_capacity(seq_len);
    let input_chars: Vec<char> = input.chars().collect();

    for i in 0..input.len()-seq_len {
        set.clear();
        for j in i..i+seq_len {
            if !set.insert(input_chars[j]) {
                break;
            }
        }

        if set.len() == seq_len {
            return Some(i+seq_len);
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(23));
    }
}
