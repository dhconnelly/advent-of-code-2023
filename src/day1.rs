fn sum_calibration_values(
    input: &str,
    find_first: impl Fn(&str) -> u32,
    find_last: impl Fn(&str) -> u32,
) -> u32 {
    input.lines().map(|line| find_first(line) * 10 + find_last(line)).sum()
}

fn find_ascii_digit(mut line: impl Iterator<Item = char>) -> u32 {
    line.find_map(|ch| ch.to_digit(10)).unwrap()
}

pub fn part1(input: &str) -> u32 {
    sum_calibration_values(
        input,
        |line| find_ascii_digit(line.chars()),
        |line| find_ascii_digit(line.chars().rev()),
    )
}

const NAMED_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight",
    "nine",
];

fn find_digit(line: &str, range: impl Iterator<Item = usize>) -> u32 {
    for i in range {
        if let Some(digit) = (line.as_bytes()[i] as char).to_digit(10) {
            return digit;
        }
        for (digit, name) in NAMED_DIGITS.iter().enumerate() {
            let candidate = &line[i..line.len().min(i + name.len())];
            if candidate == *name {
                return digit as u32;
            }
        }
    }
    unreachable!();
}

pub fn part2(input: &str) -> u32 {
    sum_calibration_values(
        input,
        |line| find_digit(line, 0..line.len()),
        |line| find_digit(line, (0..line.len()).rev()),
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
