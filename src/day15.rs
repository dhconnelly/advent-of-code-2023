fn hash(s: &str) -> u8 {
    s.bytes().fold(0u16, |cur, b| (cur + b as u16) * 17 % 256) as u8
}

pub fn part1(input: &str) -> i64 {
    input.split(',').map(str::trim).map(hash).map(|h| h as i64).sum()
}

pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(input), 1320);
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day15.txt");
        assert_eq!(part1(input), 508498);
        assert_eq!(part2(input), 0);
    }
}
