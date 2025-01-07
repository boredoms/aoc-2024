use std::collections::HashMap;

#[derive(Debug)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,

    pc: usize,

    jumped: bool,
    wrote: bool,

    program: Vec<u64>,
    output: Vec<u64>,
}

impl Computer {
    fn combo(&self, op: u64) -> u64 {
        match op {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("seven is not valid"),
            _ => panic!("not implemented"),
        }
    }

    fn bxl(&mut self, op: u64) {
        self.b ^= op;
    }

    fn bst(&mut self, op: u64) {
        self.b = self.combo(op) & 0b111;
    }

    fn jnz(&mut self, op: u64) {
        if self.a != 0 {
            self.pc = op as usize;
            self.jumped = true;
        }
    }

    fn bxc(&mut self) {
        self.b ^= self.c;
    }

    fn out(&mut self, op: u64) {
        let op = self.combo(op);
        self.output.push(op & 0b111);
        self.wrote = true;
    }

    fn dv(&self, op: u64) -> u64 {
        self.a / (1 << self.combo(op))
    }
    fn adv(&mut self, op: u64) {
        self.a = self.dv(op);
    }
    fn bdv(&mut self, op: u64) {
        self.b = self.dv(op);
    }
    fn cdv(&mut self, op: u64) {
        self.c = self.dv(op);
    }

    fn dispatch(&mut self, opcode: u64, op: u64) {
        match opcode {
            0 => self.adv(op),
            1 => self.bxl(op),
            2 => self.bst(op),
            3 => self.jnz(op),
            4 => self.bxc(),
            5 => self.out(op),
            6 => self.bdv(op),
            7 => self.cdv(op),
            _ => panic!("unsupported opcode"),
        }
    }

    fn run(&mut self) {
        while self.pc < self.program.len() {
            self.jumped = false;

            self.dispatch(self.program[self.pc], self.program[self.pc + 1]);

            if !self.jumped {
                self.pc += 2;
            }
        }
    }

    fn run_cycle(&mut self) -> u64 {
        let a = self.a;
        self.pc = 0;
        self.jumped = false;

        while self.pc < self.program.len() && self.jumped == false {
            self.dispatch(self.program[self.pc], self.program[self.pc + 1]);
            self.pc += 2;
        }

        self.a = a;
        self.output.pop().unwrap()
    }
}

fn parse(input: &str) -> Computer {
    let mut it = input.lines();

    let a = it
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    let b = it
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    let c = it
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    it.next();

    let program = it
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();

    Computer {
        a,
        b,
        c,
        pc: 0,
        jumped: false,
        wrote: false,
        program,
        output: Vec::new(),
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let mut computer = parse(input);

    computer.run();

    println!(
        "{}",
        Vec::from_iter(computer.output.iter().map(|s| s.to_string())).join(",")
    );

    0
}

// b = a % 8
// b ^= 5 (101)
// c = a / 2^b
// b ^= 6 (110)
// a /= 8
// b ^= c
// output b mod 8
// jump backward until a is zero

// reverse it:
// get the output:
// b's last three bits are now known
// a is some number < 8
// b ^ 6 gives original b ^ 5 ^ c
// a = 1..7
// c = 1..7 / 2 << 1..7 ^ 5

// b is equal to a at the start

// each run just depends on the value of a at the beginning
// or rather each jump

fn tree_search(c: &mut Computer, depth: i64) -> Option<u64> {
    if depth < 0 {
        return Some(c.a);
    }

    println!("called with c.a = {}", c.a);

    let mut min = None;

    c.a = c.a << 3;
    let a = c.a;

    // println!("called with c.a = {}", c.a);

    for i in 0..8 {
        c.a = a ^ i;

        let x = c.run_cycle();

        // println!("set a to {}", c.a);
        // println!("running one cycle with candidate a: {}", x);

        if x == c.program[depth as usize] {
            if let Some(c) = tree_search(c, depth - 1) {
                if min.is_none() || c < min.unwrap() {
                    min = Some(c);
                }
            }
        }
    }

    min
}

pub fn solve_part_two(input: &str) -> usize {
    let mut computer = parse(input);

    computer.a = 0;

    let d = computer.program.len() as i64 - 1;

    tree_search(&mut computer, d as i64).unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day17/input.txt").unwrap());
        assert_eq!(0, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day17/input.txt").unwrap());
        assert_eq!(117440, result);
    }
}
