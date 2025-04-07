#[derive(Debug)]
pub struct Input {
    patterns: Vec<String>,
    designs: Vec<String>,
}

pub fn parse(input: &str) -> Input {
    let mut iter = input.lines();

    let patterns = iter
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    // skip empty line
    let _ = iter.next();

    let designs = iter.map(|s| s.to_string()).collect();

    Input { patterns, designs }
}

fn assemble(patterns: &Vec<String>, design: &String) -> usize {
    let mut reachable = vec![0usize; design.len() + 1];
    reachable[0] = 1;

    // we see which positions are reachable
    for i in 0..reachable.len() {
        // if not reachable, continue
        if reachable[i] == 0 {
            continue;
        }

        // for each pattern check if we have a match
        for pattern in patterns {
            let l = pattern.len();

            if i + l > design.len() {
                continue;
            }

            if design[i..i + l] == *pattern {
                reachable[i + l] += reachable[i];
            }
        }
    }

    *reachable.last().unwrap()
}

pub fn solve_part_one(input: &Input) -> usize {
    input
        .designs
        .iter()
        .map(|design| assemble(&input.patterns, design))
        .filter(|i| *i != 0)
        .count()
}

pub fn solve_part_two(input: &Input) -> usize {
    input
        .designs
        .iter()
        .map(|design| assemble(&input.patterns, design))
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

    static TEST_DATA_PATH: &str = "data/test/year2024/day19.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(6, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(16, result);
    }
}
