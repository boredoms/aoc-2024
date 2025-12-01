type Input = todo!();

pub fn parse(input: &str) -> Input {}

pub fn solve_part_one(input: &Input) -> usize {
    todo!()
}

pub fn solve_part_two(input: &Input) -> usize {
    todo!()
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

    static TEST_DATA_PATH: &str = "data/test/year20XX/dayXX.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(0, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(0, result);
    }
}
