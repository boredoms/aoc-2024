use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Input {
    values: HashMap<String, u8>,
    operations: Vec<(String, String, String, String)>,
}

pub fn parse(input: &str) -> Input {
    let (initial, ops) = input.split_once("\n\n").unwrap();

    let mut values = HashMap::new();

    initial.lines().for_each(|s| {
        let (id, v) = s.split_once(": ").unwrap();

        values.insert(id.to_string(), v.parse().unwrap());
    });

    let operations = ops
        .lines()
        .map(|line| {
            let op: Vec<_> = line.split(' ').collect();

            (
                op[1].to_string(),
                op[0].to_string(),
                op[2].to_string(),
                op[4].to_string(),
            )
        })
        .collect();

    Input { values, operations }
}

fn run_circuit(values: &mut HashMap<String, u8>, ops: &[(String, String, String, String)]) {
    let mut unsolved = Vec::from(ops);
    let mut i = 0;

    while !unsolved.is_empty() {
        if i == unsolved.len() {
            i = 0;
        }

        let (op, x, y, z) = &unsolved[i];

        if values.contains_key(x) && values.contains_key(y) {
            let vx = values.get(x).unwrap();
            let vy = values.get(y).unwrap();

            let vz = match op.as_str() {
                "AND" => vx & vy,
                "OR" => vx | vy,
                "XOR" => vx ^ vy,
                _ => panic!("operation not recognized"),
            };

            values.insert(z.clone(), vz);

            // pop the condition
            let last_index = unsolved.len() - 1;
            unsolved.swap(i, last_index);
            unsolved.pop();
        } else {
            i += 1;
        }
    }
}

fn score(values: &HashMap<String, u8>) -> usize {
    let mut res = 0;

    values.iter().for_each(|(k, v)| {
        if k.starts_with('z') {
            let s: usize = k[1..].parse().unwrap();
            let v = *v as usize;

            res |= v << s;
        }
    });

    res
}

pub fn solve_part_one(input: &Input) -> usize {
    let mut input = input.clone();

    run_circuit(&mut input.values, &input.operations);

    score(&input.values)
}

// in a full adder,
// the inputs go into an XOR and an AND
// the carry input goes into an AND and and XOR and comes from an OR
// the output of the first XOR must go into an AND and and XOR
// the outputs of an AND go into an OR
// the outputs come from an XOR

pub fn solve_part_two(input: &Input) -> String {
    let mut outputs = HashSet::new();

    input.operations.iter().for_each(|(op, x, y, _)| {
        outputs.insert((x.clone(), op.clone()));
        outputs.insert((y.clone(), op.clone()));
    });

    let mut res: Vec<&str> = Vec::new();

    for (op, x, y, z) in &input.operations {
        match op.as_str() {
            "AND" => {
                if x != "x00" && y != "x00" && !outputs.contains(&(z.to_string(), "OR".to_string()))
                {
                    res.push(z);
                }
            }
            "OR" => {
                if z.starts_with('z') && z != "z45" {
                    res.push(z)
                }

                if outputs.contains(&(z.to_string(), "OR".to_string())) {
                    res.push(z);
                }
            }
            "XOR" => {
                if x.starts_with('x') && y.starts_with('y')
                    || x.starts_with('y') && y.starts_with('x')
                {
                    if x != "x00"
                        && y != "y00"
                        && !outputs.contains(&(z.to_string(), "XOR".to_string()))
                    {
                        res.push(z);
                    }
                } else {
                    if !z.starts_with('z') {
                        res.push(z);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    res.sort();
    res.join(",")
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

    static TEST_DATA_PATH: &str = "data/test/year2024/day23.txt";

    #[test]
    fn test_part_one() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_one(&input);

        assert_eq!(2024, result);
    }

    #[test]
    fn test_part_two() {
        let input = &std::fs::read_to_string(TEST_DATA_PATH).expect("Test data does not exist.");

        let input = parse(input);
        let result = solve_part_two(&input);

        assert_eq!("", result);
    }
}
