use std::sync::Mutex;

type Input = Vec<u32>;

fn hash(mut x: u32) -> u32 {
    x ^= (x << 6) & 0xffffff;
    x ^= x >> 5;
    x ^ (x << 11) & 0xffffff
}

#[inline]
fn to_index(s: &[u32]) -> usize {
    (s[0] + 19 * (s[1] + 19 * (s[2] + 19 * s[3]))) as usize
}

// create the difference vector and the sum
pub fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve_part_one(input: &Input) -> usize {
    let numbers = input;

    numbers
        .iter()
        .map(|n| {
            let mut n = *n;
            for _ in 0..2000 {
                n = hash(n)
            }
            n as usize
        })
        .sum()
}

#[inline]
fn compute_difference(a: u32, b: u32) -> u32 {
    9 - a % 10 + b % 10
}

fn compute_monkey(n: u32, res: &mut [u32]) {
    let mut value = vec![0; 130321];
    let mut seen = vec![false; 130321];

    let a = hash(n);
    let b = hash(a);
    let c = hash(b);

    let mut window = [n, a, b, c];
    let mut difference = [
        0,
        compute_difference(n, a),
        compute_difference(a, b),
        compute_difference(b, c),
    ];

    for _ in 4..2000 {
        let next = hash(window[3]);
        let next_diff = compute_difference(window[3], next);

        window.rotate_right(1);
        difference.rotate_right(1);

        window[3] = next;
        difference[3] = next_diff;

        let index = to_index(&difference);

        if !seen[index] {
            seen[index] = true;
            value[index] = window[3] % 10;
        }
    }

    for (i, e) in value.iter().enumerate() {
        res[i] += e;
    }
}

// serial code is pretty okay, let's try parallel
pub fn solve_part_two(input: &Input) -> usize {
    let numbers = input;

    let mut res = vec![0; 130321];

    let mut res_ = Mutex::new(vec![0; 130321]);

    for i in numbers {
        compute_monkey(*i, &mut res)
    }

    *res.iter().max().unwrap() as usize
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

// the last digits are unaffected by the left shifts!
// therefore, the last few bits change in a more predictable manner -> work out how
// only afected by the bits shifted in from places 6 to 8 // not really helpful...
// payoff is less than 90% of maximal value (2008 * 9)
// avg payoff is < 7
// 9 = 0b0101
// period is usually the whole number state, so it won't help to use that for identifying patterns
// algorithmic approaches (other than brute force): ???
//
// need to check 130321 sequences in total

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_PATH: &str = "data/test/year2024/day22.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(37327623, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!(23, result);
    }
}
