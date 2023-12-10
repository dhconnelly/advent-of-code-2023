use core::cmp::Ordering;

use libc_print::std_name::*;

use crate::static_vec::StaticVec;

type Pt2 = (i32, i32);

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

fn dirs_from(from: Tile) -> StaticVec<Dir, 4> {
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

fn find_loop(grid: &Grid, start: Pt2) -> Option<Set> {
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
                v.push(nbr);
                return Some(v);
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
    let looop = find_loop(&grid, start).unwrap();
    looop.len() as i32 / 2
}

fn explore(grid: &Grid, looop: &Set, from: Pt2, v: &mut Set) -> i32 {
    let mut area = 1;
    for dir in [Dir::Left, Dir::Right, Dir::Above, Dir::Below] {
        let nbr = go(from, dir);
        if v.contains(&nbr) {
            continue;
        }
        if looop.contains(&nbr) {
            continue;
        }
        v.push(nbr);
        area += explore(grid, looop, nbr, v);
    }
    area
}

fn infer(grid: &Grid, pt: Pt2) -> Option<Tile> {
    match grid.at(pt) {
        Some(b'S') => {
            let dirs = connections(grid, pt);
            if dirs.contains(&go(pt, Dir::Above)) && dirs.contains(&go(pt, Dir::Below)) {
                Some(b'|')
            } else if dirs.contains(&go(pt, Dir::Left))
                && dirs.contains(&go(pt, Dir::Right))
            {
                Some(b'-')
            } else if dirs.contains(&go(pt, Dir::Below))
                && dirs.contains(&go(pt, Dir::Right))
            {
                Some(b'F')
            } else if dirs.contains(&go(pt, Dir::Below))
                && dirs.contains(&go(pt, Dir::Left))
            {
                Some(b'7')
            } else if dirs.contains(&go(pt, Dir::Above))
                && dirs.contains(&go(pt, Dir::Left))
            {
                Some(b'J')
            } else {
                assert!(
                    dirs.contains(&go(pt, Dir::Above))
                        && dirs.contains(&go(pt, Dir::Right))
                );
                Some(b'L')
            }
        }
        tile => tile,
    }
}

fn interior_area(grid: &Grid, looop: &Set) -> i32 {
    let start = *looop
        .iter()
        .min_by(|(row1, col1), (row2, col2)| {
            let dr = row1.cmp(row2);
            if dr == Ordering::Equal {
                col1.cmp(col2)
            } else {
                dr
            }
        })
        .unwrap();

    let mut area = 0;
    let mut v = Set::empty();

    /*
    for row in 0..grid.height {
        for col in 0..grid.width {
            print!(
                "{}",
                if looop.contains(&(row, col)) {
                    grid.at((row, col)).unwrap() as char
                } else {
                    '.'
                }
            );
        }
        println!();
    }
    println!();
    */

    let (mut prev, mut cur) = (start, start);
    while cur != start || prev == start {
        let mut insides = StaticVec::<Pt2, 4>::empty();
        let tile = infer(grid, cur);
        if let Some(b'S') = grid.at(cur) {
            println!("S => {:?}", tile.unwrap() as char);
        }
        match tile {
            Some(b'F') => {
                if prev.1 > cur.1 {
                    insides.push(go(cur, Dir::Above));
                    insides.push(go(cur, Dir::Left));
                }
            }
            Some(b'J') => {
                if prev.1 < cur.1 {
                    insides.push(go(cur, Dir::Below));
                    insides.push(go(cur, Dir::Right));
                }
            }
            Some(b'7') => {
                if prev.0 > cur.0 {
                    insides.push(go(cur, Dir::Above));
                    insides.push(go(cur, Dir::Right));
                }
            }
            Some(b'L') => {
                if prev.0 < cur.0 {
                    insides.push(go(cur, Dir::Left));
                    insides.push(go(cur, Dir::Below));
                }
            }
            Some(b'|') => {
                if prev.0 < cur.0 {
                    insides.push(go(cur, Dir::Left));
                } else if prev.0 > cur.0 {
                    insides.push(go(cur, Dir::Right));
                }
            }
            Some(b'-') => {
                if prev.1 < cur.1 {
                    insides.push(go(cur, Dir::Below));
                } else if prev.1 > cur.1 {
                    insides.push(go(cur, Dir::Above));
                }
            }
            _ => {}
        };
        for pt in insides {
            if !v.contains(&pt) && !looop.contains(&pt) {
                v.push(pt);
                area += explore(grid, looop, pt, &mut v);
            }
        }

        let next =
            connections(grid, cur).into_iter().filter(|nbr| *nbr != prev).next().unwrap();
        (prev, cur) = (cur, next);
    }

    /*
    for row in 0..grid.height {
        for col in 0..grid.width {
            print!("{}", if v.contains(&(row, col)) { 'X' } else { '.' });
        }
        println!();
    }
    println!();
    */

    assert_eq!(area as usize, v.len());
    area
}

pub fn part2(input: &str) -> i32 {
    let grid = parse(input);
    let start = find_start(&grid).unwrap();
    let looop = find_loop(&grid, start).unwrap();
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
