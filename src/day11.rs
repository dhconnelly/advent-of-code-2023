use crate::static_vec::StaticVec;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
enum Tile {
    #[default]
    Empty,
    Galaxy,
}

type Pt2 = (u16, u16);
type Grid<'a> = StaticVec<StaticVec<Tile, 256>, 256>;
type Weights = (StaticVec<i64, 256>, StaticVec<i64, 256>);

fn parse(input: &str, grid: &mut Grid) {
    for (i, line) in input.lines().enumerate() {
        grid.push(StaticVec::empty());
        for b in line.bytes() {
            match b {
                b'.' => grid[i].push(Tile::Empty),
                b'#' => grid[i].push(Tile::Galaxy),
                _ => panic!("invalid tile"),
            }
        }
    }
}

fn expand(grid: &Grid, multiplier: i64) -> Weights {
    let row_weights = grid
        .iter()
        .map(|row| if row.iter().all(|t| *t == Tile::Empty) { multiplier } else { 1 })
        .collect();
    let is_col_empty = |j: usize| (0..grid.len()).all(|i| grid[i][j] == Tile::Empty);
    let col_weights = (0..grid[0].len())
        .map(|j| if is_col_empty(j) { multiplier } else { 1 })
        .collect();
    (row_weights, col_weights)
}

fn shortest_path(weights: &Weights, from: Pt2, to: Pt2) -> i64 {
    let (from_row, from_col) = (from.0 as usize, from.1 as usize);
    let (to_row, to_col) = (to.0 as usize, to.1 as usize);
    let row_dist: i64 =
        weights.0[from_row.min(to_row) + 1..from_row.max(to_row) + 1].iter().sum();
    let col_dist: i64 =
        weights.1[from_col.min(to_col) + 1..from_col.max(to_col) + 1].iter().sum();
    row_dist + col_dist
}

fn sum_shortest_paths(input: &str, multiplier: i64) -> i64 {
    let mut grid = Grid::empty();
    parse(input, &mut grid);
    let weights = expand(&grid, multiplier);
    let mut galaxies = StaticVec::<Pt2, 4096>::empty();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == Tile::Galaxy {
                let from = (i as u16, j as u16);
                galaxies.push(from);
            }
        }
    }
    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        let from = galaxies[i];
        for j in i + 1..galaxies.len() {
            let to = galaxies[j];
            sum += shortest_path(&weights, from, to);
        }
    }
    sum
}

pub fn part1(input: &str) -> i64 {
    sum_shortest_paths(input, 2)
}

pub fn part2(input: &str) -> i64 {
    sum_shortest_paths(input, 1000000)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        assert_eq!(part1(input), 374);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day11.txt");
        assert_eq!(part1(input), 9609130);
        assert_eq!(part2(input), 702152204842);
    }
}
