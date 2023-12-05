const MAX_ITEMS: usize = 32;

fn parse_seeds(line: &str) -> ([i64; MAX_ITEMS], usize) {
    let (mut seeds, mut size) = ([i64::MAX; MAX_ITEMS], 0);
    let (_, line) = line.split_once(' ').unwrap();
    for (i, seed) in line.split(' ').enumerate() {
        seeds[i] = seed.parse().unwrap();
        size += 1;
    }
    (seeds, size)
}

#[derive(Debug)]
struct RangeMap {
    dst_start: i64,
    src_start: i64,
    len: i64,
}

fn parse_range(line: &str) -> RangeMap {
    let mut nums = line.split(' ');
    let dst_start = nums.next().unwrap().parse().unwrap();
    let src_start = nums.next().unwrap().parse().unwrap();
    let len = nums.next().unwrap().parse().unwrap();
    assert!(nums.next().is_none());
    RangeMap { dst_start, src_start, len }
}

fn update(val: i64, range: &RangeMap) -> Option<i64> {
    let offset = val - range.src_start;
    if offset < 0 || offset >= range.len {
        None
    } else {
        Some(range.dst_start + offset)
    }
}

pub fn part1(input: &str) -> i64 {
    let mut chunks = input.split("\n\n");
    let (mut state, n) = parse_seeds(chunks.next().unwrap());
    let mut next_state = state;
    for chunk in chunks {
        for line in chunk.lines().skip(1) {
            let range = parse_range(line);
            for i in 0..n {
                if let Some(next) = update(state[i], &range) {
                    next_state[i] = next;
                }
            }
        }
        state = next_state;
    }
    *state.iter().min().unwrap()
}

pub fn part2(input: &str) -> i64 {
    0
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
    }

    #[test]
    fn test2() {
        let input = include_str!("../inputs/day5.txt");
        assert_eq!(part1(input), 322500873);
    }
}
