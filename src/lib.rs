use clap::{Parser, ValueEnum};

pub mod commands;
pub mod solvers;

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

#[derive(Debug)]
struct Problem {
    test_data: String,
    test_solution: String,
    input: String,
}

#[derive(Debug)]
struct Day {
    day: u8,
    problem: Problem,
}
