use core::cmp::Ordering;

use crate::static_vec::StaticVec;
use libc_print::std_name::println;

#[derive(Debug, Clone, Copy)]
enum Dir {
    L,
    R,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'L' => Dir::L,
            'R' => Dir::R,
            _ => panic!("unknown dir"),
        }
    }
}

struct Graph<'a>(StaticVec<(&'a str, (&'a str, &'a str)), 1024>);

impl<'a> Graph<'a> {
    fn get(&'a self, s: &str) -> (&'a str, &'a str) {
        self.0.search(&s, |(key, _)| key).unwrap().1
    }
}

fn parse<'a>(input: &'a str) -> (impl Iterator<Item = Dir> + 'a, Graph<'a>) {
    let mut lines = input.lines();
    let dirs = lines.next().unwrap().chars().map(Dir::from);
    lines.next().unwrap();
    let mut graph = StaticVec::empty();
    for line in lines {
        let (from, to) = line.split_once(" = ").unwrap();
        let (left, right) = to.split_once(", ").unwrap();
        graph.push((from, (&left[1..], &right[..right.len() - 1])));
    }
    graph.sort(|(left, _), (right, _)| left.cmp(right));
    (dirs.cycle(), Graph(graph))
}

pub fn part1(input: &str) -> i64 {
    let (dirs, graph) = parse(input);
    let mut steps = 0;
    let mut cur = "AAA";
    for dir in dirs {
        if cur == "ZZZ" {
            break;
        }
        let (left, right) = graph.get(cur);
        match dir {
            Dir::L => cur = left,
            Dir::R => cur = right,
        }
        steps += 1;
    }
    steps
}

pub fn part2(input: &str) -> i64 {
    let (dirs, graph) = parse(input);
    let mut steps = 0;
    let mut cur = StaticVec::<&str, 8>::empty();
    for i in 0..graph.0.len() {
        let (from, _) = graph.0[i];
        if from.ends_with("A") {
            cur.push(from);
        }
    }
    for dir in dirs {
        if (0..cur.len()).all(|i| cur[i].ends_with("Z")) {
            break;
        }
        for i in 0..cur.len() {
            let (left, right) = graph.get(cur[i]);
            match dir {
                Dir::L => cur[i] = left,
                Dir::R => cur[i] = right,
            }
        }
        steps += 1;
    }
    steps
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
        assert_eq!(part2(input), 0);
    }
}
