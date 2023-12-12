use crate::static_vec::StaticVec;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Row {
    #[default]
    Ok,
    Broken,
    Unknown,
}

impl From<u8> for Row {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Ok,
            b'#' => Self::Broken,
            b'?' => Self::Unknown,
            _ => panic!("invalid token"),
        }
    }
}

type Vec<T> = StaticVec<T, 128>;

fn arrangements((rows, lens): (Vec<Row>, Vec<usize>)) -> i64 {
    arrangements_at(&rows[..], &lens[..]).unwrap()
}

fn place(len: usize, rows: &[Row], lens: &[usize]) -> Option<i64> {
    // try to place |len| broken rows
    if len > rows.len() {
        return None;
    }
    for i in 0..len {
        if rows[i] == Row::Ok {
            return None;
        }
    }
    // ok, make sure we can now skip a working row
    if len >= rows.len() {
        arrangements_at(&rows[len..], lens)
    } else if rows[len] == Row::Broken {
        None
    } else {
        arrangements_at(&rows[len + 1..], lens)
    }
}

fn arrangements_at(rows: &[Row], lens: &[usize]) -> Option<i64> {
    match (rows.iter().next(), lens.iter().next()) {
        (Some(Row::Ok), _) => arrangements_at(&rows[1..], lens),
        (Some(Row::Broken), None) => None,
        (Some(Row::Broken), Some(len)) => place(*len, rows, &lens[1..]),
        (Some(Row::Unknown), None) => arrangements_at(&rows[1..], lens),
        (Some(Row::Unknown), Some(len)) => {
            let here = place(*len, rows, &lens[1..]).unwrap_or(0);
            let there = arrangements_at(&rows[1..], lens).unwrap_or(0);
            Some(here + there)
        }
        (None, Some(_)) => None,
        (None, None) => Some(1),
    }
}

fn parse<'a>(line: &'a str) -> (Vec<Row>, Vec<usize>) {
    let (rows, lens) = line.split_once(' ').unwrap();
    let rows = rows.bytes().map(Row::from).collect();
    let lens = lens.split(',').map(|len| len.parse::<usize>().unwrap()).collect();
    (rows, lens)
}

pub fn part1(input: &str) -> i64 {
    input.lines().map(parse).map(arrangements).sum()
}

pub fn part2(input: &str) -> i64 {
    0
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
            assert_eq!(arrangements(parse(line)), expected[i]);
        }
        assert_eq!(part1(input), 21);
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day12.txt");
        assert_eq!(part1(input), 0);
        assert_eq!(part2(input), 0);
    }
}
