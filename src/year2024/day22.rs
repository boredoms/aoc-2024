use std::collections::{HashMap, HashSet};

type Input = Vec<u32>;

fn hash(x: u32) -> u32 {
    let mask = 0xffffff;

    // add 64 * x to x and mask
    let x = x ^ (x << 6) & mask;

    // add x / 32 to x and mask
    let x = x ^ (x >> 5) & mask;

    // add 2048 * x to x and mask
    let x = x ^ (x << 11) & mask;

    x
}

fn find_period(mut n: u32) -> u64 {
    let mut i = 1;
    let mut s = hash(n);
    while s != n {
        s = hash(s);
        i += 1;
    }

    i
}

fn create_difference_vector(mut n: u32) -> Vec<(u32, i32)> {
    let mut res = Vec::with_capacity(2000);

    for _ in 0..2000 {
        let s = hash(n);

        res.push((s % 10, (s as i32 % 10 - n as i32 % 10) % 10));

        n = s;
    }

    res
}

fn handle_monkey(d: Vec<(u32, i32)>, prices: &mut HashMap<String, u32>) {
    let mut seen = HashSet::new();

    for i in 0..d.len() - 4 {
        let s = format!("{}{}{}{}", d[i + 0].1, d[i + 1].1, d[i + 2].1, d[i + 3].1);

        if !seen.contains(&s) {
            seen.insert(s.clone());
            *prices.entry(s).or_default() += d[i + 3].0;
        }
    }
}

pub fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve_part_one(input: &Input) -> usize {
    let numbers = input;

    numbers.iter().fold(0, |sum, n| {
        let mut n = *n;
        for _ in 0..2000 {
            n = hash(n);
        }
        (n as usize) + sum
    })
}

pub fn solve_part_two(input: &Input) -> usize {
    let numbers = input;

    let mut res = HashMap::new();

    for i in numbers {
        let v = create_difference_vector(*i);

        handle_monkey(v, &mut res);
    }

    println!("{:?}", res["-21-13"]);

    *res.iter().max_by_key(|s| s.1).unwrap().1 as usize
}

// the last digits are unaffected by the left shifts!
// therefore, the last few bits change in a more predictable manner -> work out how
// only afected by the bits shifted in from places 6 to 8 // not really helpful...
// payoff is less than 90% of maximal value (2008 * 9)
// avg payoff is < 7
// 9 = 0b0101
// period is usually the whole number state, so it won't help to use that for identifying patterns
// algorithmic approaches (other than brute force): ???

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
