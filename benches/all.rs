use criterion::criterion_main;
// this macro generates a module containing the benchmarks for a single year

macro_rules! benchmark_year {
    ($year:tt; $($day:tt),*) => {
        mod $year {
            use criterion::{criterion_group, Criterion};
            use std::hint::black_box;

            $(
            pub fn $day(c: &mut Criterion) {
                use std::fs::read_to_string;
                use rust_aoc::$year::$day::{parse, solve_part_one, solve_part_two};

                static DAY: &str = stringify!($day);

                let path = "data/input/".to_owned() + stringify!($year) + "/" + stringify!($day) + ".txt";

                let input = read_to_string(path).unwrap();

                c.bench_function(&format!("{}: Parse", DAY), |b| {
                    b.iter(|| parse(black_box(&input)))
                });

                c.bench_function(&format!("{}: Part 1", DAY), |b| {
                    let input = parse(&input);
                    b.iter(|| solve_part_one(black_box(&input)))
                });

                c.bench_function(&format!("{}: Part 2", DAY), |b| {
                    let input = parse(&input);
                    b.iter(|| solve_part_two(black_box(&input)))
                });
            })*

            criterion_group!(benches, $($day, )*);
        }
    };
}

benchmark_year!(year2024; day01, day02, day03, day04, day05, day06, day07, day25);

criterion_main!(year2024::benches);
