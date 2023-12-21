use heapless::{Deque, FnvIndexSet, Vec};
use libc_print::std_name::*;

type Pt = (i16, i16);
type Grid<T> = Vec<Vec<T, 256>, 256>;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Plot {
    Garden,
    Rock,
}

fn get<T: Copy>(grid: &Grid<T>, (mut r, mut c): Pt) -> T {
    (r, c) = (r.rem_euclid(grid.len() as i16), c.rem_euclid(grid[0].len() as i16));
    grid[r as usize][c as usize]
}

fn nbrs((r, c): Pt) -> [Pt; 4] {
    [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
}

fn empty_dists(width: usize, height: usize) -> Grid<i16> {
    let empty = Vec::from_iter(core::iter::repeat(-1).take(width));
    Grid::from_iter(core::iter::repeat(empty).take(height))
}

fn in_bounds<T>(grid: &Grid<T>, (r, c): Pt) -> bool {
    r >= 0 && r < grid.len() as i16 && c >= 0 && c < grid[0].len() as i16
}

fn dists(grid: &Grid<Plot>, start: Pt) -> Grid<i16> {
    let mut d: Grid<i16> = empty_dists(grid[0].len(), grid.len());
    let mut q: Deque<(Pt, i16), 1024> = Deque::new();
    q.push_back((start, 0)).unwrap();
    d[start.0 as usize][start.1 as usize] = 0;
    while let Some((pt, dist)) = q.pop_front() {
        for nbr @ (r, c) in nbrs(pt) {
            if in_bounds(grid, nbr) && get(&d, nbr) == -1 && get(grid, nbr) == Plot::Garden {
                q.push_back((nbr, dist + 1)).unwrap();
                d[r as usize][c as usize] = dist + 1;
            }
        }
    }
    d
}

fn dist(grid: &Grid<Plot>, from: Pt, to: Pt) -> i16 {
    let mut q: Deque<(Pt, i16), 1024> = Deque::new();
    let mut v: FnvIndexSet<Pt, 1024> = FnvIndexSet::new();
    q.push_back((from, 0)).unwrap();
    v.insert(from).unwrap();
    while let Some((pt, dist)) = q.pop_front() {
        for nbr in nbrs(pt) {
            if nbr == to {
                return dist + 1;
            }
            if !v.contains(&nbr) && get(grid, nbr) == Plot::Garden {
                q.push_back((nbr, dist + 1)).unwrap();
                v.insert(nbr).unwrap();
            }
        }
    }
    unreachable!()
}

fn explore(grid: &Grid<Plot>, start: Pt, max: i32) -> usize {
    let d = dists(grid, start);
    let max_dist = d.iter().map(|line| line.iter()).flatten().max().unwrap();
    let self_dists: Vec<_, 4> = [
        (0, grid.len() as i16),
        (0, -(grid.len() as i16)),
        (grid.len() as i16, 0),
        (-(grid.len() as i16), 0),
    ]
    .into_iter()
    .map(|(dr, dc)| dist(grid, start, (start.0 + dr, start.1 + dc)))
    .collect();
    println!("max_dist = {}, self_dist = {:?}", max_dist, self_dists);
    0
}

fn parse(input: &str) -> (Grid<Plot>, Pt) {
    let mut start = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.bytes()
                .enumerate()
                .map(|(c, b)| match b {
                    b'.' => Plot::Garden,
                    b'#' => Plot::Rock,
                    b'S' => {
                        start = (r as i16, c as i16);
                        Plot::Garden
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (grid, start)
}

pub fn part1(input: &str) -> usize {
    let (grid, start) = parse(input);
    explore(&grid, start, 64)
}

pub fn part2(input: &str) -> usize {
    let (grid, start) = parse(input);
    explore(&grid, start, 26501365)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";
        let (grid, start) = parse(input);
        assert_eq!(explore(&grid, start, 6), 16);
        assert_eq!(explore(&grid, start, 10), 50);
        assert_eq!(explore(&grid, start, 50), 1594);
        //assert_eq!(explore(&grid, start, 100), 6536);
        //assert_eq!(explore(&grid, start, 500), 167004);
        //assert_eq!(explore(&grid, start, 1000), 668697);
        //assert_eq!(explore(&grid, start, 5000), 16733044);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day21.txt");
        assert_eq!(part1(input), 3660);
        assert_eq!(part2(input), 0);
    }
}
