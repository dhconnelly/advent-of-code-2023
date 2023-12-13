use crate::static_vec::StaticVec;

type Grid = StaticVec<StaticVec<u8, 32>, 32>;

fn count_diffs<'a>(
    left: impl Iterator<Item = impl Iterator<Item = &'a u8>>,
    right: impl Iterator<Item = impl Iterator<Item = &'a u8>>,
) -> usize {
    left.zip(right).map(|(a, b)| a.zip(b).filter(|(x, y)| x != y).count()).sum()
}

fn reflection_term(grid: Grid, diffs: usize) -> usize {
    for row in 0..grid.len() - 1 {
        let num_rows = (grid.len() - row - 1).min(row + 1);
        let (from, to) = (row + 1 - num_rows, row + num_rows + 1);
        let above = grid[from..row + 1].iter().map(|row| row.iter());
        let below = grid[row + 1..to].iter().map(|row| row.iter()).rev();
        if count_diffs(above, below) == diffs {
            return (row + 1) * 100;
        }
    }
    for col in 0..grid[0].len() - 1 {
        let num_cols = (grid[0].len() - col - 1).min(col + 1);
        let (from, to) = (col + 1 - num_cols, col + num_cols + 1);
        let left = (from..col + 1).map(|col| grid.iter().map(move |row| &row[col]));
        let right = (col + 1..to).map(|col| grid.iter().map(move |row| &row[col]));
        if count_diffs(left, right.rev()) == diffs {
            return col + 1;
        }
    }
    0
}

fn parse(pattern: &str) -> Grid {
    pattern.lines().map(|line| line.bytes().collect()).collect()
}

fn summary(input: &str, diffs: usize) -> usize {
    input.split("\n\n").map(parse).map(|p| reflection_term(p, diffs)).sum()
}

pub fn part1(input: &str) -> usize {
    summary(input, 0)
}

pub fn part2(input: &str) -> usize {
    summary(input, 1)
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
        assert_eq!(part2(input), 400);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day13.txt");
        assert_eq!(part1(input), 27502);
        assert_eq!(part2(input), 31947);
    }
}
