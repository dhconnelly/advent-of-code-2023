use crate::{static_map::StaticSet, static_queue::StaticQueue, static_vec::StaticVec};

type Grid = StaticVec<StaticVec<Tile, 128>, 128>;
type Queue<T> = StaticQueue<T, 32768>;
type Set<T> = StaticSet<T, 128, 128>;
type Pt = (i8, i8);

#[derive(Clone, Copy, Default)]
enum Tile {
    #[default]
    Empty,
    MirrorUp,
    MirrorDown,
    SplitUpDown,
    SplitLeftRight,
}

fn parse(input: &str) -> Grid {
    let tile = |b| match b {
        b'.' => Tile::Empty,
        b'/' => Tile::MirrorUp,
        b'\\' => Tile::MirrorDown,
        b'|' => Tile::SplitUpDown,
        b'-' => Tile::SplitLeftRight,
        _ => panic!("invalid tile"),
    };
    input.lines().map(|line| line.bytes().map(tile).collect()).collect()
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    #[default]
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn reflect_up(self) -> Dir {
        match self {
            Dir::Right => Dir::Up,
            Dir::Up => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Left,
        }
    }

    fn reflect_down(self) -> Dir {
        match self {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Left,
        }
    }

    fn apply(self, (r, c): Pt) -> (Pt, Dir) {
        match self {
            Dir::Right => ((r, c + 1), self),
            Dir::Down => ((r + 1, c), self),
            Dir::Left => ((r, c - 1), self),
            Dir::Up => ((r - 1, c), self),
        }
    }
}

fn advance(grid: &Grid, pt @ (r, c): Pt, dir: Dir) -> StaticVec<(Pt, Dir), 2> {
    use Dir::*;
    use Tile::*;
    let tile = grid[r as usize][c as usize];
    match (dir, tile) {
        (Up | Down, SplitUpDown) | (Left | Right, SplitLeftRight) | (_, Empty) => {
            StaticVec::from([dir.apply(pt)])
        }
        (_, MirrorUp) => StaticVec::from([dir.reflect_up().apply(pt)]),
        (_, MirrorDown) => StaticVec::from([dir.reflect_down().apply(pt)]),
        (Left | Right, SplitUpDown) => StaticVec::from([Up.apply(pt), Down.apply(pt)]),
        (Up | Down, SplitLeftRight) => StaticVec::from([Left.apply(pt), Right.apply(pt)]),
    }
}

fn in_grid(grid: &Grid, (r, c): Pt) -> bool {
    r >= 0 && r < grid.len() as i8 && c >= 0 && c < grid[r as usize].len() as i8
}

fn explore(grid: &Grid, energized: &mut Set<Pt>) {
    let mut q = Queue::new();
    let mut v: Set<(Pt, Dir)> = Set::new();
    q.push_back(((0, 0), Dir::Right));
    v.insert(((0, 0), Dir::Right));
    energized.insert((0, 0));
    while let Some((pt, dir)) = q.pop_front() {
        for next @ (nbr, _) in advance(grid, pt, dir) {
            if v.contains(&next) || !in_grid(grid, nbr) {
                continue;
            }
            energized.insert(nbr);
            v.insert(next);
            q.push_back(next);
        }
    }
}

pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut energized = Set::new();
    explore(&grid, &mut energized);
    energized.len()
}

pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";
        assert_eq!(part1(input), 46);
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day16.txt");
        assert_eq!(part1(input), 0);
        assert_eq!(part2(input), 0);
    }
}
