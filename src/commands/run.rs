use crate::solvers::{day1, day2, day3, day4};

pub fn run(day: usize) {
    let input =
        std::fs::read_to_string(format!("data/day{day}/input.txt")).expect("input file not found");

    let (part1, part2) = match day {
        1 => (day1::solve_part_one(&input), day1::solve_part_two(&input)),
        2 => (day2::solve_part_one(&input), day2::solve_part_two(&input)),
        3 => (day3::solve_part_one(&input), day3::solve_part_two(&input)),
        4 => (day4::solve_part_one(&input), day4::solve_part_two(&input)),
        _ => todo!("not implemented"),
    };

    println!("Day {day} Solutions: {{ Part 1: {part1}, Part 2: {part2} }}");
}
