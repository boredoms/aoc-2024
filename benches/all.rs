use criterion::{criterion_group, criterion_main, Criterion};
use rust_aoc::solvers::{day1, day2, day3};
use std::hint::black_box;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/day1/input.txt").unwrap();

    c.bench_function("Day 1, Part 1", |b| {
        b.iter(|| day1::solve_part_one(black_box(&input)))
    });
    c.bench_function("Day 1, Part 2", |b| {
        b.iter(|| day1::solve_part_two(black_box(&input)))
    });

    let input = std::fs::read_to_string("data/day2/input.txt").unwrap();

    c.bench_function("Day 2, Part 1", |b| {
        b.iter(|| day2::solve_part_one(black_box(&input)))
    });
    c.bench_function("Day 2, Part 2", |b| {
        b.iter(|| day2::solve_part_two(black_box(&input)))
    });

    let input = std::fs::read_to_string("data/day3/input.txt").unwrap();

    c.bench_function("Day 3, Part 1", |b| {
        b.iter(|| day3::solve_part_one(black_box(&input)))
    });
    c.bench_function("Day 3, Part 2", |b| {
        b.iter(|| day3::solve_part_two(black_box(&input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
