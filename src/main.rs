use advent_of_code_2023::{day1, day2, day3};
use std::fmt::Display;
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

trait Solver {
    fn solve(&self, input: &str);
}

impl<T: Display + 'static, F: Fn(&str) -> T> Solver for F {
    fn solve(&self, input: &str) {
        println!("{}", self(input));
    }
}

fn solve(day: &str, input: &str) {
    let solns: &[(&str, Box<dyn Solver>, Box<dyn Solver>)] = &[
        ("day1", Box::new(day1::part1), Box::new(day1::part2)),
        ("day2", Box::new(day2::part1), Box::new(day2::part2)),
        ("day3", Box::new(day3::part1), Box::new(day3::part2)),
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
    let input = std::fs::read_to_string(&path).unwrap_or_else(|err| die(err));
    solve(&day, &input);
}
