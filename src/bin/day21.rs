use std::collections::HashSet;

type Pt = (i16, i16);
type Grid = Vec<Vec<Plot>>;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Plot {
    Garden,
    Rock,
}

fn get(grid: &Grid, (r, c): Pt) -> Plot {
    grid[r.rem_euclid(grid.len() as i16) as usize][c.rem_euclid(grid[0].len() as i16) as usize]
}

fn nbrs((r, c): Pt) -> [Pt; 4] {
    [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
}

fn tick(grid: &Grid, v: &mut HashSet<Pt>) {
    let mut next = HashSet::new();
    for pt in v.iter() {
        for nbr in nbrs(*pt).iter().filter(|nbr| get(grid, **nbr) != Plot::Rock) {
            next.insert(*nbr);
        }
    }
    *v = next;
}

fn explore(grid: &Grid, start: Pt, max: i16) -> i64 {
    let mut v = HashSet::new();
    v.insert(start);
    for _ in 0..max {
        tick(grid, &mut v);
    }
    v.len() as i64
}

fn parse(input: &str) -> (Grid, Pt) {
    let mut start = (0, 0);
    let mut grid = Grid::new();
    for (r, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (c, b) in line.bytes().enumerate() {
            row.push(match b {
                b'.' => Plot::Garden,
                b'#' => Plot::Rock,
                b'S' => {
                    start = (r as i16, c as i16);
                    Plot::Garden
                }
                _ => unreachable!(),
            });
        }
        grid.push(row);
    }
    (grid, start)
}

pub fn part1(input: &str) -> i64 {
    let (grid, start) = parse(input);
    explore(&grid, start, 64)
}

pub fn part2(input: &str) -> i64 {
    let (grid, start) = parse(input);

    // the row and column of the start is empty:
    assert!(grid[start.0 as usize].iter().all(|plot| *plot == Plot::Garden));
    assert!(grid.iter().all(|row| row[start.1 as usize] == Plot::Garden));

    // printing the grid at each step shows that it follows a diamond
    // pattern. a diamond is a fancy square, so going twice as far will
    // produce roughly 4x as many points. fit a quadratic equation:

    // find the points to interpolate
    let (y1, y2, y3) = (
        explore(&grid, start, 65),
        explore(&grid, start, 65 + 131),
        explore(&grid, start, 65 + 131 * 2),
    );

    // https://www.geeksforgeeks.org/lagrange-interpolation-formula/
    let a = y1 / 2 - y2 + y3 / 2;
    let b = -3 * (y1 / 2) + 2 * y2 - y3 / 2;
    let c = y1;
    let x = (26501365 - 65) / 131;
    let y = a * x * x + b * x + c;

    // shout out to reddit
    y
}

fn main() {
    let input = include_str!("../../inputs/day21.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}
