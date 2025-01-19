#[derive(Debug)]
struct Input {
    patterns: Vec<String>,
    designs: Vec<String>,
}

fn parse(input: &str) -> Input {
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

pub fn solve_part_one(input: &str) -> usize {
    let input = parse(input);

    input
        .designs
        .iter()
        .map(|design| assemble(&input.patterns, design))
        .filter(|i| *i != 0)
        .count()
}

pub fn solve_part_two(input: &str) -> usize {
    let input = parse(input);

    input
        .designs
        .iter()
        .map(|design| assemble(&input.patterns, design))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day19/input.txt").unwrap());
        assert_eq!(6, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day19/input.txt").unwrap());
        assert_eq!(16, result);
    }
}
