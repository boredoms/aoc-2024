use std::{
    num,
    sync::{Arc, Mutex},
};

type Input = Vec<u32>;

#[inline]
fn hash(mut x: u32) -> u32 {
    x ^= (x << 6) & 0xffffff;
    x ^= x >> 5;
    x ^ (x << 11) & 0xffffff
}

#[inline]
fn to_index(x: u32, y: u32, z: u32, w: u32) -> usize {
    (x + 19 * (y + 19 * (z + 19 * w))) as usize
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

// serial code is pretty okay, let's try parallel
pub fn solve_part_two<'a>(input: &'a Input) -> usize {
    //let mut res = vec![0; 130321];
    let num_threads = 8;

    let chunks = input.chunks(input.len() / num_threads + 1);

    let res = Arc::new(Mutex::new(vec![0; 130321]));

    std::thread::scope(|s| {
        for (i, chunk) in chunks.enumerate() {
            let res = Arc::clone(&res);

            s.spawn(move || {
                let mut value = vec![0; 130321];
                let mut seen = vec![0u16; 130321];

                for (i, n) in chunk.iter().enumerate() {
                    let a = hash(*n);
                    let b = hash(a);
                    let c = hash(b);

                    let mut x = compute_difference(*n, a);
                    let mut y = compute_difference(a, b);
                    let mut z = compute_difference(b, c);

                    let mut prev = c;

                    for _ in 4..2000 {
                        let next = hash(prev);
                        let next_diff = compute_difference(prev, next);

                        let index = to_index(x, y, z, next_diff);

                        (x, y, z) = (y, z, next_diff);

                        if seen[index] < (i + 1) as u16 {
                            seen[index] = (i + 1) as u16;
                            value[index] += next % 10;
                        }

                        prev = next;
                    }
                }
                let mut v = res.lock().unwrap();

                for (i, e) in value.iter().enumerate() {
                    v[i] += e;
                }
            });
        }
    });

    let v = res.lock().unwrap();

    *v.iter().max().unwrap() as usize
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
