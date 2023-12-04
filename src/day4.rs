use core::u128;
use libc_print::std_name::{dbg, println};

use regex::{Captures, Regex};

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
}

fn parse(m: &Captures) -> u8 {
    m.get(0).unwrap().as_str().parse().unwrap()
}

fn set_of(nums: &str) -> u128 {
    NUM_RE.captures_iter(nums).fold(0, |acc, m| dbg!(acc | (1 << parse(&m))))
}

fn set_contains(set: u128, num: u8) -> bool {
    set & (1 << num) > 0
}

fn score_card(card: &str) -> i64 {
    let (_, nums) = card.split_at(card.find(':').unwrap());
    let (winning, have) = nums.split_at(nums.find('|').unwrap());
    let winning_set = set_of(winning);
    let num_wins = NUM_RE
        .captures_iter(have)
        .filter(|m| set_contains(winning_set, parse(m)))
        .count();
    if num_wins == 0 {
        0
    } else {
        1 << (num_wins - 1)
    }
}

pub fn part1(input: &str) -> i64 {
    input.lines().map(score_card).sum()
}

pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test() {
        assert_eq!(part1(INPUT), 13);
    }
}
