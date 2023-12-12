use crate::static_vec::StaticVec;
use libc_print::std_name::*;

#[derive(Debug, Clone, Copy, Default)]
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

fn arrangements((rows, lens): (Vec<Row>, Vec<i64>)) -> i64 {
    0
}

fn arrangements_at(i: usize, rows: &[Row], lens: &[i64]) -> Option<i64> {
    None
}

fn parse<'a>(line: &'a str) -> (Vec<Row>, Vec<i64>) {
    let (rows, lens) = line.split_once(' ').unwrap();
    let rows = rows.bytes().map(Row::from).collect();
    let lens = lens.split(',').map(|len| len.parse::<i64>().unwrap()).collect();
    println!("{:?} {:?}", rows, lens);
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
        let input = "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1
";
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
