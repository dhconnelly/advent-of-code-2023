use core::iter::Iterator;
use core::str::Split;
use regex::Regex;

struct Outcome(i64, i64, i64);

struct Outcomes<'a, 'b> {
    pat: &'b Regex,
    segs: Split<'a, char>,
}

impl Iterator for Outcomes<'_, '_> {
    type Item = Outcome;

    fn next(&mut self) -> Option<Self::Item> {
        let seg = self.segs.next()?;
        let mut outcome = Outcome(0, 0, 0);
        for (_, [amt, col]) in self.pat.captures_iter(seg).map(|c| c.extract()) {
            let amt = amt.parse().unwrap();
            match col {
                "red" => outcome.0 = amt,
                "green" => outcome.1 = amt,
                "blue" => outcome.2 = amt,
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
        let (_, [id]) = self.game_pat.captures(game).unwrap().extract();
        let outcome = Outcomes { segs: outcomes.split(';'), pat: &self.outcome_pat };
        (id.parse().unwrap(), outcome)
    }
}

fn can_fit(into: &Outcome, from: &Outcome) -> bool {
    from.0 <= into.0 && from.1 <= into.1 && from.2 <= into.2
}

pub fn part1(input: &str) -> usize {
    let parser = Parser::new();
    let available = Outcome(12, 13, 14);
    let all_games = input.lines().map(|line| parser.parse(line));
    let possible_games = all_games.flat_map(|(id, mut outcomes)| {
        if outcomes.all(|outcome| can_fit(&available, &outcome)) {
            Some(id)
        } else {
            None
        }
    });
    possible_games.sum()
}

fn power(outcomes: impl Iterator<Item = Outcome>) -> i64 {
    let mut min = Outcome(0, 0, 0);
    for outcome in outcomes {
        min.0 = min.0.max(outcome.0);
        min.1 = min.1.max(outcome.1);
        min.2 = min.2.max(outcome.2);
    }
    min.0 * min.1 * min.2
}

pub fn part2(input: &str) -> i64 {
    let p = Parser::new();
    let all_games = input.lines().map(|line| p.parse(line));
    let powers = all_games.map(|(_, outcomes)| power(outcomes));
    powers.sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT), 8);
    }

    #[test]
    fn test2_1() {
        let mut lines = INPUT.lines();
        let p = Parser::new();
        assert_eq!(power(p.parse(lines.next().unwrap()).1), 48);
        assert_eq!(power(p.parse(lines.next().unwrap()).1), 12);
        assert_eq!(power(p.parse(lines.next().unwrap()).1), 1560);
        assert_eq!(power(p.parse(lines.next().unwrap()).1), 630);
        assert_eq!(power(p.parse(lines.next().unwrap()).1), 36);
        assert_eq!(lines.next(), None);
    }

    #[test]
    fn test2_2() {
        assert_eq!(part2(INPUT), 2286);
    }

    #[test]
    fn test_solution() {
        const INPUT: &str = include_str!("../inputs/day2.txt");
        let actual1 = part1(INPUT);
        assert_eq!(2204, actual1);
        let actual2 = part2(INPUT);
        assert_eq!(71036, actual2);
    }
}
