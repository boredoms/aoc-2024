use clap::Parser;
use rust_aoc::{solvers::day15, Args};

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    day15::solve_part_two(&std::fs::read_to_string("data/day15/test.txt").unwrap());
}
