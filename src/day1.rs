use libc_print::std_name::{dbg, eprintln, println};

fn find_ascii_digit(mut it: impl Iterator<Item = char>) -> Option<u32> {
    it.find_map(|ch| ch.to_digit(10))
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let a = find_ascii_digit(line.chars()).unwrap();
            let b = find_ascii_digit(line.chars().rev()).unwrap();
            a * 10 + b
        })
        .sum()
}

const DIGITS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("ten", 10),
];

fn find_first_digit(s: &str) -> Option<u32> {
    for i in 0..s.len() {
        let val = s.as_bytes()[i] as char;
        if val.is_ascii_digit() {
            return val.to_digit(10);
        }
        for (name, digit) in DIGITS {
            if &s[i..].find(name) == &Some(0usize) {
                return Some(*digit);
            }
        }
    }
    None
}

fn find_last_digit(s: &str) -> Option<u32> {
    for i in 0..s.len() {
        let i = s.len() - i - 1;
        let val = s.as_bytes()[i] as char;
        if val.is_ascii_digit() {
            return val.to_digit(10);
        }
        for (name, digit) in DIGITS {
            println!("last {} {:?} {:?}", i, &s[..i + 1].rfind(name), name);
            if i + 1 >= name.len() && &s[..i + 1].rfind(name) == &Some(i + 1 - name.len()) {
                return Some(*digit);
            }
        }
    }
    None
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let a = dbg!(find_first_digit(line).unwrap());
            let b = dbg!(find_last_digit(line).unwrap());
            a * 10 + b
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let expected = 142;
        let actual = part1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let expected = 281;
        let actual = part2(input);
        assert_eq!(expected, actual);
    }
}
