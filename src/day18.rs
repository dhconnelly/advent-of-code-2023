type Vec<T> = heapless::Vec<T, 1024>;
type Pt = (i64, i64);

#[derive(Clone, Copy, Debug)]
struct Command {
    dir: Dir,
    dist: i64,
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

fn times((r, c): Pt, n: i64) -> Pt {
    (r * n, c * n)
}

fn det((y1, x1): Pt, (y2, x2): Pt) -> i64 {
    x1 * y2 - x2 * y1
}

fn interior(cmds: &[Command]) -> i64 {
    // https://en.wikipedia.org/wiki/Shoelace_formula
    use Dir::*;
    let mut area = 0;
    let (mut prev, mut prev_pt) = ((0, 0), (0, 0));
    for i in 0..cmds.len() {
        let (cur_cmd, next_cmd) = (cmds[i], cmds[(i + 1) % cmds.len()]);
        let cur @ (row, col) = add(prev, times(cur_cmd.dir.vec(), cur_cmd.dist));
        let pt = match (cur_cmd.dir, next_cmd.dir) {
            (Up, Right) | (Right, Up) => cur,
            (Right, Down) | (Down, Right) => (row, col + 1),
            (Down, Left) | (Left, Down) => (row + 1, col + 1),
            (Left, Up) | (Up, Left) => (row + 1, col),
            _ => unreachable!(),
        };
        area += det(prev_pt, pt);
        (prev, prev_pt) = (cur, pt);
    }
    area / 2
}

fn parse(input: &str) -> impl Iterator<Item = (Command, Command)> + '_ {
    input.lines().map(|line: &str| {
        // part 1
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

        // part 2
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

pub fn part1(input: &str) -> i64 {
    let commands: Vec<Command> = parse(input).map(|x| x.0).collect();
    interior(&commands)
}

pub fn part2(input: &str) -> i64 {
    let commands: Vec<Command> = parse(input).map(|x| x.1).collect();
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
        assert_eq!(part2(input), 952408144115);
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
        assert_eq!(part2(input), 106920098354636);
    }
}
