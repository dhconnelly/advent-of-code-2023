use core::cmp::Ordering;

use crate::static_vec::StaticVec;

type Card = u8;

fn score_card(card: u8) -> i8 {
    match card {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => 11,
        b'T' => 10,
        b if b.is_ascii_digit() => (b - b'0') as i8,
        _ => panic!("unknown card"),
    }
}

fn score_joker(card: u8) -> i8 {
    if card == b'J' {
        -1
    } else {
        score_card(card)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    Empty,
}

impl HandType {
    fn score(self) -> i64 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
            HandType::Empty => 0,
        }
    }

    fn increment(self) -> HandType {
        match self {
            HandType::FiveOfAKind => HandType::FiveOfAKind,
            HandType::FourOfAKind => HandType::FiveOfAKind,
            HandType::FullHouse => HandType::FourOfAKind,
            HandType::ThreeOfAKind => HandType::FourOfAKind,
            HandType::TwoPair => HandType::FullHouse,
            HandType::OnePair => HandType::ThreeOfAKind,
            HandType::HighCard => HandType::OnePair,
            HandType::Empty => HandType::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Hand([Card; 5]);

impl Hand {
    fn counts(&self) -> StaticVec<i8, 15> {
        let mut counts = StaticVec::<i8, 15>::default();
        for i in self.0.iter().map(|c| score_card(*c)) {
            counts[i as usize] += 1;
        }
        counts
    }

    fn score_counts(counts: StaticVec<i8, 15>) -> HandType {
        let (mut fst, mut snd) = (0, 0);
        for count in counts.into_iter() {
            if count > fst {
                snd = fst;
                fst = count;
            } else if count > snd {
                snd = count;
            }
        }
        match (fst, snd) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            (1, _) => HandType::HighCard,
            (0, _) => HandType::Empty,
            _ => panic!("invalid hand"),
        }
    }

    fn score(&self) -> HandType {
        Self::score_counts(self.counts())
    }

    fn score_joker(&self) -> HandType {
        let mut counts = self.counts();
        let jokers = counts[score_card(b'J') as usize];
        counts[score_card(b'J') as usize] = 0;
        let mut typ = Self::score_counts(counts);
        for _ in 0..jokers {
            typ = typ.increment();
        }
        typ
    }
}

fn make_cmp(
    hand_type: impl Fn(&Hand) -> HandType,
    score_card: impl Fn(Card) -> i8,
) -> impl Fn(&Hand, &Hand) -> Ordering {
    move |l, r| {
        let cmp = hand_type(l).score() - hand_type(r).score();
        if cmp < 0 {
            Ordering::Less
        } else if cmp > 0 {
            Ordering::Greater
        } else {
            l.0.iter().map(|c| score_card(*c)).cmp(r.0.iter().map(|c| score_card(*c)))
        }
    }
}

fn parse_hand(s: &str) -> Hand {
    let mut hand = [0; 5];
    for (i, b) in s.as_bytes().iter().enumerate() {
        hand[i] = *b;
    }
    Hand(hand)
}

fn total_winnings(input: &str, cmp_hands: impl Fn(&Hand, &Hand) -> Ordering) -> i64 {
    let mut winnings = 0;
    let mut hands = StaticVec::<(Hand, i64), 1024>::default();
    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        hands.push((parse_hand(hand), bid.parse().unwrap()));
    }
    hands.sort(|l, r| cmp_hands(&l.0, &r.0).reverse());
    for place in 1..=hands.len() {
        winnings += place as i64 * hands[hands.len() - place].1;
    }
    winnings
}

pub fn part1(input: &str) -> i64 {
    total_winnings(input, make_cmp(Hand::score, score_card))
}

pub fn part2(input: &str) -> i64 {
    total_winnings(input, make_cmp(Hand::score_joker, score_joker))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_joker() {
        assert_eq!(parse_hand("QJJQ2").score_joker(), HandType::FourOfAKind);
        assert_eq!(parse_hand("KK677").score_joker(), HandType::TwoPair);
        assert_eq!(parse_hand("T55J5").score_joker(), HandType::FourOfAKind);
        assert_eq!(parse_hand("KTJJT").score_joker(), HandType::FourOfAKind);
        assert_eq!(parse_hand("QQQJA").score_joker(), HandType::FourOfAKind);
    }

    #[test]
    fn test_examples() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        assert_eq!(part1(input), 6440);
        assert_eq!(part2(input), 5905);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day7.txt");
        assert_eq!(part1(input), 248217452);
        assert_eq!(part2(input), 245576185);
    }
}
