use crate::solvers::{day1, day2};

pub fn run(day: usize) {
    let input =
        std::fs::read_to_string(format!("data/day{day}/input.txt")).expect("input file not found");

    let (part1, part2) = match day {
        1 => (day1::solve_part_one(&input), day1::solve_part_two(&input)),
        2 => (day2::solve_part_one(&input), day2::solve_part_two(&input)),
        _ => todo!("not implemented"),
    };

    println!("Day {day} Solutions: {{ Part 1: {part1}, Part 2: {part2} }}");
}
