type Pins = [u8; 5];

#[derive(Debug)]
pub struct Input {
    locks: Vec<Pins>,
    keys: Vec<Pins>,
}

pub fn parse(input: &str) -> Input {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    input.split("\n\n").for_each(|s| {
        let mut u: Pins = [0; 5];
        let mut it = s.lines();

        // skip first line
        it.next();

        for _ in 0..5 {
            let l = it.next().unwrap();

            for (i, j) in l.bytes().enumerate() {
                if j == b'#' {
                    u[i] += 1;
                }
            }
        }

        //  keys
        if s.starts_with(".") {
            keys.push(u);
        } else {
            locks.push(u);
        }
    });

    Input { locks, keys }
}

fn compatible(lock: &Pins, key: &Pins) -> bool {
    for i in 0..5 {
        if lock[i] + key[i] > 5 {
            return false;
        }
    }

    true
}

pub fn solve_part_one(input: &Input) -> usize {
    let mut res = 0;

    for lock in &input.locks {
        for key in &input.keys {
            if compatible(lock, key) {
                res += 1;
            }
        }
    }

    res
}

pub fn solve_part_two(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_PATH: &str = "data/test/year2024/day25.txt";

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

        assert_eq!(0, result);
    }
}
