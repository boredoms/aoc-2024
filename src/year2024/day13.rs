use regex::Regex;

use crate::util::point::Point;

#[derive(Debug)]
pub struct Equation {
    a: Point,
    b: Point,
    z: Point,
}

fn parse_point(desc: &str, re: &Regex) -> Point {
    let caps = re.captures(desc).expect("Invalid point description");

    Point::new(caps[1].parse().unwrap(), caps[2].parse().unwrap())
}

type Input = Vec<Equation>;

pub fn parse(input: &str) -> Input {
    let mut it = input.lines();
    let mut res = Vec::new();

    let re = Regex::new(r".*X[+=](\d+), Y[+=](\d+)").unwrap();

    while let (Some(v1), Some(v2), Some(v3), _) = (it.next(), it.next(), it.next(), it.next()) {
        let (a, b, z) = (
            parse_point(v1, &re),
            parse_point(v2, &re),
            parse_point(v3, &re),
        );

        res.push(Equation { a, b, z });
    }

    res
}

fn solve_equation(eq: &Equation) -> Option<(i64, i64)> {
    let l = eq.z.y * eq.a.x - eq.z.x * eq.a.y;
    let r = eq.a.x * eq.b.y - eq.b.x * eq.a.y;

    if r == 0 {
        return None;
    }

    let y = l / r;
    let x = (eq.z.x - eq.b.x * y) / eq.a.x;

    if x < 0 || y < 0 {
        return None;
    }

    if eq.z.x == eq.a.x * x + eq.b.x * y && eq.z.y == eq.a.y * x + eq.b.y * y {
        Some((x, y))
    } else {
        None
    }
}

pub fn solve_part_one(input: &Input) -> usize {
    input
        .iter()
        .filter_map(|eq| solve_equation(eq))
        .map(|(x, y)| 3 * x + y)
        .sum::<i64>() as usize
}

pub fn solve_part_two(input: &Input) -> usize {
    let adjustment = Point::new(10000000000000, 10000000000000);

    input
        .iter()
        .filter_map(|eq| {
            let new_eq = Equation {
                z: eq.z + adjustment,
                ..*eq
            };
            solve_equation(&new_eq)
        })
        .map(|(x, y)| 3 * x + y)
        .sum::<i64>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_PATH: &str = "data/test/year2024/day13.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(480, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(875318608908, result);
    }
}
