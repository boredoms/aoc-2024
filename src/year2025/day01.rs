const DIAL_START: i32 = 50;

type Input = Vec<i32>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let n = line
                .chars()
                .skip(1)
                .fold(0, |n, c| n * 10 + c.to_digit(10).unwrap()) as i32;
            if line.starts_with('L') {
                -n
            } else {
                n
            }
        })
        .collect()
}

pub fn solve_part_one(input: &Input) -> usize {
    input
        .iter()
        .fold((0, DIAL_START), |(key, sum), n| {
            let pos = (sum + n).rem_euclid(100);
            (key + i32::from(pos == 0), pos)
        })
        .0 as usize
}

pub fn solve_part_two(input: &Input) -> usize {
    input
        .iter()
        .fold((0, DIAL_START), |(key, sum), n| {
            (
                key + ((n + sum - 100 * i32::from(*n < 0 && sum != 0)) / 100).abs(),
                (sum + n).rem_euclid(100),
            )
        })
        .0 as usize
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

    static TEST_DATA_PATH: &str = "data/test/year2025/day01.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(3, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(6, result);
    }
}
