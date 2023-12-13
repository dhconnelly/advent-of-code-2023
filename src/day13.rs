use crate::static_vec::StaticVec;
use libc_print::std_name::*;

type Grid = StaticVec<StaticVec<u8, 32>, 32>;

fn parse(pattern: &str) -> Grid {
    pattern.lines().map(|line| line.bytes().collect()).collect()
}

fn reflection_term(grid: Grid) -> usize {
    for row in 0..grid.len() - 1 {
        let num_rows = (grid.len() - row - 1).min(row + 1);
        let from = row + 1 - num_rows;
        let to = row + num_rows + 1;
        if grid[from..row + 1].iter().eq(grid[row + 1..to].iter().rev()) {
            println!("match: {}-{}-{}", from, row + 1, to);
            return (row + 1) * 100;
        }
    }
    for col in 0..grid[0].len() - 1 {
        let num_cols = (grid[0].len() - col - 1).min(col + 1);
        let from = col + 1 - num_cols;
        let to = col + num_cols + 1;
        let a = (from..col + 1).map(|col| grid.iter().map(move |row| row[col]));
        let b = (col + 1..to).map(|col| grid.iter().map(move |row| row[col]));
        if a.zip(b.rev()).all(|(a, b)| a.eq(b)) {
            println!("match col: {}-{}-{}", from, col + 1, to);
            return col + 1;
        }
    }
    0
}

pub fn part1(input: &str) -> usize {
    input.split("\n\n").map(parse).map(reflection_term).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        assert_eq!(part1(input), 405);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day13.txt");
        assert_eq!(part1(input), 0);
    }
}
