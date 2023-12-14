use crate::static_vec::StaticVec;
use libc_print::std_name::*;

#[derive(Clone, Copy, Default, PartialEq)]
enum Tile {
    #[default]
    Empty,
    Round,
    Cube,
}

impl core::fmt::Debug for Tile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::Round => 'O',
            Tile::Cube => '#',
        };
        write!(f, "{}", c)
    }
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

type Grid = StaticVec<StaticVec<Tile, 128>, 128>;

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

fn roll_west(grid: &mut Grid) {
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

fn roll_east(grid: &mut Grid) {
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

fn cycle(grid: &mut Grid) {
    roll_north(grid);
    roll_west(grid);
    roll_south(grid);
    roll_east(grid);
}

fn print_grid(grid: &Grid) {
    for row in grid.iter() {
        println!("{:?}", row);
    }
    println!();
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

pub fn part2(input: &str) -> i64 {
    0
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
        assert_eq!(part2(input), 0);
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
        assert_eq!(part1(input), 0);
        assert_eq!(part2(input), 0);
    }
}
