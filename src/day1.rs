pub fn sum_calibration_values(
    input: &str,
    find_first: impl Fn(&str) -> u32,
    find_last: impl Fn(&str) -> u32,
) -> u32 {
    input.lines().map(|line| find_first(line) * 10 + find_last(line)).sum()
}

fn find_ascii_digit(mut it: impl Iterator<Item = char>) -> Option<u32> {
    it.find_map(|ch| ch.to_digit(10))
}

pub fn part1(input: &str) -> u32 {
    sum_calibration_values(
        input,
        |line| find_ascii_digit(line.chars()).unwrap(),
        |line| find_ascii_digit(line.chars().rev()).unwrap(),
    )
}

const NAMED_DIGITS: &[(&str, u32)] = &[
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

fn find_digit(s: &str, mut range: impl Iterator<Item = usize>) -> Option<u32> {
    let b = s.as_bytes();
    let ascii_digit_at = |i| (b[i] as char).to_digit(10);
    let named_digit_at = |i| {
        NAMED_DIGITS.iter().find_map(|(name, digit)| {
            if name.chars().eq(s.chars().skip(i).take(name.len())) {
                Some(*digit)
            } else {
                None
            }
        })
    };
    range.find_map(|i| ascii_digit_at(i).or_else(|| named_digit_at(i)))
}

pub fn part2(input: &str) -> u32 {
    sum_calibration_values(
        input,
        |s| find_digit(s, 0..s.len()).unwrap(),
        |s| find_digit(s, (0..s.len()).rev()).unwrap(),
    )
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

    #[test]
    fn test_solution() {
        const INPUT: &str = include_str!("../inputs/day1.txt");
        let actual1 = part1(INPUT);
        assert_eq!(54927, actual1);
        let actual2 = part2(INPUT);
        assert_eq!(54581, actual2);
    }
}
