use regex::Regex;

pub fn solve_part_one(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
        })
        .sum()

    // solution goes here
}

pub fn solve_part_two(input: &str) -> usize {
    // solution goes here
    0
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
        let result = solve_part_two(&std::fs::read_to_string("data/day3/test.txt").unwrap());
        assert_eq!(0, result);
    }
}
