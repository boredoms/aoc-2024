use std::{path::PathBuf, str::FromStr};

use rust_aoc::year2024;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TestOrRun {
    Test,
    Run,
    Exec,
    Unrecognized,
}

impl FromStr for TestOrRun {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "test" => Ok(Self::Test),
            "run" => Ok(Self::Run),
            _ => Err("option not recognized"),
        }
    }
}

#[derive(Debug)]
struct RunConfig {
    command: TestOrRun,
    year: u32,
    days: Vec<u32>,
    filename: Option<String>,
}

impl RunConfig {
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<RunConfig, &'static str> {
        args.next();

        let command = match args.next() {
            Some(s) => TestOrRun::from_str(&s)?,
            None => return Err("no command provided"),
        };

        let year = match args.next() {
            Some(s) => s.parse().or(Err("could not parse year"))?,
            None => return Err("no year provided"),
        };

        let mut days = Vec::new();

        // consume the remaining iterator
        for s in args {
            let day = s.parse().or(Err("could not parse day"))?;

            if day > 25 {
                return Err("day is outside of the allowed range [0, 25]");
            }

            days.push(day);
        }

        Ok(RunConfig {
            command,
            year,
            days,
            filename: None,
        })
    }
}

fn print_usage() {
    println!("  Usage: rust_aoc [run|test] [year:20XX] [day: YY]");
}

fn create_filename(year: u32, day: u32, cmd: TestOrRun) -> PathBuf {
    let t = match cmd {
        TestOrRun::Test => "test",
        TestOrRun::Run => "input",
        _ => unreachable!(),
    };

    let y = "year".to_string() + &year.to_string();
    let f = "day".to_string() + &format!("{:02}.txt", day);

    std::path::Path::new("data/").join(t).join(y).join(f)
}

fn dispatch(year: u32, day: u32, cmd: TestOrRun) -> Result<(String, String), String> {
    let path = create_filename(year, day, cmd);
    let filename = path.to_str().expect("path is valid unicode");

    match year {
        2024 => match day {
            1 => year2024::day01::solve(filename),
            2 => year2024::day02::solve(filename),
            3 => year2024::day03::solve(filename),
            4 => year2024::day04::solve(filename),
            5 => year2024::day05::solve(filename),
            6 => year2024::day06::solve(filename),
            7 => year2024::day07::solve(filename),
            8 => year2024::day08::solve(filename),
            9 => year2024::day09::solve(filename),
            10 => year2024::day10::solve(filename),
            11 => year2024::day11::solve(filename),
            12 => year2024::day12::solve(filename),
            13 => year2024::day13::solve(filename),
            14 => year2024::day14::solve(filename),
            15 => year2024::day15::solve(filename),
            16 => year2024::day16::solve(filename),
            17 => year2024::day17::solve(filename),
            18 => year2024::day18::solve(filename),
            19 => year2024::day19::solve(filename),
            20 => year2024::day20::solve(filename),
            21 => year2024::day21::solve(filename),
            22 => year2024::day22::solve(filename),
            23 => year2024::day23::solve(filename),
            24 => year2024::day24::solve(filename),
            25 => year2024::day25::solve(filename),
            _ => Err(format!("day not implemented {}", day)),
        },
        _ => Err(format!("year not implemented {}", year)),
    }
}

fn print_solution(year: u32, day: u32, part1: &str, part2: &str) {
    println!(
        "Advent of Code {}, Day {}:
  Part 1: {}
  Part 2: {}
",
        year, day, part1, part2
    );
}

fn main() {
    let args = std::env::args(); // iterator over the arguments

    let config = match RunConfig::parse(args) {
        Ok(cfg) => cfg,
        Err(s) => {
            println!("Error: {}", s);
            print_usage();
            std::process::exit(1);
        }
    };

    if config.days.is_empty() {
        for d in 1..=25 {
            let (part1, part2) = match dispatch(config.year, d, config.command) {
                Ok(x) => x,
                Err(s) => {
                    println!("Error: {}", s);
                    std::process::exit(1);
                }
            };

            print_solution(config.year, d, &part1, &part2);
        }
    } else {
        for d in &config.days {
            let (part1, part2) = match dispatch(config.year, *d, config.command) {
                Ok(x) => x,
                Err(s) => {
                    println!("Error: {}", s);
                    std::process::exit(1);
                }
            };

            print_solution(config.year, *d, &part1, &part2);
        }
    }
}
