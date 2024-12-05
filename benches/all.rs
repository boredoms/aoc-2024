use criterion::{criterion_group, criterion_main, Criterion};
use rust_aoc::solvers::{day1, day2, day3, day4, day5};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    // let input = std::fs::read_to_string("data/day1/input.txt").unwrap();

    // c.bench_function("Day 1, Part 1", |b| {
    //     b.iter(|| day1::solve_part_one(black_box(&input)))
    // });
    // c.bench_function("Day 1, Part 2", |b| {
    //     b.iter(|| day1::solve_part_two(black_box(&input)))
    // });

    // let input = std::fs::read_to_string("data/day2/input.txt").unwrap();

    // c.bench_function("Day 2, Part 1", |b| {
    //     b.iter(|| day2::solve_part_one(black_box(&input)))
    // });
    // c.bench_function("Day 2, Part 2", |b| {
    //     b.iter(|| day2::solve_part_two(black_box(&input)))
    // });

    // let input = std::fs::read_to_string("data/day3/input.txt").unwrap();

    // c.bench_function("Day 3, Part 1", |b| {
    //     b.iter(|| day3::solve_part_one(black_box(&input)))
    // });
    // c.bench_function("Day 3, Part 2", |b| {
    //     b.iter(|| day3::solve_part_two(black_box(&input)))
    // });

    // let input = std::fs::read_to_string("data/day4/input.txt").unwrap();

    // c.bench_function("Day 4, Part 1", |b| {
    //     b.iter(|| day4::solve_part_one(black_box(&input)))
    // });
    // c.bench_function("Day 4, Part 2", |b| {
    //     b.iter(|| day4::solve_part_two(black_box(&input)))
    // });

    let input = std::fs::read_to_string("data/day5/input.txt").unwrap();

    c.bench_function("Day 5, Part 1", |b| {
        b.iter(|| day5::solve_part_one(black_box(&input)))
    });
    c.bench_function("Day 5, Part 2", |b| {
        b.iter(|| day5::solve_part_two(black_box(&input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
