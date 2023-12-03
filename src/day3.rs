use crate::ascii::IndexAscii;
use core::iter::Iterator;
use regex::Regex;

fn is_symbol(b: &char) -> bool {
    !b.is_ascii_digit() && b != &'.'
}

fn find_symbol_in_cur(cur: &str, col_start: usize, col_end: usize) -> Option<char> {
    if col_start > 0 && is_symbol(&cur.ascii_at(col_start - 1)) {
        Some(cur.ascii_at(col_start - 1))
    } else if col_end < cur.len() && is_symbol(&cur.ascii_at(col_end)) {
        Some(cur.ascii_at(col_end))
    } else {
        None
    }
}

fn find_symbol_in_adj(
    line: &str,
    mut col_start: usize,
    mut col_end: usize,
) -> Option<char> {
    if col_start > 0 {
        col_start -= 1;
    }
    if col_end < line.len() - 1 {
        col_end += 1;
    }
    let line = &line[col_start..col_end];
    line.chars().find(is_symbol)
}

fn find_adjacent_symbol(
    above: Option<&str>,
    cur: &str,
    below: Option<&str>,
    start: usize,
    end: usize,
) -> Option<char> {
    if let Some(sym) = find_symbol_in_cur(cur, start, end) {
        return Some(sym);
    }
    above
        .and_then(|above| find_symbol_in_adj(above, start, end))
        .or_else(|| below.and_then(|below| find_symbol_in_adj(below, start, end)))
}

fn line_sum(
    pat: &Regex,
    above: Option<&str>,
    cur: &str,
    below: Option<&str>,
) -> i64 {
    let mut sum = 0;
    for num_cap in pat.captures_iter(cur).map(|cap| cap.get(1).unwrap()) {
        let (start, end) = (num_cap.start(), num_cap.end());
        if let Some(_) = find_adjacent_symbol(above, cur, below, start, end) {
            let num: i64 = num_cap.as_str().parse().unwrap();
            sum += num;
        }
    }
    sum
}

pub fn part1(input: &str) -> i64 {
    let pat = Regex::new(r"(\d+)").unwrap();
    let mut sum = 0;

    // get sliding windows of lines (above, cur, below)
    let mut lines = input.lines();
    let mut above = lines.next().unwrap();
    let maybe_cur = lines.next();
    if let Some(line) = maybe_cur {
        sum += line_sum(&pat, None, above, Some(line));
    } else {
        return sum;
    }

    let mut cur = maybe_cur.unwrap();
    for line in lines {
        let below = line;
        sum += line_sum(&pat, Some(above), cur, Some(below));
        (above, cur) = (cur, below);
    }

    sum += line_sum(&pat, Some(above), cur, None);

    sum
}

fn line_gear_ratio(
    pat: &Regex,
    above: Option<&str>,
    cur: &str,
    below: Option<&str>,
) -> i64 {
    let mut sum = 0;
    for (i, _) in cur.chars().enumerate().filter(|(_, ch)| *ch == '*') {
        let mut count = 0;
        let mut ratio = 1;

        for num_cap in pat.captures_iter(cur).map(|cap| cap.get(1).unwrap()) {
            if count < 2 {
                let (start, end) = (num_cap.start(), num_cap.end());
                if end == i || start == i + 1 {
                    let num: i64 = num_cap.as_str().parse().unwrap();
                    ratio *= num;
                    count += 1;
                }
            }
        }

        if let Some(above) = above {
            for num_cap in pat.captures_iter(above).map(|cap| cap.get(1).unwrap()) {
                if count < 2 {
                    let (start, end) = (num_cap.start(), num_cap.end());
                    if start <= i + 1 && end >= i {
                        let num: i64 = num_cap.as_str().parse().unwrap();
                        ratio *= num;
                        count += 1;
                    }
                }
            }
        }

        if let Some(below) = below {
            for num_cap in pat.captures_iter(below).map(|cap| cap.get(1).unwrap()) {
                if count < 2 {
                    let (start, end) = (num_cap.start(), num_cap.end());
                    if start <= i + 1 && end >= i {
                        let num: i64 = num_cap.as_str().parse().unwrap();
                        ratio *= num;
                        count += 1;
                    }
                }
            }
        }

        if count == 2 {
            sum += ratio;
        }
    }
    sum
}

pub fn part2(input: &str) -> i64 {
    let pat = Regex::new(r"(\d+)").unwrap();
    let mut sum = 0;

    // get sliding windows of lines (above, cur, below)
    let mut lines = input.lines();
    let mut above = lines.next().unwrap();
    let maybe_cur = lines.next();
    if let Some(line) = maybe_cur {
        sum += line_gear_ratio(&pat, None, above, Some(line));
    } else {
        return sum;
    }

    let mut cur = maybe_cur.unwrap();
    for line in lines {
        let below = line;
        sum += line_gear_ratio(&pat, Some(above), cur, Some(below));
        (above, cur) = (cur, below);
    }

    sum += line_gear_ratio(&pat, Some(above), cur, None);

    sum
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
    }
}
