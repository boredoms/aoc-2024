use std::{collections::HashMap, iter::zip};

type Input = (Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Input {
    let (left, right) = input
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
                .unwrap()
        })
        .unzip();

    (left, right)
}

pub fn solve_part_one(input: &Input) -> u32 {
    let (mut left, mut right) = input.clone();

    left.sort();
    right.sort();

    zip(left, right).map(|(a, b)| a.abs_diff(b)).sum()
}

pub fn solve_part_two(input: &Input) -> u32 {
    let (left, right) = input;

    let mut counts: HashMap<u32, u32> = HashMap::new();

    right.iter().for_each(|x| {
        *counts.entry(*x).or_default() += 1;
    });

    left.iter()
        .filter_map(|x| counts.get(&x).map(|n| x * n))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_PATH: &str = "data/test/year2024/day01.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(11, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(31, result);
    }
}
