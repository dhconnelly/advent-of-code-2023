use core::{iter::Iterator, str::Lines};
use regex::Regex;

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

fn find_adjacent_in_window(
    pat: &Regex,
    (above, cur, below): LineWindow,
    around_range: (usize, usize),
    mut f: impl FnMut(&str),
) {
    for cap in pat.captures_iter(cur).map(|cap| cap.get(0).unwrap()) {
        let (cap_start, cap_end) = (cap.start(), cap.end());
        if cap_start == around_range.1 || cap_end == around_range.0 {
            f(cap.as_str());
        }
    }
    for line in [above, below].into_iter().flatten() {
        for cap in pat.captures_iter(line).map(|cap| cap.get(0).unwrap()) {
            let (cap_start, cap_end) = (cap.start(), cap.end());
            if cap_start <= around_range.1 && cap_end >= around_range.0 {
                f(cap.as_str());
            }
        }
    }
}

fn line_symbol_sum(num_pat: &Regex, sym_pat: &Regex, window: LineWindow) -> i64 {
    let mut sum = 0;
    for num_cap in num_pat.captures_iter(window.1).map(|cap| cap.get(0).unwrap()) {
        let (start, end) = (num_cap.start(), num_cap.end());
        find_adjacent_in_window(sym_pat, window, (start, end), |_| {
            sum += num_cap.as_str().parse::<i64>().unwrap();
        });
    }
    sum
}

pub fn part1(input: &str) -> i64 {
    let num_pat = Regex::new(r"\d+").unwrap();
    let sym_pat = Regex::new(r"[^.\d]").unwrap();
    input.lines().windows().map(|w| line_symbol_sum(&num_pat, &sym_pat, w)).sum()
}

fn line_gear_sum(pat: &Regex, window: LineWindow) -> i64 {
    let (above, cur, below) = window;
    let mut sum = 0;
    for (i, _) in cur.chars().enumerate().filter(|(_, ch)| *ch == '*') {
        let (mut count, mut ratio) = (0, 1);
        find_adjacent_in_window(pat, (above, cur, below), (i, i + 1), |cap| {
            ratio *= cap.parse::<i64>().unwrap();
            count += 1;
        });
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
