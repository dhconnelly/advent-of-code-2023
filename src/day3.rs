use core::{iter::Iterator, ops::Range, str::Lines};
use regex::{CaptureMatches, Match, Regex};

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref SYM_RE: Regex = Regex::new(r"[^.\d]").unwrap();
}

// Iterator for (above, cur, below) line windows

type LineWindow<'a> = (Option<&'a str>, &'a str, Option<&'a str>);

struct Windows<'a> {
    lines: Lines<'a>,
    buf: [Option<&'a str>; 2],
}

impl<'a> Iterator for Windows<'a> {
    type Item = LineWindow<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let window = (self.buf[0].take(), self.buf[1].take()?, self.lines.next());
        self.buf = [Some(window.1), window.2];
        Some(window)
    }
}

fn windows(input: &str) -> Windows {
    let mut lines = input.lines();
    let buf = [None, lines.next()];
    Windows { lines, buf }
}

// Iterator for matches of a pattern around a given string

struct Adjacent<'a, 'b> {
    cur: CaptureMatches<'a, 'b>,
    above: Option<CaptureMatches<'a, 'b>>,
    below: Option<CaptureMatches<'a, 'b>>,
    range: Range<usize>,
}

fn find_adjacent<'a, 'b>(
    pat: &'a Regex,
    w: LineWindow<'b>,
    range: Range<usize>,
) -> Adjacent<'a, 'b> {
    let cur = pat.captures_iter(w.1);
    let above = w.0.map(|above| pat.captures_iter(above));
    let below = w.2.map(|below| pat.captures_iter(below));
    Adjacent { cur, above, below, range }
}

impl<'a, 'b> Iterator for Adjacent<'a, 'b> {
    type Item = Match<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        for cap in &mut self.cur {
            let cap = cap.get(0).unwrap();
            if cap.start() == self.range.end || cap.end() == self.range.start {
                return Some(cap);
            }
        }
        for caps in [&mut self.above, &mut self.below].into_iter().flatten() {
            for cap in caps {
                let cap = cap.get(0).unwrap();
                if cap.start() <= self.range.end && cap.end() >= self.range.start {
                    return Some(cap);
                }
            }
        }
        None
    }
}

fn sliding_windows_sum(input: &str, f: impl Fn(LineWindow) -> i64) -> i64 {
    windows(input).map(f).sum()
}

fn parse_num(m: Match) -> i64 {
    m.as_str().parse().unwrap()
}

pub fn part1(input: &str) -> i64 {
    sliding_windows_sum(input, |w @ (_, cur, _)| {
        let nums = NUM_RE.captures_iter(cur).map(|m| m.get(0).unwrap());
        let has_adj_sym = |m: &Match| find_adjacent(&SYM_RE, w, m.range()).count() > 0;
        let part_nums = nums.filter(has_adj_sym).map(parse_num);
        part_nums.sum()
    })
}

pub fn part2(input: &str) -> i64 {
    sliding_windows_sum(input, |w @ (_, cur, _)| {
        let gears = cur.chars().enumerate().filter(|p| p.1 == '*').map(|p| p.0);
        let gear_ratio = |i| {
            let mut adj_nums = find_adjacent(&NUM_RE, w, i..i + 1).map(parse_num);
            let ratio = adj_nums.next()? * adj_nums.next()?;
            Some(ratio).xor(adj_nums.next())
        };
        gears.flat_map(gear_ratio).sum()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        assert_eq!(part1(input), 4361);
        assert_eq!(part2(input), 467835);
    }

    #[test]
    fn test2() {
        let input = include_str!("../inputs/day3.txt");
        assert_eq!(part1(input), 556057);
        assert_eq!(part2(input), 82824352);
    }
}
