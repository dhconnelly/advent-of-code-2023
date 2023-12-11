use crate::static_vec::StaticVec;
use heapless::{
    binary_heap::{BinaryHeap, Min},
    FnvIndexMap, FnvIndexSet,
};

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

fn neighbors(grid: &Grid, (row, col): Pt2) -> StaticVec<Pt2, 4> {
    let mut nbrs = StaticVec::empty();
    if row > 0 {
        nbrs.push((row - 1, col));
    }
    if row < grid.len() as u16 - 1 {
        nbrs.push((row + 1, col));
    }
    if col > 0 {
        nbrs.push((row, col - 1));
    }
    if col < grid[row as usize].len() as u16 - 1 {
        nbrs.push((row, col + 1));
    }
    nbrs
}

fn step_dist(
    (row_weights, col_weights): &Weights,
    (from_row, from_col): Pt2,
    (to_row, _): Pt2,
) -> i64 {
    if from_row == to_row {
        col_weights[from_col as usize]
    } else {
        row_weights[from_row as usize]
    }
}

fn shortest_paths(
    grid: &Grid,
    weights: &Weights,
    from: Pt2,
) -> FnvIndexMap<Pt2, i64, 4096> {
    let mut q = BinaryHeap::<(i64, Pt2), Min, 4096>::new();
    let mut v = FnvIndexSet::<Pt2, 32768>::new();
    let mut dists = FnvIndexMap::new();
    q.push((0, from)).unwrap();
    v.insert(from).unwrap();
    while let Some((dist, cur)) = q.pop() {
        for nbr in neighbors(grid, cur) {
            if v.contains(&nbr) {
                continue;
            }
            let (row, col) = (nbr.0 as usize, nbr.1 as usize);
            let nbr_dist = dist + step_dist(weights, cur, nbr);
            if grid[row][col] == Tile::Galaxy {
                dists.insert(nbr, nbr_dist).unwrap();
            }
            v.insert(nbr).unwrap();
            q.push((nbr_dist, nbr)).unwrap();
        }
    }
    dists
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
        let dists = shortest_paths(&grid, &weights, from);
        for j in i + 1..galaxies.len() {
            let to = galaxies[j];
            sum += dists[&to];
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
