use advent_of_code_2023::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUTS: &[&str] = &[
    include_str!("../inputs/day1.txt"),
    include_str!("../inputs/day2.txt"),
    include_str!("../inputs/day3.txt"),
    include_str!("../inputs/day4.txt"),
    include_str!("../inputs/day5.txt"),
    include_str!("../inputs/day6.txt"),
    include_str!("../inputs/day7.txt"),
    include_str!("../inputs/day8.txt"),
    include_str!("../inputs/day9.txt"),
    include_str!("../inputs/day10.txt"),
    include_str!("../inputs/day11.txt"),
    include_str!("../inputs/day12.txt"),
    include_str!("../inputs/day13.txt"),
    include_str!("../inputs/day14.txt"),
];

fn benchmark(c: &mut Criterion) {
    c.bench_function("day1part1", |b| b.iter(|| day1::part1(black_box(INPUTS[0]))));
    c.bench_function("day1part2", |b| b.iter(|| day1::part2(black_box(INPUTS[0]))));
    c.bench_function("day2part1", |b| b.iter(|| day2::part1(black_box(INPUTS[1]))));
    c.bench_function("day2part2", |b| b.iter(|| day2::part2(black_box(INPUTS[1]))));
    c.bench_function("day3part1", |b| b.iter(|| day3::part1(black_box(INPUTS[2]))));
    c.bench_function("day3part2", |b| b.iter(|| day3::part2(black_box(INPUTS[2]))));
    c.bench_function("day4part1", |b| b.iter(|| day4::part1(black_box(INPUTS[3]))));
    c.bench_function("day4part2", |b| b.iter(|| day4::part2(black_box(INPUTS[3]))));
    c.bench_function("day5part1", |b| b.iter(|| day5::part1(black_box(INPUTS[4]))));
    c.bench_function("day5part2", |b| b.iter(|| day5::part2(black_box(INPUTS[4]))));
    c.bench_function("day6part1", |b| b.iter(|| day6::part1(black_box(INPUTS[5]))));
    c.bench_function("day6part2", |b| b.iter(|| day6::part2(black_box(INPUTS[5]))));
    c.bench_function("day7part1", |b| b.iter(|| day7::part1(black_box(INPUTS[6]))));
    c.bench_function("day7part2", |b| b.iter(|| day7::part2(black_box(INPUTS[6]))));
    c.bench_function("day8part1", |b| b.iter(|| day8::part1(black_box(INPUTS[7]))));
    c.bench_function("day8part2", |b| b.iter(|| day8::part2(black_box(INPUTS[7]))));
    c.bench_function("day9part1", |b| b.iter(|| day9::part1(black_box(INPUTS[8]))));
    c.bench_function("day9part2", |b| b.iter(|| day9::part2(black_box(INPUTS[8]))));
    c.bench_function("day10part1", |b| b.iter(|| day10::part1(black_box(INPUTS[9]))));
    c.bench_function("day10part2", |b| b.iter(|| day10::part2(black_box(INPUTS[9]))));
    c.bench_function("day11part1", |b| b.iter(|| day11::part1(black_box(INPUTS[10]))));
    c.bench_function("day11part2", |b| b.iter(|| day11::part2(black_box(INPUTS[10]))));
    c.bench_function("day12part1", |b| b.iter(|| day12::part1(black_box(INPUTS[11]))));
    c.bench_function("day12part2", |b| b.iter(|| day12::part2(black_box(INPUTS[11]))));
    c.bench_function("day13part1", |b| b.iter(|| day13::part1(black_box(INPUTS[12]))));
    c.bench_function("day13part2", |b| b.iter(|| day13::part2(black_box(INPUTS[12]))));
    c.bench_function("day14part1", |b| b.iter(|| day14::part1(black_box(INPUTS[13]))));
    c.bench_function("day14part2", |b| b.iter(|| day14::part2(black_box(INPUTS[13]))));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
