use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

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

fn solve_uwu(values: &mut HashMap<String, u8>, ops: &Vec<(String, String, String, String)>) {
    let mut unsolved = ops.clone();
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
        if k.chars().next().unwrap() == 'z' {
            let s: usize = k[1..].parse().unwrap();
            let v = *v as usize;

            res |= v << s;
        }
    });

    res
}

fn get_x(values: &HashMap<String, u8>) -> usize {
    let mut res = 0;

    values.iter().for_each(|(k, v)| {
        if k.chars().next().unwrap() == 'x' {
            let s: usize = k[1..].parse().unwrap();
            let v = *v as usize;

            res |= v << s;
        }
    });

    res
}

fn get_y(values: &HashMap<String, u8>) -> usize {
    let mut res = 0;

    values.iter().for_each(|(k, v)| {
        if k.chars().next().unwrap() == 'y' {
            let s: usize = k[1..].parse().unwrap();
            let v = *v as usize;

            res |= v << s;
        }
    });

    res
}

fn set_values(values: &mut HashMap<String, u8>, mut x: u64, mut y: u64) {
    for i in 0..64 {
        let xs = format!("x{:0>2}", i);
        values.insert(xs, (x >> i & 1) as u8);

        let ys = format!("y{:0>2}", i);
        values.insert(ys, (y >> i & 1) as u8);
    }
}

pub fn solve_part_one(input: &Input) -> usize {
    let mut input = input.clone();

    solve_uwu(&mut input.values, &input.operations);

    let s = score(&input.values);

    println!("{:b}", s);

    s
}

fn find_op(
    op1: String,
    op2: String,
    ops: &Vec<(String, String, String, String)>,
) -> Vec<(String, String)> {
    ops.iter()
        .filter(|(_, x, y, _)| *x == op1 && *y == op2 || *x == op2 && *y == op1)
        .map(|(op, _, _, z)| (op.clone(), z.clone()))
        .collect()
}

fn validate_full_adder(
    ops: &Vec<(String, String, String, String)>,
    carry: &mut String,
    n: u8,
) -> Vec<String> {
    let xn = format!("x{:02}", n);
    let yn = format!("y{:02}", n);
    let zn = format!("z{:02}", n);

    println!("Carry_{n}: {carry}");

    let mut zs = find_op(xn, yn, ops);

    if zs.len() != 2 {
        println!("Problem in zs of half adder {n}");
        println!("{:?}", zs);
    }

    if zs[0].0 == "XOR" {
        zs.swap(0, 1);
    }

    // get the half added wire
    let xy_and = zs[0].1.clone();

    if xy_and.starts_with("z") {
        println!("x AND y of adder {n} routed wrongly: {}", xy_and);
    }

    let xy_xor = zs[1].1.clone();

    if xy_xor.starts_with("z") {
        println!("x XOR y of adder {n} routed wrongly: {}", xy_xor);
    }

    let mut cxy_ops = find_op(xy_xor, carry.clone(), ops);

    if cxy_ops.len() != 2 {
        println!("Problem in cxy_ops of half adder {n}");
        println!("{:?}", cxy_ops);
    }

    if cxy_ops[0].0 == "XOR" {
        cxy_ops.swap(0, 1);
    }

    if cxy_ops[1].1 != zn {
        println!("Output of adder {n} routed wrongly: {}", cxy_ops[1].1);
        println!("Should be {zn}");
        println!("{:?}", cxy_ops);
    }

    let cxy_carry = cxy_ops[0].1.clone();

    let carry_ops = find_op(cxy_carry, xy_and, ops);

    if carry_ops.len() != 1 {
        println!("Problem in carry_ops of half adder {n}");
        println!("{:?}", carry_ops);
    }

    if carry_ops[0].1.starts_with("z") {
        println!("Carry of adder {n} routed wrongly: {}", carry_ops[0].1);
        println!("{:?}", cxy_ops);
    }

    *carry = carry_ops[0].1.clone();

    println!("Adder {n} appears to be valid. (?)");

    Vec::new()
}

fn validate_adder(ops: &Vec<(String, String, String, String)>) -> Vec<String> {
    let xn = format!("x{:02}", 0);
    let yn = format!("y{:02}", 0);

    // ops and wires of the input gates
    let mut zs = find_op(xn, yn, ops);
    let mut carry;

    // swap so zs[0] is the carry
    if zs[0].0 == "XOR" {
        zs.swap(0, 1);
    }

    if zs[1].1 != "z00" {
        println!("Something is wrongo on bit 0");
    }

    carry = zs[0].1.clone();

    println!("0 bit carry is {}", carry);

    for i in 1..44 {
        validate_full_adder(ops, &mut carry, i);
    }

    Vec::new()
}

// need to swap z05 and frn

// wnf and vtj
// wnf should be x16 XOR y16
// vtj should be x16 AND y16

// z21 and gmq

// z39 and wtt

pub fn solve_part_two(input: &Input) -> usize {
    let mut input = input.clone();

    validate_adder(&input.operations);

    //set_values(&mut input.values, 0b100000, 0b10000);

    // let x = get_x(&input.values);
    // let y = get_y(&input.values);

    // let xy = x + y;

    // solve(&mut input.values, &input.operations);

    // x00 should only affect z00 and z01 and so on

    //println!("{:?}", input.values);

    // for i in 0..45 {
    //     let mut s = find_predecessors(&format!("z{:02}", i), &input.operations);
    //     s.sort();

    //     println!("{:?}", s);
    // }
    // let s = score(&input.values);

    // println!("{:b}", xy);
    // println!("{:b}", s);

    let mut wires = vec!["z39", "wtt", "z21", "gmq", "wnf", "vtj", "z05", "frn"];
    wires.sort();

    println!("{}", wires.join(","));

    1
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

        assert_eq!(0, result);
    }
}
