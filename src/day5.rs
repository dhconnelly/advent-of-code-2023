use crate::static_vec::StaticVec;

type RangeVec = StaticVec<Range, 128>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Range {
    lo: i64,
    hi: i64,
}

impl Default for Range {
    fn default() -> Self {
        Range { lo: i64::MAX, hi: i64::MAX }
    }
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

fn parse_seeds(line: &str, state: &mut RangeVec) {
    for seed in line.split_once(' ').unwrap().1.split(' ') {
        let lo = seed.parse().unwrap();
        state.push(Range { lo, hi: lo });
    }
}

fn parse_seed_ranges(line: &str, state: &mut RangeVec) {
    let mut toks = line.split_once(' ').unwrap().1.split(' ');
    while let Some(lo) = toks.next() {
        let lo = lo.parse().unwrap();
        let len = toks.next().unwrap().parse::<i64>().unwrap();
        state.push(Range { lo, hi: lo + len - 1 });
    }
}

struct RangeMap {
    src: Range,
    dst: Range,
}

fn parse_map(line: &str) -> RangeMap {
    let mut nums = line.split(' ');
    let dst_start = nums.next().unwrap().parse().unwrap();
    let src_start = nums.next().unwrap().parse().unwrap();
    let len = nums.next().unwrap().parse::<i64>().unwrap();
    let src = Range { lo: src_start, hi: src_start + len - 1 };
    let dst = Range { lo: dst_start, hi: dst_start + len - 1 };
    RangeMap { src, dst }
}

enum Update {
    NoChange,
    Moved(Range),
    Split { unmoved: Range, moved: Range },
}

fn apply_map(range: &Range, RangeMap { src, dst }: &RangeMap) -> Update {
    if src.contains(range) {
        Update::Moved(range.shift(dst.lo - src.lo))
    } else if let Some(intersection) = src.intersection(range) {
        let moved = intersection.shift(dst.lo - src.lo);
        let unmoved = if range.lo < intersection.lo {
            Range { lo: range.lo, hi: intersection.lo - 1 }
        } else {
            Range { lo: intersection.hi + 1, hi: range.hi }
        };
        Update::Split { unmoved, moved }
    } else {
        Update::NoChange
    }
}

fn update_ranges(maps: &str, state: &mut RangeVec, scratch: &mut RangeVec) {
    for map in maps.lines().skip(1).map(parse_map) {
        for i in 0..state.len() {
            match apply_map(&state[i], &map) {
                Update::NoChange => continue,
                Update::Moved(next) => scratch[i] = next,
                Update::Split { unmoved, moved } => {
                    state[i] = unmoved;
                    scratch[i] = unmoved;
                    scratch.push(moved);
                }
            }
        }
    }
    *state = *scratch;
}

fn min_location<'a>(state: RangeVec, all_maps: impl Iterator<Item = &'a str>) -> i64 {
    let (mut state, mut scratch) = (state, state);
    for maps in all_maps {
        update_ranges(maps, &mut state, &mut scratch);
    }
    state.into_iter().min().unwrap().lo
}

pub fn part1(input: &str) -> i64 {
    let mut sections = input.split("\n\n");
    let mut state = RangeVec::default();
    parse_seeds(sections.next().unwrap(), &mut state);
    min_location(state, sections)
}

pub fn part2(input: &str) -> i64 {
    let mut sections = input.split("\n\n");
    let mut state = RangeVec::default();
    parse_seed_ranges(sections.next().unwrap(), &mut state);
    min_location(state, sections)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

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

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 46);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day5.txt");
        assert_eq!(part1(input), 322500873);
        assert_eq!(part2(input), 108956227);
    }
}
