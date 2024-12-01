use clap::Parser;
use rust_aoc::{commands::init::init, Args, Command};

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Init => init(args.day),
        _ => todo!("Not implemented."),
    }

    println!("{:?}", args);
}
