use core::u128;

fn parse_nums(s: &str) -> impl Iterator<Item = u8> + '_ {
    s.split_whitespace().flat_map(|tok| tok.parse::<u8>().ok())
}

fn set_of(it: impl Iterator<Item = u8>) -> u128 {
    it.fold(0, |acc, x| acc | (1 << x))
}

fn set_contains(set: u128, num: u8) -> bool {
    set & (1 << num) > 0
}

fn card_wins(card: &str) -> (usize, usize) {
    let (card_name, nums) = card.split_once(':').unwrap();
    let card_idx = parse_nums(card_name).next().unwrap().into();
    let (winning, have) = nums.split_once('|').unwrap();
    let winning = set_of(parse_nums(winning));
    let have = parse_nums(have);
    let num_wins = have.filter(|x| set_contains(winning, *x)).count();
    (card_idx, num_wins)
}

pub fn part1(input: &str) -> usize {
    let wins = input.lines().map(card_wins).map(|(_, wins)| wins);
    let scores = wins.map(|wins| if wins == 0 { 0 } else { 1 << (wins - 1) });
    scores.sum()
}

pub fn part2(input: &str) -> usize {
    let mut counts = [0usize; 256];
    for (card, wins) in input.lines().map(card_wins) {
        counts[card] += 1;
        for i in 0..wins {
            counts[card + i + 1] += counts[card];
        }
    }
    counts.into_iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        assert_eq!(part1(input), 13);
        assert_eq!(part2(input), 30);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day4.txt");
        assert_eq!(part1(input), 19855);
        assert_eq!(part2(input), 10378710);
    }
}
