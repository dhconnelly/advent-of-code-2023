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
    fn value(&self) -> Option<i64> {
        if let Self::Valid(outcome) = self {
            Some(*outcome)
        } else {
            None
        }
    }

    fn unwrap(&self) -> i64 {
        self.value().unwrap()
    }

    fn unwrap_or(&self, value: i64) -> i64 {
        self.value().unwrap_or(value)
    }
}

fn arrangements_memoized((springs, lens): (Vec<Spring>, Vec<usize>)) -> i64 {
    let mut m = Matrix::of(Vec::of(Outcome::None));
    arrangements(&springs[..], &lens[..], &mut m).unwrap()
}

fn place(len: usize, springs: &[Spring], lens: &[usize], m: &mut Matrix<Outcome>) -> Outcome {
    // try to place |len| broken springs
    // then, make sure we can now skip a working spring
    if len > springs.len() {
        Outcome::Invalid
    } else if springs[..len].iter().any(|spring| *spring == Spring::Ok) {
        Outcome::Invalid
    } else if len >= springs.len() {
        arrangements(&springs[len..], lens, m)
    } else if springs[len] == Spring::Broken {
        Outcome::Invalid
    } else {
        arrangements(&springs[len + 1..], lens, m)
    }
}

fn arrangements(springs: &[Spring], lens: &[usize], m: &mut Matrix<Outcome>) -> Outcome {
    if let memo @ (Outcome::Valid(_) | Outcome::Invalid) = m[springs.len()][lens.len()] {
        return memo;
    }
    let outcome = match (springs.iter().next(), lens.iter().next()) {
        (Some(Spring::Ok), _) => arrangements(&springs[1..], lens, m),
        (Some(Spring::Broken), None) => Outcome::Invalid,
        (Some(Spring::Broken), Some(len)) => place(*len, springs, &lens[1..], m),
        (Some(Spring::Unknown), None) => arrangements(&springs[1..], lens, m),
        (Some(Spring::Unknown), Some(len)) => {
            let here = place(*len, springs, &lens[1..], m).unwrap_or(0);
            let there = arrangements(&springs[1..], lens, m).unwrap_or(0);
            Outcome::Valid(here + there)
        }
        (None, Some(_)) => Outcome::Invalid,
        (None, None) => Outcome::Valid(1),
    };
    m[springs.len()][lens.len()] = outcome;
    outcome
}

fn parse<'a>(line: &'a str) -> (Vec<Spring>, Vec<usize>) {
    let (springs, lens) = line.split_once(' ').unwrap();
    let springs = springs.bytes().map(Spring::from).collect();
    let lens = lens.split(',').map(|len| len.parse::<usize>().unwrap()).collect();
    (springs, lens)
}

pub fn part1(input: &str) -> i64 {
    input.lines().map(parse).map(arrangements_memoized).sum()
}

fn expand((mut springs, lens): (Vec<Spring>, Vec<usize>)) -> (Vec<Spring>, Vec<usize>) {
    springs.push(Spring::Unknown);
    (
        springs.into_iter().cycle().take(springs.len() * 5 - 1).collect(),
        lens.into_iter().cycle().take(lens.len() * 5).collect(),
    )
}

pub fn part2(input: &str) -> i64 {
    input.lines().map(parse).map(expand).map(arrangements_memoized).sum()
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
        let expected: StaticVec<i64, 6> = StaticVec::from([1, 4, 1, 1, 4, 10]);
        for (i, line) in input.lines().enumerate() {
            assert_eq!(arrangements_memoized(parse(line)), expected[i]);
        }
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
