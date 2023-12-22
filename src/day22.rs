use heapless::Vec;

type Bricks = Vec<Brick, 2048>;
type Brick = (Pt, Pt);
type Pt = (i16, i16, i16);

// adjacency list
type Overlaps = Vec<Vec<u16, 256>, 2048>;

const ZERO: Pt = (0, 0, 0);
const REMOVED: Brick = (ZERO, ZERO);

// parse the bricks and return them in ascending sorted order by z-coord
fn parse(input: &str) -> Bricks {
    let parse_pt = |s: &str| {
        let mut toks = s.split(',');
        let x = toks.next().unwrap().parse::<i16>().unwrap();
        let y = toks.next().unwrap().parse::<i16>().unwrap();
        let z = toks.next().unwrap().parse::<i16>().unwrap();
        (x, y, z)
    };
    let parse_brick = |line: &str| {
        let (a, b) = line.split_once('~').unwrap();
        (parse_pt(a), parse_pt(b))
    };
    let mut bricks: Bricks = input.lines().map(parse_brick).collect();
    bricks.sort_by_key(|brick| brick.0 .2);
    bricks
}

fn compute_intersections(bricks: &Bricks, overlaps: &mut Overlaps) {
    fn has_overlap(l: &Brick, r: &Brick) -> bool {
        let ix = (l.0 .0.max(r.0 .0), l.1 .0.min(r.1 .0));
        let iy = (l.0 .1.max(r.0 .1), l.1 .1.min(r.1 .1));
        ix.0 <= ix.1 && iy.0 <= iy.1
    }
    overlaps.clear();
    overlaps.resize_default(bricks.len()).unwrap();
    for i in 0..bricks.len() - 1 {
        for j in i + 1..bricks.len() {
            if has_overlap(&bricks[i], &bricks[j]) {
                overlaps[i].push(j as u16).unwrap();
                overlaps[j].push(i as u16).unwrap();
            }
        }
    }
}

fn drop_dist(bricks: &Bricks, i: usize, overlaps: &Overlaps) -> i16 {
    // find the closest brick in the z-dimension that overlaps with this one
    // in the x and y dimensions and find the distance. if none exists, we
    // can fall all the way to the bottom.
    let above_z = bricks[i].0 .2;
    overlaps[i]
        .iter()
        .filter_map(|j| {
            let below_z = bricks[*j as usize].1 .2;
            if below_z < above_z {
                Some(above_z - below_z - 1)
            } else {
                None
            }
        })
        .min()
        .unwrap_or(above_z - 1)
}

fn drop(bricks: &mut Bricks, i: usize, overlaps: &Overlaps) -> bool {
    let dz = drop_dist(bricks, i, overlaps);
    if dz > 0 {
        bricks[i].0 .2 -= dz;
        bricks[i].1 .2 -= dz;
    }
    dz > 0
}

fn drop_all(bricks: &mut Bricks, from: usize, to: usize, overlaps: &Overlaps) -> usize {
    (from..to).map(|i| drop(bricks, i, overlaps) as usize).sum()
}

fn can_remove(bricks: &mut Bricks, below: usize, overlaps: &Overlaps) -> bool {
    // look for a brick that would fall on this one
    for above in below + 1..bricks.len() {
        if bricks[above].0 .2 <= bricks[below].1 .2 || !overlaps[above].contains(&(below as u16)) {
            continue;
        }
        let saved = bricks[below];
        bricks[below] = REMOVED;
        let d = drop_dist(bricks, above, overlaps);
        bricks[below] = saved;
        if d > 0 {
            return false;
        }
    }
    true
}

fn remove(bricks: &mut Bricks, i: usize, overlaps: &Overlaps, memo: &mut [Option<usize>]) -> usize {
    // remove the brick and see how many fall. recursive: if a brick falls, then
    // each one it supports falls, and each one that one supports falls, etc.
    *memo[i].get_or_insert_with(|| {
        let mut next: Bricks = bricks.clone();
        next[i] = REMOVED;
        drop_all(&mut next, i + 1, bricks.len(), overlaps)
    })
}

pub fn part1(input: &str) -> usize {
    let mut bricks = parse(input);
    let mut overlaps = Overlaps::new();
    compute_intersections(&bricks, &mut overlaps);

    let n = bricks.len();
    drop_all(&mut bricks, 0, n, &overlaps);

    (0..bricks.len()).filter(|i| can_remove(&mut bricks, *i, &overlaps)).count()
}

pub fn part2(input: &str) -> usize {
    let mut bricks = parse(input);
    let mut overlaps = Overlaps::new();
    compute_intersections(&bricks, &mut overlaps);

    let n = bricks.len();
    drop_all(&mut bricks, 0, n, &overlaps);

    let mut memo: Vec<Option<usize>, 2048> = Vec::new();
    memo.resize(bricks.len(), None).unwrap();
    (0..bricks.len()).map(|i| remove(&mut bricks, i, &overlaps, &mut memo)).sum()
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
