type Seq = crate::static_vec::StaticVec<i64, 32>;

fn next(seq: Seq) -> i64 {
    if seq.iter().all(|x| *x == 0) {
        0
    } else {
        seq[seq.len() - 1] + next(diffs(&seq))
    }
}

fn prev(seq: Seq) -> i64 {
    if seq.iter().all(|x| *x == 0) {
        0
    } else {
        seq[0] - prev(diffs(&seq))
    }
}

fn diffs(seq: &Seq) -> Seq {
    (1..seq.len()).map(|i| seq[i] - seq[i - 1]).collect()
}

fn parse(line: &str) -> Seq {
    line.split_whitespace().map(|tok| tok.parse::<i64>().unwrap()).collect()
}

pub fn part1(input: &str) -> i64 {
    input.lines().map(parse).map(next).sum()
}

pub fn part2(input: &str) -> i64 {
    input.lines().map(parse).map(prev).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        assert_eq!(part1(input), 114);
        assert_eq!(part2(input), 2);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day9.txt");
        assert_eq!(part1(input), 1581679977);
        assert_eq!(part2(input), 889);
    }
}
