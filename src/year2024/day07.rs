use std::mem::swap;

#[derive(Debug)]
pub struct Equation {
    result: i64,
    operands: Vec<i64>,
}

type Input = Vec<Equation>;

#[inline]
fn next_power_of_ten(n: i64) -> i64 {
    if n < 10 {
        10
    } else if n < 100 {
        100
    } else {
        1000
    }
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (result, operands) = line.split_once(": ").unwrap();

            Equation {
                result: result.parse::<i64>().unwrap(),
                operands: operands.split(' ').map(|n| n.parse().unwrap()).collect(),
            }
        })
        .collect()
}

fn lower_bound(eq: &Equation) -> i64 {
    let mut i = *eq.operands.first().unwrap();

    for j in eq.operands.iter().skip(1) {
        if *j == 1 || i == 1 {
            continue;
        } else {
            i += j;
        }
    }

    i
}

fn upper_bound(eq: &Equation) -> i64 {
    let mut i = *eq.operands.first().unwrap();

    for j in eq.operands.iter().skip(1) {
        if *j == 1 || i == 1 {
            i += j;
        } else {
            i *= j;
        }
    }

    i
}

fn is_possible(eq: &Equation) -> bool {
    let mut next: Vec<i64> = Vec::new();
    let mut candidates: Vec<i64> = vec![eq.result];

    for (i, e) in eq.operands.iter().rev().enumerate() {
        for c in &candidates {
            if c % *e == 0 {
                next.push(c / *e);
            }

            if c - *e == 0 && i == eq.operands.len() - 1 {
                return true;
            }

            if c - *e > 0 {
                next.push(c - *e);
            }
        }
        candidates.clear();

        swap(&mut candidates, &mut next);
    }

    false
}

fn is_possible_2(eq: &Equation) -> bool {
    let mut next: Vec<i64> = Vec::new();
    let mut candidates: Vec<i64> = vec![eq.operands[0]];

    for i in eq.operands.iter().skip(1) {
        for c in &candidates {
            next.push(c * i);
            next.push(c + i);
            next.push(c * next_power_of_ten(*i) + i);
        }

        candidates.clear();
        swap(&mut candidates, &mut next);
    }

    for i in candidates {
        if i == eq.result {
            return true;
        }
    }
    false
}

pub fn solve_part_one(input: &Input) -> usize {
    let mut res: i64 = 0;

    for eq in input.iter() {
        // these checks get rid of some equations immediately
        if lower_bound(eq) > eq.result {
            continue;
        }

        if upper_bound(eq) < eq.result {
            continue;
        }

        if is_possible(&eq) {
            res += eq.result;
        }
    }
    res as usize
}

pub fn solve_part_two(input: &Input) -> usize {
    let mut res: i64 = 0;

    for eq in input.iter() {
        if is_possible_2(&eq) {
            res += eq.result;
        }
    }

    res as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_PATH: &str = "data/input/year2024/day07.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(3749, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(11387, result);
    }
}
