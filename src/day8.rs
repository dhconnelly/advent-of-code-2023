use crate::static_vec::StaticVec;

type Graph<'a> = StaticVec<(&'a str, (&'a str, &'a str)), 1024>;

fn lookup<'a>(graph: &'a Graph, key: &str) -> (&'a str, &'a str) {
    graph.binary_search_by_key(&key, |(s, _)| s).unwrap().1
}

fn parse<'a>(input: &'a str) -> (&str, Graph) {
    let mut lines = input.lines();
    let dirs = lines.next().unwrap();
    lines.next().unwrap();
    let mut graph = StaticVec::empty();
    for line in lines {
        let (from, to) = line.split_once(" = ").unwrap();
        let (left, right) = to.split_once(", ").unwrap();
        graph.push((from, (&left[1..], &right[..right.len() - 1])));
    }
    graph.sort_by(|(left, _), (right, _)| left.cmp(right));
    (dirs, graph)
}

fn dist(from: &str, to: impl Fn(&str) -> bool, dirs: &str, graph: &Graph) -> i64 {
    let mut steps = 0;
    let mut cur = from;
    for dir in dirs.bytes().cycle() {
        if to(cur) {
            break;
        }
        let (left, right) = lookup(&graph, cur);
        match dir {
            b'L' => cur = left,
            _ => cur = right,
        }
        steps += 1;
    }
    steps
}

pub fn part1(input: &str) -> i64 {
    let (dirs, graph) = parse(input);
    dist("AAA", |cur| cur == "ZZZ", dirs, &graph)
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
    let (dirs, graph) = parse(input);
    (0..graph.len())
        .filter_map(|i| match graph[i].0 {
            from if from.ends_with("A") => Some(from),
            _ => None,
        })
        .fold(1, |total_dist, start| {
            let this_dist = dist(start, |cur| cur.ends_with('Z'), dirs, &graph);
            lcm(total_dist, this_dist)
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
