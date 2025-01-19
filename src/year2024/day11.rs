use std::collections::HashMap;

fn parse(input: &str) -> Vec<u64> {
    input.split(' ').map(|c| c.parse().unwrap()).collect()
}

#[inline]
fn digits(mut x: u64) -> u64 {
    let mut digits = 0;
    while x > 0 {
        digits += 1;
        x /= 10;
    }

    digits
}

#[inline]
fn split(x: u64, digits: u64) -> (u64, u64) {
    let half = 10u64.pow((digits / 2) as u32);

    (x / half, x % half)
}

fn blink_memoized(stones: &Vec<u64>, blinks: usize) -> usize {
    // stack for dfs
    let mut stack = Vec::with_capacity(blinks + 1);

    // table containing stone and depth information
    let mut table: HashMap<(u64, usize), u64> = HashMap::new();

    for stone in stones {
        stack.push((*stone, 0));
    }

    loop {
        let (stone, p) = *stack.last().unwrap();

        if p == blinks {
            table.insert((stone, p), 1);
        }

        // if we have already seen this value in another branch, we are done and can short circuit
        if table.contains_key(&(stone, p)) {
            stack.pop();
        } else {
            if stone == 0 {
                if let Some(v) = table.get(&(1, p + 1)) {
                    table.insert((stone, p), *v);
                } else {
                    stack.push((1, p + 1));
                }
            } else {
                let digits = digits(stone);

                if digits % 2 == 0 {
                    let (a, b) = split(stone, digits);

                    let mut both = true;
                    let mut res = 0;

                    if let Some(v) = table.get(&(a, p + 1)) {
                        res += *v;
                    } else {
                        both = false;
                        stack.push((a, p + 1));
                    }

                    if let Some(v) = table.get(&(b, p + 1)) {
                        res += *v;
                    } else {
                        both = false;
                        stack.push((b, p + 1));
                    }

                    if both {
                        table.insert((stone, p), res);
                    }
                } else {
                    if let Some(v) = table.get(&(2024 * stone, p + 1)) {
                        table.insert((stone, p), *v);
                    } else {
                        stack.push((2024 * stone, p + 1));
                    }
                }
            }
        }

        if stack.is_empty() {
            break;
        }
    }

    //println!("{:?}", table);

    let mut res = 0;

    for stone in stones {
        res += table.get(&(*stone, 0)).unwrap();
    }

    res as usize
}

fn blink_faster(stone: u64, blinks: usize) -> usize {
    let res = 0;

    let mut stones = vec![0; blinks + 1];
    let mut positions = Vec::with_capacity(blinks + 1);

    stones[0] = stone;
    positions.push(0);

    let mut res = 0;

    loop {
        let p = positions.pop().unwrap();
        let mut stone = stones[p];

        for i in p..blinks {
            if stone == 0 {
                stone = 1;
            } else {
                let d = digits(stone);

                if d % 2 == 0 {
                    let (a, b) = split(stone, d);

                    stone = a;
                    stones[i + 1] = b;
                    positions.push(i + 1);
                } else {
                    stone *= 2024;
                }
            }
        }

        res += 1;

        if positions.is_empty() {
            break;
        }
    }

    res
}

fn blink(stones: &mut Vec<u64>) {
    let mut temp = Vec::with_capacity(stones.len());

    for stone in stones.iter_mut() {
        if *stone == 0 {
            *stone = 1;
        } else {
            let d = digits(*stone);

            if d % 2 == 0 {
                let (a, b) = split(*stone, d);
                let s = stone.to_string();

                *stone = a;
                temp.push(b);
            } else {
                *stone *= 2024;
            }
        }
    }

    stones.extend_from_slice(&temp);
}

pub fn solve_part_one(input: &str) -> usize {
    let stones = parse(input);

    blink_memoized(&stones, 25)
}

pub fn solve_part_two(input: &str) -> usize {
    let stones = parse(input);

    blink_memoized(&stones, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day11/test.txt").unwrap());
        assert_eq!(55312, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day11/test.txt").unwrap());
        assert_eq!(0, result);
    }
}
