use crate::static_vec::StaticVec;
use core::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Card {
    #[default]
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn score(&self) -> i8 {
        *self as i8
    }

    fn score_joker(&self) -> i8 {
        match self {
            Card::J => -1,
            _ => self.score(),
        }
    }
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            b'2' => Card::Two,
            b'3' => Card::Three,
            b'4' => Card::Four,
            b'5' => Card::Five,
            b'6' => Card::Six,
            b'7' => Card::Seven,
            b'8' => Card::Eight,
            b'9' => Card::Nine,
            b'T' => Card::Ten,
            b'J' => Card::J,
            b'Q' => Card::Q,
            b'K' => Card::K,
            b'A' => Card::A,
            _ => panic!("invalid card"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum HandType {
    Empty,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn score(self) -> i8 {
        self as i8
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
    fn counts(&self) -> StaticVec<i8, 13> {
        let mut counts = StaticVec::of(0);
        for card in self.0.iter() {
            counts[card.score() as usize] += 1;
        }
        counts
    }

    fn score_counts(card_counts: StaticVec<i8, 13>) -> HandType {
        match card_counts.into_iter().fold((0, 0), |(fst, snd), count| {
            if count > fst {
                (count, fst)
            } else if count > snd {
                (fst, count)
            } else {
                (fst, snd)
            }
        }) {
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

    fn typ(&self) -> HandType {
        Self::score_counts(self.counts())
    }

    fn typ_joker(&self) -> HandType {
        let mut counts = self.counts();
        let jokers = counts[Card::J as usize];
        counts[Card::J as usize] = 0;
        (0..jokers).fold(Self::score_counts(counts), |typ, _| typ.increment())
    }
}

fn make_cmp(
    hand_type: impl Fn(&Hand) -> HandType,
    score_card: impl Fn(&Card) -> i8,
) -> impl Fn(&Hand, &Hand) -> Ordering {
    move |l, r| match hand_type(l).score() - hand_type(r).score() {
        cmp if cmp < 0 => Ordering::Less,
        cmp if cmp > 0 => Ordering::Greater,
        _ => l.0.iter().map(&score_card).cmp(r.0.iter().map(&score_card)),
    }
}

fn parse_hand(s: &str) -> Hand {
    let mut hand = [Card::default(); 5];
    for (i, b) in s.as_bytes().iter().copied().enumerate() {
        hand[i] = b.into();
    }
    Hand(hand)
}

fn total_winnings(input: &str, cmp_hands: impl Fn(&Hand, &Hand) -> Ordering) -> i64 {
    let mut hands = StaticVec::<(Hand, i64), 1024>::empty();
    for (hand, bid) in input.lines().map(|line| line.split_once(' ').unwrap()) {
        hands.push((parse_hand(hand), bid.parse().unwrap()));
    }
    hands.sort_by(|l, r| cmp_hands(&l.0, &r.0));
    (0..hands.len()).map(|place| (place + 1) as i64 * hands[place].1).sum()
}

pub fn part1(input: &str) -> i64 {
    total_winnings(input, make_cmp(Hand::typ, Card::score))
}

pub fn part2(input: &str) -> i64 {
    total_winnings(input, make_cmp(Hand::typ_joker, Card::score_joker))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_joker() {
        assert_eq!(parse_hand("QJJQ2").typ_joker(), HandType::FourOfAKind);
        assert_eq!(parse_hand("KK677").typ_joker(), HandType::TwoPair);
        assert_eq!(parse_hand("T55J5").typ_joker(), HandType::FourOfAKind);
        assert_eq!(parse_hand("KTJJT").typ_joker(), HandType::FourOfAKind);
        assert_eq!(parse_hand("QQQJA").typ_joker(), HandType::FourOfAKind);
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
