use heapless::{Deque, FnvIndexSet, Vec};
use libc_print::std_name::*;

type Pt = (i64, i64);
type Grid = Vec<Vec<Plot, 256>, 256>;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Plot {
    Garden,
    Rock,
}

fn get(grid: &Grid, (r, c): Pt) -> Plot {
    grid[r.rem_euclid(grid.len() as i64) as usize][c.rem_euclid(grid[0].len() as i64) as usize]
}

fn nbrs((r, c): Pt, grid: &Grid) -> Vec<Pt, 4> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(dr, dc)| (r + dr, c + dc))
        .filter(|(r, _)| *r >= 0 && *r < grid.len() as i64)
        .filter(|(_, c)| *c >= 0 && *c < grid[0].len() as i64)
        .collect()
}

fn explore(grid: &Grid, start: Pt, max: u8) -> usize {
    let mut q: Deque<(Pt, u8), 4096> = Deque::new();
    let mut v: FnvIndexSet<Pt, 4096> = FnvIndexSet::new();
    q.push_back((start, 0)).unwrap();
    while let Some((pt, dist)) = q.pop_front() {
        if dist == max {
            break;
        }
        v.remove(&pt);
        for nbr in nbrs(pt, grid) {
            if !v.contains(&nbr) && get(grid, nbr) == Plot::Garden {
                q.push_back((nbr, dist + 1)).unwrap();
                v.insert(nbr).unwrap();
            }
        }
    }
    v.len()
}

fn parse(input: &str) -> (Grid, Pt) {
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
                        start = (r as i64, c as i64);
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

pub fn part2(input: &str) -> i64 {
    0
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
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day21.txt");
        assert_eq!(part1(input), 3660);
        assert_eq!(part2(input), 0);
    }
}
