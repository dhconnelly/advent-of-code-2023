use core::{iter::Iterator, str::Lines};
use regex::{CaptureMatches, Regex};

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

trait Windowable<'a> {
    fn windows(self) -> Windows<'a>;
}

impl<'a> Windowable<'a> for Lines<'a> {
    fn windows(mut self) -> Windows<'a> {
        let buf = [None, self.next()];
        Windows { lines: self, buf }
    }
}

// Iterator for matches of a pattern around a given string

struct Adjacent<'a, 'b> {
    cur: CaptureMatches<'a, 'b>,
    above: Option<CaptureMatches<'a, 'b>>,
    below: Option<CaptureMatches<'a, 'b>>,
    range: (usize, usize),
}

impl<'a, 'b> Adjacent<'a, 'b> {
    fn new(pat: &'a Regex, w: LineWindow<'b>, range: (usize, usize)) -> Self {
        let cur = pat.captures_iter(w.1);
        let above = w.0.map(|above| pat.captures_iter(above));
        let below = w.2.map(|below| pat.captures_iter(below));
        Self { cur, above, below, range }
    }
}

impl<'a, 'b> Iterator for Adjacent<'a, 'b> {
    type Item = &'b str;

    fn next(&mut self) -> Option<Self::Item> {
        for cap in &mut self.cur {
            let cap = cap.get(0).unwrap();
            if cap.start() == self.range.1 || cap.end() == self.range.0 {
                return Some(cap.as_str());
            }
        }
        for caps in [&mut self.above, &mut self.below].into_iter().flatten() {
            for cap in caps {
                let cap = cap.get(0).unwrap();
                if cap.start() <= self.range.1 && cap.end() >= self.range.0 {
                    return Some(cap.as_str());
                }
            }
        }
        None
    }
}

// part 1

fn line_symbol_sum(num: &Regex, sym: &Regex, window: LineWindow) -> i64 {
    let mut sum = 0;
    for cap in num.captures_iter(window.1).map(|cap| cap.get(0).unwrap()) {
        if Adjacent::new(sym, window, (cap.start(), cap.end())).next().is_some() {
            sum += cap.as_str().parse::<i64>().unwrap();
        }
    }
    sum
}

pub fn part1(input: &str) -> i64 {
    let num = Regex::new(r"\d+").unwrap();
    let sym = Regex::new(r"[^.\d]").unwrap();
    input.lines().windows().map(|w| line_symbol_sum(&num, &sym, w)).sum()
}

// part 2

fn line_gear_sum(pat: &Regex, window: LineWindow) -> i64 {
    let mut sum = 0;
    for (i, _) in window.1.chars().enumerate().filter(|(_, ch)| *ch == '*') {
        let (mut count, mut ratio) = (0, 1);
        for cap in Adjacent::new(pat, window, (i, i + 1)) {
            ratio *= cap.parse::<i64>().unwrap();
            count += 1;
        }
        if count == 2 {
            sum += ratio;
        }
    }
    sum
}

pub fn part2(input: &str) -> i64 {
    let pat = Regex::new(r"\d+").unwrap();
    input.lines().windows().map(|w| line_gear_sum(&pat, w)).sum()
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
