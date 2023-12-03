use core::iter::Iterator;
use regex::Regex;

fn find_adjacent_in_window(
    pat: &Regex,
    above: Option<&str>,
    cur: &str,
    below: Option<&str>,
    start: usize,
    end: usize,
    mut f: impl FnMut(&str),
) {
    for cap in pat.captures_iter(cur).map(|cap| cap.get(0).unwrap()) {
        let (cap_start, cap_end) = (cap.start(), cap.end());
        if cap_start == end || cap_end == start {
            f(cap.as_str());
        }
    }
    for line in [above, below].into_iter().flatten() {
        for cap in pat.captures_iter(line).map(|cap| cap.get(0).unwrap()) {
            let (cap_start, cap_end) = (cap.start(), cap.end());
            if cap_start <= end && cap_end >= start {
                f(cap.as_str());
            }
        }
    }
}

fn process_windows(
    input: &str,
    mut f: impl FnMut(Option<&str>, &str, Option<&str>),
) {
    // get sliding windows of lines (above, cur, below)
    let mut lines = input.lines();
    let mut above = lines.next().unwrap();

    // first line
    let maybe_cur = lines.next();
    if let Some(line) = maybe_cur {
        f(None, above, Some(line));
    } else {
        return;
    }

    // middle lines
    let mut cur = maybe_cur.unwrap();
    for line in lines {
        let below = line;
        f(Some(above), cur, Some(below));
        (above, cur) = (cur, below);
    }

    // last line
    f(Some(above), cur, None);
}

fn line_sum(
    num_pat: &Regex,
    sym_pat: &Regex,
    above: Option<&str>,
    cur: &str,
    below: Option<&str>,
) -> i64 {
    let mut sum = 0;
    for num_cap in num_pat.captures_iter(cur).map(|cap| cap.get(0).unwrap()) {
        let (start, end) = (num_cap.start(), num_cap.end());
        find_adjacent_in_window(sym_pat, above, cur, below, start, end, |_| {
            sum += num_cap.as_str().parse::<i64>().unwrap();
        });
    }
    sum
}

pub fn part1(input: &str) -> i64 {
    let num_pat = Regex::new(r"\d+").unwrap();
    let sym_pat = Regex::new(r"[^.\d]").unwrap();
    let mut sum = 0;
    process_windows(input, |above, cur, below| {
        sum += line_sum(&num_pat, &sym_pat, above, cur, below);
    });
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
        find_adjacent_in_window(pat, above, cur, below, i, i + 1, |cap| {
            if count < 2 {
                let num: i64 = cap.parse().unwrap();
                ratio *= num;
                count += 1;
            }
        });
        if count == 2 {
            sum += ratio;
        }
    }
    sum
}

pub fn part2(input: &str) -> i64 {
    let pat = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    process_windows(input, |above, cur, below| {
        sum += line_gear_ratio(&pat, above, cur, below);
    });
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
        assert_eq!(part2(input), 82824352);
    }
}
