use advent_of_code_2023::*;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("usage: advent_of_code_2023 <DAY> <FILE>")]
    Usage,
    #[error("unknown day: {0}")]
    UnknownDay(String),
    #[error("{0}")]
    IO(#[from] std::io::Error),
}

fn die(err: impl Into<Error>) -> ! {
    std::eprintln!("{}", err.into());
    std::process::exit(1);
}

trait Solve {
    fn solve(&self, input: &str);
}

impl<T: Debug + 'static, F: Fn(&str) -> T> Solve for F {
    fn solve(&self, input: &str) {
        println!("{:?}", self(input));
    }
}

type Solver = Box<dyn Solve>;

fn solve(day: &str, input: &str) {
    let solns: &[(&str, Solver, Solver)] = &[
        ("day1", Box::new(day1::part1), Box::new(day1::part2)),
        ("day2", Box::new(day2::part1), Box::new(day2::part2)),
        ("day3", Box::new(day3::part1), Box::new(day3::part2)),
        ("day4", Box::new(day4::part1), Box::new(day4::part2)),
        ("day5", Box::new(day5::part1), Box::new(day5::part2)),
        ("day6", Box::new(day6::part1), Box::new(day6::part2)),
        ("day7", Box::new(day7::part1), Box::new(day7::part2)),
        ("day8", Box::new(day8::part1), Box::new(day8::part2)),
        ("day9", Box::new(day9::part1), Box::new(day9::part2)),
        ("day10", Box::new(day10::part1), Box::new(day10::part2)),
        ("day11", Box::new(day11::part1), Box::new(day11::part2)),
        ("day12", Box::new(day12::part1), Box::new(day12::part2)),
        ("day13", Box::new(day13::part1), Box::new(day13::part2)),
        ("day14", Box::new(day14::part1), Box::new(day14::part2)),
        ("day15", Box::new(day15::part1), Box::new(day15::part2)),
        ("day16", Box::new(day16::part1), Box::new(day16::part2)),
        ("day17", Box::new(day17::part1), Box::new(day17::part2)),
        ("day18", Box::new(day18::part1), Box::new(day18::part2)),
        ("day19", Box::new(day19::part1), Box::new(day19::part2)),
        ("day20", Box::new(day20::part1), Box::new(day20::part2)),
        ("day22", Box::new(day22::part1), Box::new(day22::part2)),
        ("day23", Box::new(day23::part1), Box::new(day23::part2)),
        ("day24", Box::new(day24::part1), Box::new(day24::part2)),
    ];
    let soln = solns
        .iter()
        .find(|soln| soln.0 == day)
        .unwrap_or_else(|| die(Error::UnknownDay(day.to_string())));
    soln.1.solve(input);
    soln.2.solve(input);
}

fn main() {
    let mut args = std::env::args().skip(1);
    let day = args.next().unwrap_or_else(|| die(Error::Usage));
    let path = args.next().unwrap_or_else(|| die(Error::Usage));
    let input = std::fs::read_to_string(path).unwrap_or_else(|err| die(err));
    solve(&day, &input);
}
