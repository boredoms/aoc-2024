use once_cell::sync::Lazy;
use regex::Regex;

pub fn solve_part_one(input: &str) -> usize {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

    RE.captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
        })
        .sum()
}

pub fn solve_part_two(input: &str) -> usize {
    input
        .split("do()")
        .map(|s| match s.split_once("don't()") {
            Some((active, _)) => solve_part_one(active),
            None => solve_part_one(s),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day3/test.txt").unwrap());
        assert_eq!(161, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day3/test2.txt").unwrap());
        assert_eq!(48, result);
    }
}
