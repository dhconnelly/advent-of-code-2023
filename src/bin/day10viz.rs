use advent_of_code_2023::static_queue::StaticQueue;
use advent_of_code_2023::static_vec::StaticVec;
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
        if row < 0 || row >= self.height {
            None
        } else if col < 0 || col >= self.width {
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
        let nbrs = tube_connections(&grid, cur);
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

fn explore(grid: &Grid, looop: &Set<Pt2>, from: Pt2, v: &mut Set<Pt2>) {
    for dir in [Dir::Left, Dir::Right, Dir::Above, Dir::Below] {
        let nbr = go(from, dir);
        if v.contains(&nbr) || looop.contains(&nbr) {
            continue;
        }
        v.insert(nbr).unwrap();
        explore(grid, looop, nbr, v);
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
                explore(grid, looop, pt, &mut v);
            }
        }
        let nbrs = tube_connections(grid, cur);
        let next = nbrs.into_iter().filter(|nbr| *nbr != prev).next().unwrap();
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

fn main() {
    let path = std::env::args().skip(1).next().unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    println!("{}", part1(&text));
    println!("{}", part2(&text));
}
