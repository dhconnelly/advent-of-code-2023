use crate::static_vec::StaticVec;

type Vec<T> = StaticVec<T, 128>;
type Matrix<T> = Vec<Vec<T>>;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Spring {
    #[default]
    Ok,
    Broken,
    Unknown,
}

impl From<u8> for Spring {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Ok,
            b'#' => Self::Broken,
            b'?' => Self::Unknown,
            _ => panic!("invalid token"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
enum Outcome {
    #[default]
    None,
    Invalid,
    Valid(i64),
}

impl Outcome {
    fn unwrap_or(&self, value: i64) -> i64 {
        if let Self::Valid(outcome) = self {
            *outcome
        } else {
            value
        }
    }
}

fn arrangements_memoized(springs: &[Spring], lens: &[usize]) -> i64 {
    let mut m = Matrix::of(Vec::from([Outcome::None; 128]));
    place(springs, lens, &mut m).unwrap_or(0)
}

// count how many ways we can place |len| broken springs at the *beginning* of
// the array |springs| and the remaining |lens| of broken spring lengths anywhere
// in the remaining array
fn place_here(len: usize, springs: &[Spring], lens: &[usize], m: &mut Matrix<Outcome>) -> Outcome {
    // try to place |len| broken springs
    // then, make sure we can now skip a working spring
    if len > springs.len() || springs[..len].iter().any(|spring| *spring == Spring::Ok) {
        Outcome::Invalid
    } else if len >= springs.len() {
        place(&springs[len..], lens, m)
    } else if springs[len] == Spring::Broken {
        Outcome::Invalid
    } else {
        place(&springs[len + 1..], lens, m)
    }
}

// count how many ways we can place strings of broken strings of lengths |lens| into
// the array |springs|
fn place(springs: &[Spring], lens: &[usize], m: &mut Matrix<Outcome>) -> Outcome {
    if let memo @ (Outcome::Valid(_) | Outcome::Invalid) = m[springs.len()][lens.len()] {
        return memo;
    }
    let outcome = match (springs.iter().next(), lens.iter().next()) {
        (None, None) => Outcome::Valid(1),
        (None, Some(_)) => Outcome::Invalid,
        (Some(Spring::Ok), _) => place(&springs[1..], lens, m),
        (Some(Spring::Broken), None) => Outcome::Invalid,
        (Some(Spring::Broken), Some(len)) => place_here(*len, springs, &lens[1..], m),
        (Some(Spring::Unknown), None) => place(&springs[1..], lens, m),
        (Some(Spring::Unknown), Some(len)) => {
            let here = place_here(*len, springs, &lens[1..], m).unwrap_or(0);
            let there = place(&springs[1..], lens, m).unwrap_or(0);
            Outcome::Valid(here + there)
        }
    };
    m[springs.len()][lens.len()] = outcome;
    outcome
}

fn parse(line: &str, springs: &mut Vec<Spring>, lens: &mut Vec<usize>) {
    let (lhs, rhs) = line.split_once(' ').unwrap();
    for spring in lhs.bytes().map(Spring::from) {
        springs.push(spring);
    }
    for len in rhs.split(',').map(|len| len.parse::<usize>().unwrap()) {
        lens.push(len);
    }
}

fn expand(by: usize, springs: &mut Vec<Spring>, lens: &mut Vec<usize>) {
    let springs_len = springs.len();
    let lens_len = lens.len();
    for _ in 1..by {
        springs.push(Spring::Unknown);
        for j in 0..springs_len {
            springs.push(springs[j]);
        }
        for j in 0..lens_len {
            lens.push(lens[j]);
        }
    }
}

fn sum_arrangements(input: &str, copies: usize) -> i64 {
    let mut sum = 0;
    let mut springs = Vec::empty();
    let mut lens = Vec::empty();
    for line in input.lines() {
        springs.clear();
        lens.clear();
        parse(line, &mut springs, &mut lens);
        expand(copies, &mut springs, &mut lens);
        sum += arrangements_memoized(&springs[..], &lens[..]);
    }
    sum
}

pub fn part1(input: &str) -> i64 {
    sum_arrangements(input, 1)
}

pub fn part2(input: &str) -> i64 {
    sum_arrangements(input, 5)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        assert_eq!(part1(input), 21);
        assert_eq!(part2(input), 525152);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day12.txt");
        assert_eq!(part1(input), 8419);
        assert_eq!(part2(input), 160500973317706);
    }
}
