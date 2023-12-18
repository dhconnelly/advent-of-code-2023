use core::iter::Peekable;
use heapless::{FnvIndexMap, Vec};
use libc_print::std_name::*;

type Color = u32;
type Pt = (i16, i16);
const START: Pt = (0, 0);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty,
    Dug,
    Trench(Color),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn vec(self) -> Pt {
        match self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        }
    }
}

fn add((r1, c1): Pt, (r2, c2): Pt) -> Pt {
    (r1 + r2, c1 + c2)
}

fn sub((r1, c1): Pt, (r2, c2): Pt) -> Pt {
    (r1 - r2, c1 - c2)
}

fn times((r, c): Pt, n: i16) -> Pt {
    (r * n, c * n)
}

struct Command {
    dir: Dir,
    dist: i16,
    col: Color,
}

fn parse(input: &str) -> impl Iterator<Item = Command> + '_ {
    input.lines().map(|line: &str| {
        let mut toks = line.split(' ');
        let dir = match toks.next().unwrap() {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!("invalid direction"),
        };
        let dist = toks.next().unwrap().parse().unwrap();
        let col = toks.next().unwrap();
        let col = u32::from_str_radix(&col[2..col.len() - 1], 16).unwrap();
        Command { dir, dist, col }
    })
}

fn length<'a>(trench: impl Iterator<Item = &'a Command>) -> usize {
    trench.map(|Command { dist, .. }| *dist as usize).sum()
}

fn interior(cmds: &[Command]) -> usize {
    use Dir::*;
    let mut rows: FnvIndexMap<i16, Vec<i16, 32>, 1024> = FnvIndexMap::new();
    let mut push = |(row, col): Pt| {
        if let Some(cols) = rows.get_mut(&row) {
            cols.push(col).unwrap();
        } else {
            rows.insert(row, [col].into_iter().collect()).unwrap();
        }
    };

    let (mut prev, mut cur) = (sub(START, cmds[cmds.len() - 1].dir.vec()), START);
    for cmd in cmds {
        for _ in 0..cmd.dist {
            let next = add(cur, cmd.dir.vec());

            // vertical, not at a corner: add column
            match cmd.dir {
                Up | Down if cur.0 != prev.0 && cur.0 != next.0 => push(cur),
                Down if cur.1 < prev.1 => push(cur),
                Right if cur.0 > prev.0 => push(cur),
                Left if cur.0 < prev.0 => push(cur),
                Up if cur.1 > prev.1 => push(cur),
                _ => (),
            }

            (prev, cur) = (cur, next);
        }
    }

    let mut area = 0;
    for row in rows.values_mut() {
        row.sort();
        let mut i = 0;
        while i < row.len() - 1 {
            let (a, b) = (row[i], row[i + 1]);
            area += (b - a - 1) as usize;
            i += 2;
        }
    }

    area
}

pub fn part1(input: &str) -> usize {
    let commands: Vec<Command, 1024> = parse(input).collect();
    interior(&commands) + length(commands.iter())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
        assert_eq!(part1(input), 62);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day18.txt");
        assert_eq!(part1(input), 40761);
    }
}
