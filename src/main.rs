use advent_of_code_2023::{day1, day2};
use std::fmt::Display;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("usage: advent_of_code_2023 <DAY> <FILE>")]
    Usage,
    #[error("unknown day: {0}")]
    UnknownDay(String),
    #[error("{0}")]
    IO(#[from] io::Error),
}

fn die(err: impl Into<Error>) -> ! {
    std::eprintln!("{}", err.into());
    std::process::exit(1);
}

fn output<T: Display, U: Display>((part1, part2): (T, U)) {
    println!("{}", part1);
    println!("{}", part2);
}

fn main() {
    let mut args = std::env::args().skip(1);
    let day = args.next().unwrap_or_else(|| die(Error::Usage));
    let path = args.next().unwrap_or_else(|| die(Error::Usage));
    let input = std::fs::read_to_string(&path).unwrap_or_else(|err| die(err));
    match day.as_str() {
        "day1" => output(day1::run(&input)),
        "day2" => output(day2::run(&input)),
        day => die(Error::UnknownDay(day.to_string())),
    }
}
