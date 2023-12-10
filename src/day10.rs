use crate::static_vec::StaticVec;

type Pt2 = (i32, i32);

#[derive(Clone, Copy, Debug, Default)]
enum Dir {
    #[default]
    Above,
    Below,
    Left,
    Right,
}

fn go((row, col): Pt2, dir: Dir) -> Pt2 {
    match dir {
        Dir::Left => (row, col - 1),
        Dir::Right => (row, col + 1),
        Dir::Above => (row - 1, col),
        Dir::Below => (row + 1, col),
    }
}

fn dirs_from(from: Tile) -> StaticVec<Dir, 4> {
    match from {
        b'|' => [Dir::Above, Dir::Below].into_iter().collect(),
        b'-' => [Dir::Left, Dir::Right].into_iter().collect(),
        b'L' => [Dir::Above, Dir::Right].into_iter().collect(),
        b'J' => [Dir::Above, Dir::Left].into_iter().collect(),
        b'7' => [Dir::Left, Dir::Below].into_iter().collect(),
        b'F' => [Dir::Right, Dir::Below].into_iter().collect(),
        b'.' => StaticVec::empty(),
        b'S' => [Dir::Left, Dir::Right, Dir::Above, Dir::Below].into_iter().collect(),
        _ => panic!("invalid tile"),
    }
}

type Tile = u8;

#[derive(Debug)]
struct Grid<'a> {
    data: &'a [u8],
    width: i32,
    height: i32,
}

impl Grid<'_> {
    fn at(&self, (row, col): Pt2) -> Option<Tile> {
        if row < 0 || row >= self.height {
            None
        } else if col < 0 || col >= self.width {
            None
        } else {
            Some(self.data[(row * (self.width + 1) + col) as usize])
        }
    }

    fn neighbors(&self, from: Pt2) -> StaticVec<Pt2, 4> {
        let tile = match self.at(from) {
            None => return StaticVec::empty(),
            Some(tile) => tile,
        };
        dirs_from(tile)
            .into_iter()
            .map(|dir| go(from, dir))
            .filter(|pt| self.at(*pt).is_some())
            .collect()
    }
}

fn parse(input: &str) -> Grid {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    Grid { data: input.as_bytes(), width, height }
}

fn connections(grid: &Grid, from: Pt2) -> StaticVec<Pt2, 4> {
    grid.neighbors(from)
        .into_iter()
        .filter(|nbr| grid.neighbors(*nbr).contains(&from))
        .collect()
}

fn find_start(grid: &Grid) -> Option<Pt2> {
    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid.at((row, col)) == Some(b'S') {
                return Some((row, col));
            }
        }
    }
    None
}

// TODO: make a faster set
type Set = StaticVec<Pt2, 32768>;
// TODO: add a range-contains
type Queue = StaticVec<(Pt2, i32), 32768>;

fn loop_max(grid: &Grid, start: Pt2) -> Option<i32> {
    let mut q = Queue::empty();
    let mut v = Set::empty();
    q.push((start, 0));
    v.push(start);
    let mut qi = 0;
    while qi < q.len() {
        let (cur, dist) = q[qi];
        qi += 1;
        let nbrs = connections(&grid, cur);
        for nbr in nbrs {
            if q.contains(&(nbr, dist + 1)) {
                return Some(dist + 1);
            }
            if v.contains(&nbr) {
                continue;
            }
            v.push(nbr);
            q.push((nbr, dist + 1));
        }
    }
    None
}

pub fn part1(input: &str) -> i32 {
    let grid = parse(input);
    let start = find_start(&grid).unwrap();
    loop_max(&grid, start).unwrap()
}

pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
        assert_eq!(part1(input), 4);
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn test_example2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
        assert_eq!(part1(input), 8);
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day10.txt");
        assert_eq!(part1(input), 0);
        assert_eq!(part2(input), 0);
    }
}
