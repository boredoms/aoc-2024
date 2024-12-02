use clap::Parser;
use rust_aoc::{commands::init::init, commands::run::run, Args, Command};

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Init => init(args.day),
        Command::Run => run(args.day.into()),
        _ => todo!("Not implemented."),
    }

    println!("{:?}", args);
}
