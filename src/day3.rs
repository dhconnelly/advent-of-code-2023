use core::iter::Iterator;
use regex::Regex;

type LineWindow<'a> = (Option<&'a str>, &'a str, Option<&'a str>);

fn find_adjacent_in_window(
    pat: &Regex,
    window: LineWindow,
    around_range: (usize, usize),
    mut f: impl FnMut(&str),
) {
    let (start, end) = around_range;
    let (above, cur, below) = window;
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

fn sum_windows(input: &str, mut f: impl FnMut(LineWindow) -> i64) -> Option<i64> {
    // process sliding windows (above, cur, below)
    let mut lines = input.lines();

    // first line
    let mut above = lines.next()?;
    let next = lines.next();
    let mut sum = f((None, above, next));

    // abort if done
    let mut cur = if let Some(next) = next {
        next
    } else {
        return Some(sum);
    };

    // middle lines
    for line in lines {
        sum += f((Some(above), cur, Some(line)));
        (above, cur) = (cur, line);
    }

    // last line
    sum += f((Some(above), cur, None));
    Some(sum)
}

fn line_sum(num_pat: &Regex, sym_pat: &Regex, window: LineWindow) -> i64 {
    let (_, cur, _) = window;
    let mut sum = 0;
    for num_cap in num_pat.captures_iter(cur).map(|cap| cap.get(0).unwrap()) {
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
    sum_windows(input, |window| line_sum(&num_pat, &sym_pat, window)).unwrap()
}

fn line_gear_ratio(pat: &Regex, window: LineWindow) -> i64 {
    let (above, cur, below) = window;
    let mut sum = 0;
    for (i, _) in cur.chars().enumerate().filter(|(_, ch)| *ch == '*') {
        let mut count = 0;
        let mut ratio = 1;
        find_adjacent_in_window(pat, (above, cur, below), (i, i + 1), |cap| {
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
    sum_windows(input, |window| line_gear_ratio(&pat, window)).unwrap()
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
