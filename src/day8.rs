use crate::static_vec::StaticVec;

type Dir = u8;
type Graph<'a> = StaticVec<(&'a str, (&'a str, &'a str)), 1024>;
type IndexedGraph = StaticVec<(usize, usize), 1024>;
type IndexedKeys<'a> = StaticVec<&'a str, 1024>;

fn parse<'a>(input: &'a str) -> (&'a [Dir], IndexedGraph, IndexedKeys<'a>) {
    let mut lines = input.lines();
    let dirs = lines.next().unwrap().as_bytes();

    // build the key graph
    let mut graph = lines.skip(1).fold(Graph::empty(), |mut graph, line| {
        let (from, to) = line.split_once(" = ").unwrap();
        let (left, right) = to.split_once(", ").unwrap();
        graph.push((from, (&left[1..], &right[..right.len() - 1])));
        graph
    });
    graph.sort_by(|(left, _), (right, _)| left.cmp(right));

    // reduce to an index graph with a sidetable of keys to avoid online
    // binary searches
    let mut indexed_graph = IndexedGraph::empty();
    let mut keys = IndexedKeys::empty();
    for (key, (left, right)) in graph.into_iter() {
        let left = graph.binary_search_by_key(&left, |(s, _)| s).unwrap();
        let right = graph.binary_search_by_key(&right, |(s, _)| s).unwrap();
        indexed_graph.push((left, right));
        keys.push(key);
    }

    (dirs, indexed_graph, keys)
}

fn dist(from: usize, to: impl Fn(usize) -> bool, dirs: &[Dir], g: &IndexedGraph) -> i64 {
    let mut steps = 0;
    let mut cur = from;
    for dir in dirs.iter().cycle() {
        if to(cur) {
            break;
        }
        let (left, right) = g[cur];
        match dir {
            b'L' => cur = left,
            _ => cur = right,
        }
        steps += 1;
    }
    steps
}

pub fn part1(input: &str) -> i64 {
    let (dirs, graph, keys) = parse(input);
    let start = keys.binary_search_by_key(&"AAA", |s| *s).unwrap();
    let end = keys.binary_search_by_key(&"ZZZ", |s| *s).unwrap();
    dist(start, |cur| cur == end, dirs, &graph)
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while x != y {
        if x > y {
            x = x - y;
        } else {
            y = y - x;
        }
    }
    x
}

fn lcm(x: i64, y: i64) -> i64 {
    x * y / gcd(x, y)
}

pub fn part2(input: &str) -> i64 {
    let (dirs, graph, keys) = parse(input);
    let starts = (0..graph.len()).filter(|i| keys[*i].ends_with('A'));
    starts.fold(1, |total, start| {
        lcm(total, dist(start, |cur| keys[cur].ends_with('Z'), dirs, &graph))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_example2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_example3() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        assert_eq!(part2(input), 6);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day8.txt");
        assert_eq!(part1(input), 19783);
        assert_eq!(part2(input), 9177460370549);
    }
}
