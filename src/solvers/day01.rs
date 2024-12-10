use std::{collections::HashMap, iter::zip};

struct Lists {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Lists {
    fn sort(&mut self) {
        self.left.sort();
        self.right.sort();
    }
}

fn parse(input: &str) -> Lists {
    let (left, right) = input
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
                .unwrap()
        })
        .unzip();

    Lists { left, right }
}

pub fn solve_part_one(input: &str) -> usize {
    // solution goes here
    let mut lists = parse(input);

    lists.sort();

    zip(lists.left, lists.right)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>() as usize
}

pub fn solve_part_two(input: &str) -> usize {
    let lists = parse(input);

    let mut counts: HashMap<u32, u32> = HashMap::new();

    for x in lists.right {
        *counts.entry(x).or_default() += 1;
    }

    lists
        .left
        .into_iter()
        .fold(0, |s, x| s + x * *counts.entry(x).or_default()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day1/test.txt").unwrap());
        assert_eq!(11, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day1/test.txt").unwrap());
        assert_eq!(31, result);
    }
}
