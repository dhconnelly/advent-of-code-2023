use libc_print::std_name::*;

type Vec<T> = heapless::Vec<T, 1024>;
type Range = (i64, i64);
type Pt = (i64, i64);
const START: Pt = (0, 0);

#[derive(Clone, Debug)]
enum Intersection {
    None,
    Contained,
    Split2 { intersection: Range, remaining: Range },
    Split3 { intersection: Range, remaining: (Range, Range) },
}

fn intersect(of: Range, with: Range) -> Intersection {
    if of.1 < with.0 || of.0 > with.1 {
        Intersection::None
    } else if of.0 >= with.0 && of.1 <= with.1 {
        Intersection::Contained
    } else if of.0 < with.0 && of.1 > with.1 {
        Intersection::Split3 {
            intersection: with,
            remaining: ((of.0, with.0 - 1), (with.1 + 1, of.1)),
        }
    } else {
        let intersection = (of.0.max(with.0), of.1.min(with.1));
        let remaining = if of.0 < intersection.0 {
            (of.0, intersection.0 - 1)
        } else {
            (intersection.1 + 1, of.1)
        };
        Intersection::Split2 { intersection, remaining }
    }
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

fn times((r, c): Pt, n: i64) -> Pt {
    (r * n, c * n)
}

#[derive(Clone, Debug)]
struct Command {
    dir: Dir,
    dist: i64,
}

fn parse(input: &str) -> impl Iterator<Item = (Command, Command)> + '_ {
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
        let cmd1 = Command { dir, dist };

        let hex = toks.next().unwrap();
        let dist = i64::from_str_radix(&hex[2..7], 16).unwrap();
        let dir = match hex.as_bytes()[7] {
            b'0' => Dir::Right,
            b'1' => Dir::Down,
            b'2' => Dir::Left,
            b'3' => Dir::Up,
            _ => panic!("invalid direction"),
        };
        let cmd2 = Command { dir, dist };

        (cmd1, cmd2)
    })
}

fn length<'a>(trench: impl Iterator<Item = &'a Command>) -> i64 {
    trench.map(|Command { dist, .. }| *dist).sum()
}

fn interior(cmds: &[Command]) -> i64 {
    let mut lrs: Vec<(i64, Range)> = Vec::new();
    let mut rls: Vec<(i64, Range)> = Vec::new();

    let mut cur = START;
    for cmd in cmds {
        let next = add(cur, times(cmd.dir.vec(), cmd.dist));
        match cmd.dir {
            Dir::Right => lrs.push((cur.0, (cur.1, next.1))).unwrap(),
            Dir::Left => rls.push((cur.0, (next.1, cur.1))).unwrap(),
            _ => (),
        }
        cur = next;
    }

    lrs.sort();
    rls.sort();

    let mut area = 0;
    let mut i = 0;
    while i < lrs.len() {
        let row1 = lrs[i].0;
        let lr = lrs[i].1;
        let prev_area = area;
        let mut found = false;
        for (row2, rl) in rls.iter().filter(|(row2, _)| row2 > &row1) {
            match intersect(lr, *rl) {
                Intersection::None => continue,
                Intersection::Contained => {
                    println!("{} {} {:?} {:?} {:?}", row1, row2, lr, rl, intersect(lr, *rl));
                    area += (lr.1 - lr.0 + 1) * (row2 - row1 + 1);
                    found = true;
                    break;
                }
                Intersection::Split2 { intersection, remaining } => {
                    println!("{} {} {:?} {:?} {:?}", row1, row2, lr, rl, intersect(lr, *rl));
                    lrs.push((row1, remaining)).unwrap();
                    area += (intersection.1 - intersection.0 + 1) * (row2 - row1 + 1);
                    found = true;
                    break;
                }
                Intersection::Split3 { intersection, remaining } => {
                    println!("{} {} {:?} {:?} {:?}", row1, row2, lr, rl, intersect(lr, *rl));
                    lrs.push((row1, remaining.0)).unwrap();
                    lrs.push((row1, remaining.1)).unwrap();
                    area += (intersection.1 - intersection.0 + 1) * (row2 - row1 + 1);
                    found = true;
                    break;
                }
            }
        }
        assert!(found);
        i += 1;
        println!("{} {}", area - prev_area, area);
    }
    println!("{}", area);

    // collect missing pieces
    use Dir::*;
    let mut prev = &cmds[cmds.len() - 1];
    for i in 0..cmds.len() {
        let cur = &cmds[i];
        let next = &cmds[(i + 1) % cmds.len()];
        match (prev.dir, cur.dir, next.dir) {
            (Left, Up, Left) => area += cur.dist,
            (Right, Up, Left) => area += cur.dist - 1,
            (Left, Down, Left) => area += cur.dist,
            (Left, Down, Right) => area += cur.dist - 1,
            _ => (),
        }
        println!("{}", area);
        prev = cur;
    }

    area
}

pub fn part1(input: &str) -> i64 {
    let commands: Vec<Command> = parse(input).map(|x| x.0).collect();
    interior(&commands)
}

pub fn part2(input: &str) -> i64 {
    let commands: Vec<Command> = parse(input).map(|x| x.1).collect();
    for cmd in &commands {
        println!("{:?}", cmd);
    }
    interior(&commands)
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
        //assert_eq!(part2(input), 952408144115);
    }

    #[test]
    fn test_contrived() {
        let input = "R 5 (#aaaaa1)
D 3 (#aaaaa1)
L 2 (#aaaaa1)
U 2 (#aaaaa1)
L 1 (#aaaaa1)
D 2 (#aaaaa1)
L 2 (#aaaaa1)
U 3 (#aaaaa1)
";
        assert_eq!(part1(input), 24);

        let input = "R 3 (#aaaaa1)
D 1 (#aaaaa1)
L 3 (#aaaaa1)
U 1 (#aaaaa1)
";
        assert_eq!(part1(input), 8);

        let input = "R 2 (#aaaaa1)
D 2 (#aaaaa1)
R 1 (#aaaaa1)
U 2 (#aaaaa1)
R 2 (#aaaaa1)
D 7 (#aaaaa1)
L 2 (#aaaaa1)
U 2 (#aaaaa1)
L 1 (#aaaaa1)
D 2 (#aaaaa1)
L 2 (#aaaaa1)
U 7 (#aaaaa1)
";
        assert_eq!(part1(input), 48);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day18.txt");
        assert_eq!(part1(input), 40761);
        //assert_eq!(part2(input), 40761);
    }
}
