use crate::static_queue::StaticQueue;
use crate::static_vec::StaticVec;
use heapless::FnvIndexSet;

type Tile = u8;
type Pt2 = (i32, i32);
type Set<T> = FnvIndexSet<T, 16384>;
type Queue<T> = StaticQueue<T, 16384>;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

#[derive(Debug)]
struct Grid<'a> {
    data: &'a [u8],
    width: i32,
    height: i32,
}

impl Grid<'_> {
    fn at(&self, (row, col): Pt2) -> Option<Tile> {
        if row < 0 || row >= self.height || col < 0 || col >= self.width {
            None
        } else {
            Some(self.data[(row * (self.width + 1) + col) as usize])
        }
    }
}

fn tube_directions(from: Tile) -> StaticVec<Dir, 4> {
    match from {
        b'|' => [Dir::Above, Dir::Below].into_iter().collect(),
        b'-' => [Dir::Left, Dir::Right].into_iter().collect(),
        b'L' => [Dir::Above, Dir::Right].into_iter().collect(),
        b'J' => [Dir::Above, Dir::Left].into_iter().collect(),
        b'7' => [Dir::Left, Dir::Below].into_iter().collect(),
        b'F' => [Dir::Right, Dir::Below].into_iter().collect(),
        b'S' => [Dir::Left, Dir::Right, Dir::Above, Dir::Below].into_iter().collect(),
        _ => StaticVec::empty(),
    }
}

fn tube_neighbors(grid: &Grid, from: Pt2) -> StaticVec<Pt2, 4> {
    grid.at(from)
        .map(tube_directions)
        .into_iter()
        .flatten()
        .map(|dir| go(from, dir))
        .filter(|pt| grid.at(*pt).is_some())
        .collect()
}

fn tube_connections(grid: &Grid, from: Pt2) -> StaticVec<Pt2, 4> {
    tube_neighbors(grid, from)
        .into_iter()
        .filter(|nbr| tube_neighbors(grid, *nbr).contains(&from))
        .collect()
}

fn find(grid: &Grid, tile: Tile) -> Option<Pt2> {
    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid.at((row, col)) == Some(tile) {
                return Some((row, col));
            }
        }
    }
    None
}

fn find_loop(grid: &Grid, start: Pt2, v: &mut Set<Pt2>) {
    let mut q = Queue::new();
    q.push_back((start, 0));
    v.insert(start).unwrap();
    while let Some(front @ (cur, dist)) = q.pop_front() {
        let nbrs = tube_connections(grid, cur);
        for nbr in nbrs {
            if q.front() == Some(&front) {
                v.insert(nbr).unwrap();
                return;
            }
            if v.contains(&nbr) {
                continue;
            }
            v.insert(nbr).unwrap();
            q.push_back((nbr, dist + 1));
        }
    }
}

fn interior_neighbors(grid: &Grid, prev: Pt2, cur: Pt2) -> StaticVec<Pt2, 4> {
    use Dir::*;
    match grid.at(cur) {
        Some(b'F') if prev.1 > cur.1 => StaticVec::from([go(cur, Above), go(cur, Left)]),
        Some(b'J') if prev.1 < cur.1 => StaticVec::from([go(cur, Below), go(cur, Right)]),
        Some(b'7') if prev.0 > cur.0 => StaticVec::from([go(cur, Above), go(cur, Right)]),
        Some(b'L') if prev.0 < cur.0 => StaticVec::from([go(cur, Left), go(cur, Below)]),
        Some(b'|') if prev.0 < cur.0 => StaticVec::from([go(cur, Left)]),
        Some(b'|') if prev.0 > cur.0 => StaticVec::from([go(cur, Right)]),
        Some(b'-') if prev.1 < cur.1 => StaticVec::from([go(cur, Below)]),
        Some(b'-') if prev.1 > cur.1 => StaticVec::from([go(cur, Above)]),
        _ => StaticVec::empty(),
    }
}

fn explore(looop: &Set<Pt2>, from: Pt2, v: &mut Set<Pt2>) {
    for dir in [Dir::Left, Dir::Right, Dir::Above, Dir::Below] {
        let nbr = go(from, dir);
        if v.contains(&nbr) || looop.contains(&nbr) {
            continue;
        }
        v.insert(nbr).unwrap();
        explore(looop, nbr, v);
    }
}

fn interior_area(grid: &Grid, looop: &Set<Pt2>) -> i32 {
    let start = *looop.iter().min_by(|(r1, c1), (r2, c2)| r1.cmp(r2).then(c1.cmp(c2))).unwrap();
    let mut v = Set::new();
    let (mut prev, mut cur) = (start, start);
    while cur != start || prev == start {
        for pt in interior_neighbors(grid, prev, cur) {
            if !v.contains(&pt) && !looop.contains(&pt) {
                v.insert(pt).unwrap();
                explore(looop, pt, &mut v);
            }
        }
        let nbrs = tube_connections(grid, cur);
        let next = nbrs.into_iter().find(|nbr| *nbr != prev).unwrap();
        (prev, cur) = (cur, next);
    }
    v.len() as i32
}

fn parse(input: &str) -> Grid {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    Grid { data: input.as_bytes(), width, height }
}

pub fn part1(input: &str) -> i32 {
    let grid = parse(input);
    let start = find(&grid, b'S').unwrap();
    let mut looop = Set::new();
    find_loop(&grid, start, &mut looop);
    looop.len() as i32 / 2
}

pub fn part2(input: &str) -> i32 {
    let grid = parse(input);
    let start = find(&grid, b'S').unwrap();
    let mut looop = Set::new();
    find_loop(&grid, start, &mut looop);
    interior_area(&grid, &looop)
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
    }

    #[test]
    fn test_example3() {
        let input = "...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O.....
";
        assert_eq!(part2(input), 4);
    }

    #[test]
    fn test_example4() {
        let input = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
";
        assert_eq!(part2(input), 4);
    }

    #[test]
    fn test_example5() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        assert_eq!(part2(input), 8);
    }

    #[test]
    fn test_example6() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        assert_eq!(part2(input), 10);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day10.txt");
        assert_eq!(part1(input), 7102);
        assert_eq!(part2(input), 363);
    }
}
