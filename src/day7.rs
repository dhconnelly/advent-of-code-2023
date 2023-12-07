use core::cmp::Ordering;
use libc_print::std_name::println;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card(u8);

impl Card {
    fn score(self) -> i64 {
        let Card(card) = self;
        match card {
            b'A' => 14,
            b'K' => 13,
            b'Q' => 12,
            b'J' => 11,
            b'T' => 10,
            b if b.is_ascii_digit() => (b - b'0') as i64,
            _ => panic!("unknown card"),
        }
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
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand([Card; 5]);

impl Hand {
    fn typ(&self) -> HandType {
        todo!()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (score_l, score_r) = (self.typ().score(), other.typ().score());
        if score_l < score_r {
            Some(Ordering::Less)
        } else if score_l > score_r {
            Some(Ordering::Greater)
        } else {
            for (l, r) in self.0.iter().zip(other.0.iter()) {
                let cmp = l.score() - r.score();
                if cmp < 0 {
                    return Some(Ordering::Less);
                } else if cmp > 0 {
                    return Some(Ordering::Greater);
                }
            }
            Some(Ordering::Equal)
        }
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Hand {
        let mut hand = [Card(0); 5];
        for (i, b) in s.as_bytes().iter().enumerate() {
            hand[i] = Card(*b);
        }
        Hand(hand)
    }
}

pub fn part1(input: &str) -> i64 {
    let mut winnings = 0;
    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand: Hand = hand.into();
        let bid: i64 = bid.parse().unwrap();
        let typ = hand.typ();
        println!("{:?} {:?} {:?} {}", hand, bid, typ, typ.score());
    }
    winnings
}

pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        assert_eq!(part1(input), 6440);
    }
}
