type Pins = [u8; 5];

#[derive(Debug)]
struct Input {
    locks: Vec<Pins>,
    keys: Vec<Pins>,
}

fn parse(input: &str) -> Input {
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

pub fn solve_part_one(input: &str) -> usize {
    let input = parse(input);
    let mut res = 0;

    for lock in &input.locks {
        for key in &input.keys {
            if compatible(lock, key) {
                res += 1;
            }
        }
    }

    println!("{:?}", input);

    res
}

pub fn solve_part_two(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day25/input.txt").unwrap());
        assert_eq!(0, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two("");
        assert_eq!(0, result);
    }
}
