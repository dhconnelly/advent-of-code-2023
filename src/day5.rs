const MAX_ITEMS: usize = 1024;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Range {
    lo: i64,
    hi: i64,
}

impl Range {
    fn intersection(&self, other: &Range) -> Option<Range> {
        if self.lo > other.hi || self.hi < other.lo {
            None
        } else {
            let lo = self.lo.max(other.lo);
            let hi = self.hi.min(other.hi);
            Some(Range { lo, hi })
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.lo <= other.lo && self.hi >= other.hi
    }

    fn shift(&self, offset: i64) -> Range {
        Range { lo: self.lo + offset, hi: self.hi + offset }
    }
}

const MAX_RANGE: Range = Range { lo: i64::MAX, hi: i64::MAX };

fn parse_seeds(line: &str) -> ([Range; MAX_ITEMS], usize) {
    let (mut seeds, mut size) = ([MAX_RANGE; MAX_ITEMS], 0);
    let (_, line) = line.split_once(' ').unwrap();
    for (i, seed) in line.split(' ').enumerate() {
        let lo = seed.parse().unwrap();
        seeds[i] = Range { lo, hi: lo };
        size += 1;
    }
    (seeds, size)
}

fn parse_seed_ranges(line: &str) -> ([Range; MAX_ITEMS], usize) {
    let (mut seeds, mut size) = ([MAX_RANGE; MAX_ITEMS], 0);
    let (_, line) = line.split_once(' ').unwrap();
    let mut toks = line.split(' ').enumerate();
    while let Some((_, lo)) = toks.next() {
        let lo = lo.parse().unwrap();
        let (i, len) = toks.next().unwrap();
        let len: i64 = len.parse().unwrap();
        seeds[i / 2] = Range { lo, hi: lo + len - 1 };
        size += 1;
    }
    (seeds, size)
}

#[derive(Debug)]
struct RangeMap {
    src: Range,
    dst: Range,
}

fn parse_range(line: &str) -> RangeMap {
    let mut nums = line.split(' ');
    let dst_start = nums.next().unwrap().parse().unwrap();
    let src_start = nums.next().unwrap().parse().unwrap();
    let len: i64 = nums.next().unwrap().parse().unwrap();
    assert!(nums.next().is_none());
    let src = Range { lo: src_start, hi: src_start + len - 1 };
    let dst = Range { lo: dst_start, hi: src_start + len - 1 };
    RangeMap { src, dst }
}

#[derive(Debug)]
enum Outcome {
    NoChange,
    Moved(Range),
    Split2(Range, Range),
}

fn update(range: &Range, RangeMap { src, dst }: &RangeMap) -> Outcome {
    if src.contains(range) {
        Outcome::Moved(range.shift(dst.lo - src.lo))
    } else if let Some(intersection) = src.intersection(range) {
        let moved = intersection.shift(dst.lo - src.lo);
        let unmoved = if range.lo < intersection.lo {
            Range { lo: range.lo, hi: intersection.lo - 1 }
        } else {
            Range { lo: intersection.hi + 1, hi: range.hi }
        };
        Outcome::Split2(unmoved, moved)
    } else {
        Outcome::NoChange
    }
}

fn min_location<'a>(
    mut state: [Range; MAX_ITEMS],
    mut n: usize,
    chunks: impl Iterator<Item = &'a str>,
) -> i64 {
    let mut next_state = state;
    for chunk in chunks {
        for line in chunk.lines().skip(1) {
            let range = parse_range(line);
            for i in 0..n {
                match update(&state[i], &range) {
                    Outcome::NoChange => continue,
                    Outcome::Moved(next) => next_state[i] = next,
                    Outcome::Split2(a, b) => {
                        next_state[i] = a;
                        next_state[n] = b;
                        n += 1;
                    }
                }
            }
        }
        state = next_state;
    }
    state.iter().min().unwrap().lo
}

pub fn part1(input: &str) -> i64 {
    let mut chunks = input.split("\n\n");
    let (state, n) = parse_seeds(chunks.next().unwrap());
    min_location(state, n, chunks)
}

pub fn part2(input: &str) -> i64 {
    let mut chunks = input.split("\n\n");
    let (state, n) = parse_seed_ranges(chunks.next().unwrap());
    min_location(state, n, chunks)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!(part1(input), 35);
        assert_eq!(part2(input), 46);
    }

    #[test]
    fn test2() {
        let input = include_str!("../inputs/day5.txt");
        assert_eq!(part1(input), 322500873);
    }
}
