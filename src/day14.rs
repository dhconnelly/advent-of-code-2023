use crate::static_vec::StaticVec;
use heapless::FnvIndexMap;

type Grid = StaticVec<StaticVec<Tile, 128>, 128>;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
enum Tile {
    #[default]
    Empty,
    Round,
    Cube,
}

impl Tile {
    fn load(&self) -> usize {
        match self {
            Tile::Round => 1,
            _ => 0,
        }
    }
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'#' => Self::Cube,
            b'O' => Self::Round,
            _ => panic!("invalid tile"),
        }
    }
}

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.bytes().map(Tile::from).collect()).collect()
}

fn roll_north(grid: &mut Grid) {
    for col in 0..grid[0].len() {
        let mut avail = 0;
        for row in 0..grid.len() {
            match grid[row][col] {
                Tile::Round if avail == row => avail = row + 1,
                Tile::Cube => avail = row + 1,
                Tile::Round => {
                    (grid[avail][col], grid[row][col]) = (Tile::Round, Tile::Empty);
                    avail += 1;
                }
                Tile::Empty => {}
            }
        }
    }
}

fn roll_south(grid: &mut Grid) {
    for col in 0..grid[0].len() {
        let mut avail = grid.len();
        for row in (0..grid.len()).rev() {
            match grid[row][col] {
                Tile::Round if avail - 1 == row => avail = row,
                Tile::Cube => avail = row,
                Tile::Round => {
                    (grid[avail - 1][col], grid[row][col]) = (Tile::Round, Tile::Empty);
                    avail -= 1;
                }
                Tile::Empty => {}
            }
        }
    }
}

fn roll_west(grid: &mut Grid) {
    for row in 0..grid.len() {
        let mut avail = 0;
        for col in 0..grid[0].len() {
            match grid[row][col] {
                Tile::Round if avail == col => avail = col + 1,
                Tile::Cube => avail = col + 1,
                Tile::Round => {
                    (grid[row][avail], grid[row][col]) = (Tile::Round, Tile::Empty);
                    avail += 1;
                }
                Tile::Empty => {}
            }
        }
    }
}

fn roll_east(grid: &mut Grid) {
    for row in 0..grid.len() {
        let mut avail = grid[0].len();
        for col in (0..grid[0].len()).rev() {
            match grid[row][col] {
                Tile::Round if avail - 1 == col => avail = col,
                Tile::Cube => avail = col,
                Tile::Round => {
                    (grid[row][avail - 1], grid[row][col]) = (Tile::Round, Tile::Empty);
                    avail -= 1;
                }
                Tile::Empty => {}
            }
        }
    }
}

fn cycle(grid: &mut Grid) {
    roll_north(grid);
    roll_west(grid);
    roll_south(grid);
    roll_east(grid);
}

fn total_load(grid: &Grid) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, row)| row.iter().map(move |tile| (grid.len() - i) * tile.load()))
        .flatten()
        .sum()
}

pub fn part1(input: &str) -> usize {
    let mut grid = parse(input);
    roll_north(&mut grid);
    total_load(&grid)
}

static mut CACHE: FnvIndexMap<Grid, usize, 1024> = FnvIndexMap::new();

fn cache_get(grid: &Grid) -> Option<usize> {
    unsafe { CACHE.get(grid).copied() }
}

fn cache_set(grid: &Grid, i: usize) {
    unsafe {
        CACHE.insert(grid.clone(), i).unwrap();
    }
}

pub fn part2(input: &str) -> usize {
    let original = parse(input);
    let mut grid = original.clone();
    let (mut first, mut second) = (None, None);
    for i in 0..1000000000 {
        if let Some(j) = cache_get(&grid) {
            (first, second) = (Some(j), Some(i));
            break;
        } else {
            cache_set(&grid, i);
        }
        cycle(&mut grid);
    }

    let (first, second) = (first.unwrap(), second.unwrap());
    let mut grid = original.clone();
    for _ in 0..first {
        cycle(&mut grid);
    }
    let repeats = (1000000000 - first) / (second - first);
    let remaining = 1000000000 % repeats - first;
    for _ in 0..remaining {
        cycle(&mut grid);
    }
    total_load(&grid)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        assert_eq!(part1(input), 136);
        assert_eq!(part2(input), 64);
    }

    #[test]
    fn test_cycle() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        let mut grid = parse(input);
        cycle(&mut grid);
        assert_eq!(
            parse(
                ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
"
            ),
            grid
        );
        cycle(&mut grid);
        assert_eq!(
            parse(
                ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
"
            ),
            grid
        );
        cycle(&mut grid);
        assert_eq!(
            parse(
                ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
"
            ),
            grid
        );
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day14.txt");
        assert_eq!(part1(input), 109638);
        assert_eq!(part2(input), 102657);
    }
}