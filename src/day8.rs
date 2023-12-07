pub fn part1(input: &str) -> i64 {
    0
}

pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "";
        assert_eq!(part1(input), 0);
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day8.txt");
        assert_eq!(part1(input), 0);
        assert_eq!(part2(input), 0);
    }
}
