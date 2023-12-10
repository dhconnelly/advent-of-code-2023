use crate::lines::{windows, LineWindow};
use core::{iter::Iterator, ops::Range};
use regex::{CaptureMatches, Match, Regex};

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref SYM_RE: Regex = Regex::new(r"[^.\d]").unwrap();
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

fn is_symbol(ch: char) -> bool {
    !ch.is_ascii_digit() && ch != '.'
}

fn has_adj_symbol((above, cur, below): &LineWindow, m: &Match) -> bool {
    let Range { mut start, mut end } = m.range();
    start = 1.max(start) - 1;
    end = cur.len().min(end + 1);
    let b = cur.as_bytes();
    if is_symbol(b[start] as char) || is_symbol(b[end - 1] as char) {
        return true;
    }
    [above, below]
        .into_iter()
        .flatten()
        .any(|line| line[Range { start, end }].contains(is_symbol))
}

pub fn part1(input: &str) -> i64 {
    sliding_windows_sum(input, |w @ (_, cur, _)| {
        let nums = NUM_RE.captures_iter(cur).map(|m| m.get(0).unwrap());
        let part_nums = nums.filter(|m| has_adj_symbol(&w, m)).map(parse_num);
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
