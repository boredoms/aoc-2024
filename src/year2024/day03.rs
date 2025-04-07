use once_cell::sync::Lazy;
use regex::Regex;

type Input<'a> = &'a str;

pub fn parse(input: &str) -> Input {
    input
}

pub fn solve_part_one(input: &Input) -> usize {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

    RE.captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
        })
        .sum()
}

pub fn solve_part_two(input: &Input) -> usize {
    input
        .split("do()")
        .map(|s| match s.split_once("don't()") {
            Some((active, _)) => solve_part_one(&active),
            None => solve_part_one(&s),
        })
        .sum()
}

pub fn solve(filename: &str) -> Result<(String, String), String> {
    let input =
        &std::fs::read_to_string(filename).or(Err(format!("could not read file {}", filename)))?;

    let input = parse(input);

    Ok((
        solve_part_one(&input).to_string(),
        solve_part_two(&input).to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_PATH_1: &str = "data/test/year2024/day03-1.txt";
    static TEST_DATA_PATH_2: &str = "data/test/year2024/day03-2.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH_1).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(161, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH_2).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(48, result);
    }
}
