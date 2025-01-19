use std::{cmp, collections::HashMap, mem::swap};

fn keypad_coordinate(c: char) -> (i8, i8) {
    match c {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!("character not on keypad"),
    }
}

fn arrowkeys_coordinate(c: char) -> (i8, i8) {
    match c {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!("character not on arrow keys"),
    }
}

fn calculate_paths() -> HashMap<(char, char), Vec<String>> {
    let mut paths = HashMap::new();

    let numpad_chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'];
    let numpad_gap = (0, 3);

    let arrow_chars = vec!['<', '>', '^', 'v', 'A'];
    let arrow_gap = (0, 0);

    for a in &numpad_chars {
        for b in &numpad_chars {
            let (ax, ay) = keypad_coordinate(*a);
            let (bx, by) = keypad_coordinate(*b);

            let xs = if bx - ax < 0 {
                "<".repeat((ax - bx) as usize)
            } else {
                ">".repeat((bx - ax) as usize)
            };

            let ys = if by - ay < 0 {
                "^".repeat((ay - by) as usize)
            } else {
                "v".repeat((by - ay) as usize)
            };

            if (ax, by) == numpad_gap {
                paths.insert((*a, *b), vec![xs + &ys + "A"]);
            } else if (bx, ay) == numpad_gap {
                paths.insert((*a, *b), vec![ys + &xs + "A"]);
            } else {
                if xs.is_empty() {
                    paths.insert((*a, *b), vec![ys + "A"]);
                } else if ys.is_empty() {
                    paths.insert((*a, *b), vec![xs + "A"]);
                } else {
                    paths.insert((*a, *b), vec![xs.clone() + &ys + "A", ys + &xs + "A"]);
                }
            }
        }
    }

    for a in &arrow_chars {
        for b in &arrow_chars {
            let (ax, ay) = arrowkeys_coordinate(*a);
            let (bx, by) = arrowkeys_coordinate(*b);

            let xs = if bx - ax < 0 {
                "<".repeat((ax - bx) as usize)
            } else {
                ">".repeat((bx - ax) as usize)
            };

            let ys = if by - ay < 0 {
                "^".repeat((ay - by) as usize)
            } else {
                "v".repeat((by - ay) as usize)
            };

            if (ax, by) == arrow_gap {
                paths.insert((*a, *b), vec![xs + &ys + "A"]);
            } else if (bx, ay) == arrow_gap {
                paths.insert((*a, *b), vec![ys + &xs + "A"]);
            } else {
                if xs.is_empty() {
                    paths.insert((*a, *b), vec![ys + "A"]);
                } else if ys.is_empty() {
                    paths.insert((*a, *b), vec![xs + "A"]);
                } else {
                    paths.insert((*a, *b), vec![xs.clone() + &ys + "A", ys + &xs + "A"]);
                }
            }
        }
    }

    paths
}

fn get_costs(
    paths: &HashMap<(char, char), Vec<String>>,
    mut costs: &mut Vec<HashMap<(char, char), usize>>,
    seq: &str,
    depth: usize,
) -> usize {
    if depth == 0 {
        return seq.len();
    }

    let mut prev = 'A';

    let mut total = 0;

    for c in seq.chars() {
        if let Some(n) = costs[depth].get(&(prev, c)) {
            total += n;
        } else {
            //println!("going from {prev} to {c}");

            let v = paths.get(&(prev, c)).unwrap();

            //println!("paths: {:?}", v);

            let min = v
                .iter()
                .map(|s| get_costs(paths, &mut costs, s, depth - 1))
                .min()
                .unwrap_or(0);
            costs[depth].insert((prev, c), min);

            //println!("cost of {min}");

            total += min;
        }

        prev = c;
    }

    total
}

fn calculate_costs(seq: &str, paths: &HashMap<(char, char), Vec<String>>, depth: usize) -> usize {
    let mut prev = 'A';

    let mut costs = vec![HashMap::new(); depth + 1];

    let mut total = 0;

    for c in seq.chars() {
        if let Some(n) = costs[depth].get(&(prev, c)) {
            total += n;
        } else {
            let v = paths.get(&(prev, c)).unwrap();

            let min = v
                .iter()
                .map(|s| get_costs(paths, &mut costs, s, depth - 1))
                .min()
                .unwrap_or(0);
            costs[depth].insert((prev, c), min);
            total += min;
        }

        prev = c;
    }

    // println!("{:?}", costs);

    total
}

pub fn solve_part_one(input: &str) -> usize {
    let paths = calculate_paths();

    println!("{:?}", paths);

    input
        .lines()
        .map(|line| {
            let num = line.split_once("A").unwrap().0.parse::<usize>().unwrap();

            //            let line = "0";

            let l = calculate_costs(line, &paths, 3);
            println!("num: {num}, len: {}", l);

            num * l
        })
        .sum()
}

pub fn solve_part_two(input: &str) -> usize {
    let paths = calculate_paths();

    println!("{:?}", paths);

    input
        .lines()
        .map(|line| {
            let num = line.split_once("A").unwrap().0.parse::<usize>().unwrap();

            //            let line = "0";

            let l = calculate_costs(line, &paths, 26);
            println!("num: {num}, len: {}", l);

            num * l
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day21/test.txt").unwrap());
        assert_eq!(0, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day21/input.txt").unwrap());
        assert_eq!(0, result);
    }
}

// we don't actually compute a shortest path atm, it only works for small n
