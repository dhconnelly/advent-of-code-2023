use core::ops::Deref;
use heapless::Vec;
use libc_print::std_name::*;

type Bricks = Vec<Brick, 2048>;
type Brick = (Pt, Pt);

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pt {
    x: i16,
    y: i16,
    z: i16,
}

const ZERO: Pt = Pt { x: 0, y: 0, z: 0 };

fn intersects_xy(l: &Brick, r: &Brick) -> bool {
    let ix = (l.0.x.max(r.0.x), l.1.x.min(r.1.x));
    let iy = (l.0.y.max(r.0.y), l.1.y.min(r.1.y));
    ix.0 <= ix.1 && iy.0 <= iy.1
}

// parse the bricks and return them in ascending sorted order by z-coord
fn parse(input: &str) -> Bricks {
    let parse_pt = |s: &str| {
        let mut toks = s.split(',');
        let x = toks.next().unwrap().parse::<i16>().unwrap();
        let y = toks.next().unwrap().parse::<i16>().unwrap();
        let z = toks.next().unwrap().parse::<i16>().unwrap();
        Pt { x, y, z }
    };
    let parse_brick = |line: &str| {
        let (a, b) = line.split_once('~').unwrap();
        (parse_pt(a), parse_pt(b))
    };
    let mut bricks: Bricks = input.lines().map(parse_brick).collect();
    bricks.sort_by_key(|brick| brick.0.z);
    bricks
}

static mut INTERSECTIONS: Vec<Vec<bool, 2048>, 2048> = Vec::new();

fn compute_intersections(bricks: &Bricks) {
    unsafe {
        INTERSECTIONS.clear();
        INTERSECTIONS.resize_default(bricks.len()).unwrap();
        for (i, brick) in bricks.iter().enumerate() {
            INTERSECTIONS[i].resize_default(bricks.len()).unwrap();
            for (j, other) in bricks.iter().enumerate() {
                INTERSECTIONS[i][j] = intersects_xy(brick, other);
            }
        }
    }
}

fn intersects_cached(i: usize, j: usize) -> bool {
    unsafe { INTERSECTIONS[i][j] }
}

fn drop_dist(bricks: &Bricks, i: usize) -> i16 {
    // find the closest brick in the z-dimension that overlaps with this one
    // in the x and y dimemnsions and find the distance. if none exists, we
    // can fall all the way to the bottom.
    let brick = bricks[i];
    bricks[..i]
        .iter()
        .enumerate()
        .rev()
        .filter(|(j, other)| other.1.z < brick.0.z && intersects_cached(i, *j))
        .map(|(_, other)| brick.0.z - other.1.z - 1)
        .min()
        .unwrap_or(brick.0.z - 1)
}

fn drop(bricks: &mut Bricks, i: usize) {
    let dz = drop_dist(bricks, i);
    if dz > 0 {
        bricks[i].0.z -= dz;
        bricks[i].1.z -= dz;
    }
}

fn drop_all(bricks: &mut Bricks) {
    for i in 0..bricks.len() {
        drop(bricks, i);
    }
}

fn fall_count(bricks: &mut Bricks, i: usize) -> usize {
    let brick = bricks[i];
    // look for a brick that would fall on this one
    let mut fall_count = 0;
    for j in i + 1..bricks.len() {
        let other = bricks[j];
        if other.0.z <= brick.1.z || !intersects_cached(i, j) {
            continue;
        }
        bricks[i] = (ZERO, ZERO);
        // if it doesn't fall, we can remove
        let d = drop_dist(bricks, j);
        bricks[i] = brick;
        if d > 0 {
            fall_count += 1;
        }
    }
    fall_count
}

pub fn part1(input: &str) -> usize {
    let mut bricks = parse(input);
    compute_intersections(&bricks);
    drop_all(&mut bricks);
    (0..bricks.len()).filter(|i| fall_count(&mut bricks, *i) == 0).count()
}

pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";
        assert_eq!(part1(input), 5);
        assert_eq!(part2(input), 0);

        let input = include_str!("../inputs/day22.txt");
        assert_eq!(part1(input), 403);
        assert_eq!(part2(input), 0);
    }
}
