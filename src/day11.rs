use heapless::{Deque, FnvIndexMap, FnvIndexSet};
use libc_print::std_name::*;

use crate::static_vec::StaticVec;

type Pt2 = (u16, u16);

#[derive(Clone, Copy, PartialEq, Default, Debug)]
enum Tile {
    #[default]
    Empty,
    Galaxy,
}

type Grid<'a> = StaticVec<StaticVec<Tile, 256>, 256>;

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

fn expand(grid: &mut Grid) {
    let mut i = 0;
    while i < grid.len() {
        if grid[i].iter().all(|t| *t == Tile::Empty) {
            grid.insert(i, grid[i]);
            i += 1;
        }
        i += 1;
    }
    let mut j = 0;
    while j < grid[0].len() {
        if (0..grid.len()).all(|i| grid[i][j] == Tile::Empty) {
            for i in 0..grid.len() {
                let t = grid[i][j];
                grid[i].insert(j, t);
            }
            j += 1;
        }
        j += 1;
    }
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

fn shortest_paths(grid: &Grid, from: Pt2) -> FnvIndexMap<Pt2, i32, 4096> {
    let mut q = Deque::<(Pt2, i32), 4096>::new();
    let mut v = FnvIndexSet::<Pt2, 32768>::new();
    let mut dists = FnvIndexMap::new();
    q.push_front((from, 0)).unwrap();
    v.insert(from).unwrap();
    while let Some((cur, dist)) = q.pop_front() {
        for nbr in neighbors(grid, cur) {
            if v.contains(&nbr) {
                continue;
            }
            let (row, col) = (nbr.0 as usize, nbr.1 as usize);
            if grid[row][col] == Tile::Galaxy {
                dists.insert(nbr, dist + 1).unwrap();
            }
            v.insert(nbr).unwrap();
            q.push_back((nbr, dist + 1)).unwrap();
        }
    }
    dists
}

pub fn part1(input: &str) -> i32 {
    let mut grid = Grid::empty();
    parse(input, &mut grid);
    expand(&mut grid);
    let mut sum = 0;
    let mut galaxies = StaticVec::<Pt2, 4096>::empty();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == Tile::Galaxy {
                let from = (i as u16, j as u16);
                galaxies.push(from);
            }
        }
    }
    for i in 0..galaxies.len() - 1 {
        let from = galaxies[i];
        let dists = shortest_paths(&grid, from);
        for j in i + 1..galaxies.len() {
            let to = galaxies[j];
            sum += dists[&to];
        }
    }
    sum
}

pub fn part2(input: &str) -> i64 {
    0
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
        assert_eq!(part1(input), 0);
    }
}
