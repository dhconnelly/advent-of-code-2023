use core::iter::Iterator;
use core::str::Split;

use libc_print::std_name::println;
use regex::Regex;

#[derive(Debug)]
struct Outcome {
    r: i64,
    g: i64,
    b: i64,
}

impl Outcome {
    fn new(r: i64, g: i64, b: i64) -> Self {
        Self { r, g, b }
    }
}

fn parse(pat: &Regex, line: &str) -> (usize, Outcome) {
    (0, Outcome::new(0, 0, 0))
}

struct Outcomes<'a, 'b> {
    pat: &'b Regex,
    segs: Split<'a, char>,
}

impl Iterator for Outcomes<'_, '_> {
    type Item = Outcome;
    fn next(&mut self) -> Option<Self::Item> {
        let seg = self.segs.next()?;
        let mut outcome = Outcome::new(0,0,0);
        for (_, [amt, col]) in self.pat.captures_iter(seg).map(|c| c.extract()) {
            let amt = amt.parse().unwrap();
            match col {
                "red" => outcome.r = amt,
                "green" => outcome.g = amt,
                "blue" => outcome.b = amt,
                _ => panic!("invalid color"),
            }
        }
        Some(outcome)
    }
}

struct Parser {
    game_pat: Regex,
    outcome_pat: Regex,
}

impl Parser {
    fn new() -> Self {
        let game_pat = Regex::new(r"Game (\d+)").unwrap();
        let outcome_pat = Regex::new(r"(\d+) (red|blue|green)").unwrap();
        Self { game_pat, outcome_pat }
    }

    fn parse<'a, 'b>(&'b self, line: &'a str) -> (usize, Outcomes<'a, 'b>) {
        let (game, outcomes) = line.split_once(':').unwrap();
        let game_caps = self.game_pat.captures(game).unwrap();
        let (_, [id]) = game_caps.extract();
        let id = id.parse().unwrap();
        let segs = outcomes.split(';');
        (id, Outcomes { segs, pat: &self.outcome_pat })
    }
}

fn can_fit(into: &Outcome, from: &Outcome) -> bool {
    from.r <= into.r && from.g <= into.g && from.b <= into.b
}

pub fn part1(input: &str) -> usize {
    let want = Outcome::new(12, 13, 14);
    let p = Parser::new();
    let mut sum = 0;
    for (id, outcomes) in input.lines().map(|line| p.parse(line)) {
        let mut ok = true;
        for outcome in outcomes {
            if !can_fit(&want, &outcome) {
                ok = false;
                break;
            }
        }
        if ok {
            sum += id;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let output = part1(input);
        let expected = 8;
        assert_eq!(output, expected);
    }
}
