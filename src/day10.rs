use core::fmt::Display;
use libc_print::std_name::*;

use crate::static_vec::StaticVec;

struct Grid {
    data: StaticVec<u8, 32768>,
    width: usize,
    height: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                write!(f, "{}", self.data[i * self.width + j] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Grid {
    let mut tiles = StaticVec::empty();
    let (mut width, mut height) = (0, 0);
    for line in input.lines() {
        width = 0;
        for tile in line.bytes() {
            tiles.push(tile);
            width += 1;
        }
        height += 1;
    }
    Grid { data: tiles, width, height }
}

pub fn part1(input: &str) -> i64 {
    println!("{}", parse(input));
    0
}

pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
        assert_eq!(part1(input), 4);
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn test_examples() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
        assert_eq!(part1(input), 8);
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day10.txt");
        assert_eq!(part1(input), 0);
        assert_eq!(part2(input), 0);
    }
}
