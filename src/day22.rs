use heapless::Vec;

type Bricks = Vec<Brick, 2048>;
type Brick = (Pt, Pt);

#[derive(Clone, Copy, PartialEq)]
struct Pt {
    x: i16,
    y: i16,
    z: i16,
}

const ZERO: Pt = Pt { x: 0, y: 0, z: 0 };
const REMOVED: Brick = (ZERO, ZERO);

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

// won't fit on the stack
static mut INTERSECTIONS: Vec<Vec<bool, 2048>, 2048> = Vec::new();

fn compute_intersections(bricks: &Bricks) {
    fn has_overlap(l: &Brick, r: &Brick) -> bool {
        let ix = (l.0.x.max(r.0.x), l.1.x.min(r.1.x));
        let iy = (l.0.y.max(r.0.y), l.1.y.min(r.1.y));
        ix.0 <= ix.1 && iy.0 <= iy.1
    }
    unsafe {
        INTERSECTIONS.clear();
        INTERSECTIONS.resize_default(bricks.len()).unwrap();
        for i in 0..bricks.len() - 1 {
            INTERSECTIONS[i].resize_default(bricks.len()).unwrap();
            for j in i + 1..bricks.len() {
                INTERSECTIONS[j].resize_default(bricks.len()).unwrap();
                let overlap = has_overlap(&bricks[i], &bricks[j]);
                INTERSECTIONS[i][j] = overlap;
                INTERSECTIONS[j][i] = overlap;
            }
        }
    }
}

fn has_overlap(i: usize, j: usize) -> bool {
    unsafe { INTERSECTIONS[i][j] }
}

fn drop_dist(bricks: &Bricks, i: usize) -> i16 {
    // find the closest brick in the z-dimension that overlaps with this one
    // in the x and y dimensions and find the distance. if none exists, we
    // can fall all the way to the bottom.
    // TODO: cache (bricks[..i], i)
    bricks[..i]
        .iter()
        .enumerate()
        .filter(|(j, other)| other.1.z < bricks[i].0.z && has_overlap(i, *j))
        .map(|(_, other)| bricks[i].0.z - other.1.z - 1)
        .min()
        .unwrap_or(bricks[i].0.z - 1)
}

fn drop(bricks: &mut Bricks, i: usize) -> bool {
    let dz = drop_dist(bricks, i);
    if dz > 0 {
        bricks[i].0.z -= dz;
        bricks[i].1.z -= dz;
    }
    dz > 0
}

fn drop_all(bricks: &mut Bricks, from: usize, to: usize) -> usize {
    (from..to).map(|i| drop(bricks, i) as usize).sum()
}

fn supporting(bricks: &mut Bricks, below: usize, above: usize) -> bool {
    if bricks[above].0.z <= bricks[below].1.z || !has_overlap(below, above) {
        return false;
    }
    let saved = bricks[below];
    bricks[below] = REMOVED;
    let d = drop_dist(bricks, above);
    bricks[below] = saved;
    d > 0
}

fn can_remove(bricks: &mut Bricks, i: usize) -> bool {
    // look for a brick that would fall on this one
    (i + 1..bricks.len()).all(|j| !supporting(bricks, i, j))
}

fn remove_count(bricks: &mut Bricks, i: usize, memo: &mut [Option<usize>]) -> usize {
    // remove the brick and see how many fall. recursive: if a brick falls, then
    // each one it supports falls, and each one that one supports falls, etc.
    *memo[i].get_or_insert_with(|| {
        let mut next: Bricks = bricks.clone();
        next[i] = REMOVED;
        drop_all(&mut next, i + 1, bricks.len())
    })
}

pub fn part1(input: &str) -> usize {
    let mut bricks = parse(input);
    compute_intersections(&bricks);
    let n = bricks.len();
    drop_all(&mut bricks, 0, n);
    (0..bricks.len()).filter(|i| can_remove(&mut bricks, *i)).count()
}

pub fn part2(input: &str) -> usize {
    let mut bricks = parse(input);
    let n = bricks.len();
    drop_all(&mut bricks, 0, n);
    compute_intersections(&bricks);
    let mut memo: Vec<Option<usize>, 2048> = Vec::new();
    memo.resize(bricks.len(), None).unwrap();
    (0..bricks.len()).map(|i| remove_count(&mut bricks, i, &mut memo)).sum()
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
        assert_eq!(part2(input), 7);

        let input = include_str!("../inputs/day22.txt");
        assert_eq!(part1(input), 403);
        assert_eq!(part2(input), 70189);
    }
}
