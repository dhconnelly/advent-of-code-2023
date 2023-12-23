pub fn part1(input: &str) -> usize {
    0
}

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "";

    const REAL_INPUT: &str = include_str!("../inputs/day24.txt");

    #[test]
    fn test_example() {
        assert_eq!(part1(TEST_INPUT), 0);
        assert_eq!(part2(TEST_INPUT), 0);
    }

    #[test]
    fn test_real() {
        assert_eq!(part1(REAL_INPUT), 0);
        assert_eq!(part2(REAL_INPUT), 0);
    }
}
