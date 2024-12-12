use crate::solvers::{day01, day02, day11, day3, day4, day5, day6};

pub fn run(day: usize) {
    let input =
        std::fs::read_to_string(format!("data/day{day}/input.txt")).expect("input file not found");

    let (part1, part2) = match day {
        1 => (day01::solve_part_one(&input), day01::solve_part_two(&input)),
        2 => (day02::solve_part_one(&input), day02::solve_part_two(&input)),
        3 => (day3::solve_part_one(&input), day3::solve_part_two(&input)),
        4 => (day4::solve_part_one(&input), day4::solve_part_two(&input)),
        5 => (day5::solve_part_one(&input), day5::solve_part_two(&input)),
        6 => day6::solve_both(&input),
        11 => (day11::solve_part_one(&input), day11::solve_part_two(&input)),
        _ => todo!("not implemented"),
    };

    println!("Day {day} Part 1: {part1}, Part 2: {part2}");
}
