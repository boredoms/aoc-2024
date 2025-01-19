use std::mem::swap;

struct Equation {
    result: i64,
    operands: Vec<i64>,
}

fn parse(input: &str) -> Vec<Equation> {
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
            next.push((c.to_string() + &i.to_string()).parse::<i64>().unwrap());
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

pub fn solve_part_one(input: &str) -> usize {
    let input = parse(input);

    let mut res: i64 = 0;

    for eq in input.iter() {
        if is_possible(&eq) {
            res += eq.result;
        }
    }
    res as usize
}

pub fn solve_part_two(input: &str) -> usize {
    let input = parse(input);

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

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day7/input.txt").unwrap());
        assert_eq!(3749, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day7/input.txt").unwrap());
        assert_eq!(11387, result);
    }
}
