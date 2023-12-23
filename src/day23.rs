use heapless::{Deque, FnvIndexMap, FnvIndexSet, Vec};
use libc_print::std_name::*;

type Pt = (i16, i16);
type Grid = Vec<Vec<Tile, 256>, 256>;
type Tile = u8;

fn start(grid: &Grid) -> Pt {
    let start = (0, 1);
    assert!(matches!(get(&grid, start), Some(b'.')));
    start
}

fn end(grid: &Grid) -> Pt {
    let end = (grid.len() as i16 - 1, grid.len() as i16 - 2);
    assert!(matches!(get(&grid, end), Some(b'.')));
    end
}

fn get(grid: &Grid, (r, c): Pt) -> Option<Tile> {
    if r >= 0 && (r as usize) < grid.len() && c >= 0 && (c as usize) < grid[0].len() {
        Some(grid[r as usize][c as usize])
    } else {
        None
    }
}

fn nbrs(grid: &Grid, pt @ (r, c): Pt) -> Vec<Pt, 4> {
    fn can_pass(tile: u8) -> bool {
        matches!(tile, b'.' | b'^' | b'v' | b'<' | b'>')
    }
    match get(grid, pt).unwrap() {
        b'.' => [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
            .into_iter()
            .filter(|pt| get(grid, *pt).map(can_pass).unwrap_or(false))
            .collect(),
        b'^' => Vec::from_slice(&[(r - 1, c)]).unwrap(),
        b'v' => Vec::from_slice(&[(r + 1, c)]).unwrap(),
        b'<' => Vec::from_slice(&[(r, c - 1)]).unwrap(),
        b'>' => Vec::from_slice(&[(r, c + 1)]).unwrap(),
        _ => Vec::new(),
    }
}

fn longest_path(grid: &Grid) -> usize {
    let mut frontier: Deque<(Pt, Pt, usize), 1024> = Deque::new();
    let end = end(grid);
    frontier.push_back((start(grid), (-1, -1), 0)).unwrap();
    let mut max_dist = 0;
    while let Some((cur, prev, dist)) = frontier.pop_back() {
        if cur == end {
            max_dist = max_dist.max(dist);
        }
        for nbr in nbrs(grid, cur).into_iter().filter(|nbr| *nbr != prev) {
            frontier.push_back((nbr, cur, dist + 1)).unwrap();
        }
    }
    max_dist
}

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.bytes().collect()).collect()
}

pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    longest_path(&grid)
}

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    const REAL_INPUT: &str = include_str!("../inputs/day23.txt");

    #[test]
    fn test_example() {
        assert_eq!(part1(TEST_INPUT), 94);
        assert_eq!(part2(TEST_INPUT), 0);
    }

    #[test]
    fn test_real() {
        assert_eq!(part1(REAL_INPUT), 2042);
        assert_eq!(part2(REAL_INPUT), 0);
    }
}
