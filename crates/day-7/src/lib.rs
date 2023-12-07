use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum CardKind {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for CardKind {
    fn from(value: char) -> Self {
        match value {
            '1' => CardKind::One,
            '2' => CardKind::Two,
            '3' => CardKind::Three,
            '4' => CardKind::Four,
            '5' => CardKind::Five,
            '6' => CardKind::Six,
            '7' => CardKind::Seven,
            '8' => CardKind::Eight,
            '9' => CardKind::Nine,
            'T' => CardKind::Ten,
            'J' => CardKind::Jack,
            'Q' => CardKind::Queen,
            'K' => CardKind::King,
            'A' => CardKind::Ace,
            _ => unimplemented!("failed to convert {} to a card kind", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl From<HashMap<CardKind, u32>> for HandKind {
    fn from(value: HashMap<CardKind, u32>) -> Self {
        match value.len() {
            1 => HandKind::FiveOfKind,
            2 => {
                let hand = value.into_values().collect::<Vec<_>>();
                match hand.as_slice() {
                    &[1, 2] | &[2, 1] => HandKind::TwoPair,
                    &[3, 1] | &[1, 3] => HandKind::ThreeOfKind,
                    &[4, 1] | &[1, 4] => HandKind::FourOfKind,
                    &[3, 2] | &[2, 3] => HandKind::FullHouse,
                    _ => unreachable!("first count: {}, second count: {}", hand[0], hand[1]),
                }
            }
            3 => {
                let hand = value.into_values().collect::<Vec<_>>();
                match hand.as_slice() {
                    &[3, 1, 1] | &[1, 3, 1] | &[1, 1, 3] => HandKind::ThreeOfKind,
                    &[2, 2, 1] | &[2, 1, 2] | &[1, 2, 2] => HandKind::TwoPair,
                    _ => unreachable!(
                        "first count: {}, second count: {}, third count: {}",
                        hand[0], hand[1], hand[2]
                    ),
                }
            }
            4 => HandKind::OnePair,
            5 => HandKind::HighCard,
            _ => unreachable!("incorrectly parsed hand:\n{:?}", value),
        }
    }
}

#[derive(Debug)]
struct Hand {
    bid: u64,
    kind: HandKind,
    held: [CardKind; 5],
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (hand, bid) = value.split_once(' ').unwrap();
        let bid = bid.parse().expect("failed to parse bid");

        let mut map: HashMap<CardKind, u32> = HashMap::new();
        let mut held = [CardKind::One; 5];

        for (i, card) in hand.chars().enumerate() {
            let card = card.into();
            if let Some(held) = map.get_mut(&card) {
                *held += 1;
            } else {
                map.insert(card, 1);
            }
            held[i] = card;
        }

        Self {
            bid,
            kind: map.into(),
            held,
        }
    }
}

pub fn solve_part_1(input: &str) -> u64 {
    let mut result = 0;

    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        hands.push(line.into());
    }
    hands.sort_by(|l, r| {
        let mut ordering = l.kind.cmp(&r.kind);
        if ordering == Ordering::Equal {
            for (l, r) in l.held.iter().zip(r.held.iter()) {
                if l != r {
                    ordering = l.cmp(r);
                    break;
                }
            }
        }
        ordering
    });
    for (i, h) in hands.into_iter().enumerate() {
        let i = i + 1;
        result += i as u64 * h.bid;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparison() {
        let input = "33332 1
2AAAA 2
77888 3
77788 4";
        assert_eq!(solve_part_1(input), 4 + 2 * 3 + 3 * 2 + 4)
    }

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(input()), 6440)
    }

    fn input() -> &'static str {
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
    }
}
