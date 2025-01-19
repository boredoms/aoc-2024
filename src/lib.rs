use std::path::{Path, PathBuf};

use clap::{Parser, ValueEnum};

pub mod year2024;

pub mod util {
    pub mod grid;
    pub mod point;
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Command {
    Init,
    Download,
    Test,
    Run,
}

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(value_enum)]
    pub command: Command,
    pub day: u8,
    pub download_flag: Option<bool>,
}

// a problem consists of a day, year and a closure to execute
#[derive(Debug)]
struct Problem {
    year: u16,
    day: u8,
    path: PathBuf,
    task: fn(String) -> (usize, usize),
}

// macro_rules! problem {
//     ($year:tt, $day:tt) => {
//         fn $year() -> Problem {
//             Problem {
//                 year: 0,
//                 day: 0,
//                 path: Path::new("data").to_owned(),
//                 task: |s| {
//                     use solvers::$day::*;

//                     (solve_part_one(&s), solve_part_two(&s))
//                 },
//             }
//         }
//     };
// }

// problem! {yongus, day01}

// fn bap() {
//     let p = yongus();
// }
