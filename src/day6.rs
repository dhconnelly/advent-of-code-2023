use libc_print::libc_println;

fn nums<'a>(line: &'a str) -> impl Iterator<Item = i64> + 'a {
    line.split_whitespace().skip(1).map(|tok| tok.trim().parse::<i64>().unwrap())
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = (i64, i64)> + 'a {
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let dists = lines.next().unwrap();
    nums(times).zip(nums(dists))
}

fn solve(time: i64, dist: i64) -> (f64, f64) {
    // x*(time-x) > dist
    // -x^2 + time*x - dist > 0
    // always two roots and between them it's positive
    let (a, b, c) = (-1.0, time as f64, -dist as f64);
    let discrim = b * b - 4.0 * a * c;
    let fst = (-b + discrim.sqrt()) / (2.0 * a);
    let snd = (-b - discrim.sqrt()) / (2.0 * a);
    (fst, snd)
}

fn ceil(x: f64) -> i64 {
    let ceil = x.ceil();
    if ceil == x {
        ceil as i64 + 1
    } else {
        ceil as i64
    }
}

fn floor(x: f64) -> i64 {
    let floor = x.floor();
    if floor == x {
        floor as i64 - 1
    } else {
        floor as i64
    }
}

pub fn part1(input: &str) -> i64 {
    let mut ways = 1;
    for (time, dist) in parse(input) {
        let (lo, hi) = solve(time, dist);
        let n = floor(hi) - ceil(lo) + 1;
        ways *= n;
    }
    ways
}

pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(part1(input), 288);
    }
}
