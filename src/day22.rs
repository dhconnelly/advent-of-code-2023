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

type Cache = Vec<Vec<u16, 256>, 2048>;

fn compute_intersections(bricks: &Bricks, cache: &mut Cache) {
    fn has_overlap(l: &Brick, r: &Brick) -> bool {
        let ix = (l.0.x.max(r.0.x), l.1.x.min(r.1.x));
        let iy = (l.0.y.max(r.0.y), l.1.y.min(r.1.y));
        ix.0 <= ix.1 && iy.0 <= iy.1
    }
    cache.clear();
    cache.resize_default(bricks.len()).unwrap();
    for i in 0..bricks.len() - 1 {
        for j in i + 1..bricks.len() {
            if has_overlap(&bricks[i], &bricks[j]) {
                cache[i].push(j as u16).unwrap();
                cache[j].push(i as u16).unwrap();
            }
        }
    }
}

fn drop_dist(bricks: &Bricks, i: usize, cache: &Cache) -> i16 {
    // find the closest brick in the z-dimension that overlaps with this one
    // in the x and y dimensions and find the distance. if none exists, we
    // can fall all the way to the bottom.
    cache[i]
        .iter()
        .filter(|j| bricks[**j as usize].1.z < bricks[i].0.z)
        .map(|j| bricks[i].0.z - bricks[*j as usize].1.z - 1)
        .min()
        .unwrap_or(bricks[i].0.z - 1)
}

fn drop(bricks: &mut Bricks, i: usize, cache: &Cache) -> bool {
    let dz = drop_dist(bricks, i, cache);
    if dz > 0 {
        bricks[i].0.z -= dz;
        bricks[i].1.z -= dz;
    }
    dz > 0
}

fn drop_all(bricks: &mut Bricks, from: usize, to: usize, cache: &Cache) -> usize {
    (from..to).map(|i| drop(bricks, i, cache) as usize).sum()
}

fn supporting(bricks: &mut Bricks, below: usize, above: usize, cache: &Cache) -> bool {
    if bricks[above].0.z <= bricks[below].1.z || !cache[above].contains(&(below as u16)) {
        return false;
    }
    let saved = bricks[below];
    bricks[below] = REMOVED;
    let d = drop_dist(bricks, above, cache);
    bricks[below] = saved;
    d > 0
}

fn can_remove(bricks: &mut Bricks, i: usize, cache: &Cache) -> bool {
    // look for a brick that would fall on this one
    (i + 1..bricks.len()).all(|j| !supporting(bricks, i, j, cache))
}

fn remove_count(bricks: &mut Bricks, i: usize, cache: &Cache, memo: &mut [Option<usize>]) -> usize {
    // remove the brick and see how many fall. recursive: if a brick falls, then
    // each one it supports falls, and each one that one supports falls, etc.
    *memo[i].get_or_insert_with(|| {
        let mut next: Bricks = bricks.clone();
        next[i] = REMOVED;
        drop_all(&mut next, i + 1, bricks.len(), cache)
    })
}

pub fn part1(input: &str) -> usize {
    let mut bricks = parse(input);
    let mut cache = Cache::new();
    compute_intersections(&bricks, &mut cache);
    let n = bricks.len();
    drop_all(&mut bricks, 0, n, &cache);
    (0..bricks.len()).filter(|i| can_remove(&mut bricks, *i, &cache)).count()
}

pub fn part2(input: &str) -> usize {
    let mut bricks = parse(input);
    let n = bricks.len();
    let mut cache = Cache::new();
    compute_intersections(&bricks, &mut cache);
    drop_all(&mut bricks, 0, n, &cache);
    let mut memo: Vec<Option<usize>, 2048> = Vec::new();
    memo.resize(bricks.len(), None).unwrap();
    (0..bricks.len()).map(|i| remove_count(&mut bricks, i, &cache, &mut memo)).sum()
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
