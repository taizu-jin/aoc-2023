use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum CardKind {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum CardKindWildCard {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

macro_rules! impl_card_kind_from {
    ($enum:ident) => {
        impl From<char> for $enum {
            fn from(value: char) -> Self {
                match value {
                    '2' => $enum::Two,
                    '3' => $enum::Three,
                    '4' => $enum::Four,
                    '5' => $enum::Five,
                    '6' => $enum::Six,
                    '7' => $enum::Seven,
                    '8' => $enum::Eight,
                    '9' => $enum::Nine,
                    'T' => $enum::Ten,
                    'J' => $enum::Jack,
                    'Q' => $enum::Queen,
                    'K' => $enum::King,
                    'A' => $enum::Ace,
                    _ => unimplemented!("failed to convert {} to a card kind", value),
                }
            }
        }
    };
}

impl_card_kind_from!(CardKind);
impl_card_kind_from!(CardKindWildCard);

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

impl From<&[u32]> for HandKind {
    fn from(hand: &[u32]) -> Self {
        match hand.len() {
            1 => HandKind::FiveOfKind,
            2 => match hand {
                &[1, 2] | &[2, 1] => HandKind::TwoPair,
                &[3, 1] | &[1, 3] => HandKind::ThreeOfKind,
                &[4, 1] | &[1, 4] => HandKind::FourOfKind,
                &[3, 2] | &[2, 3] => HandKind::FullHouse,
                _ => unreachable!("first count: {}, second count: {}", hand[0], hand[1]),
            },
            3 => match hand {
                &[3, 1, 1] | &[1, 3, 1] | &[1, 1, 3] => HandKind::ThreeOfKind,
                &[2, 2, 1] | &[2, 1, 2] | &[1, 2, 2] => HandKind::TwoPair,
                _ => unreachable!(
                    "first count: {}, second count: {}, third count: {}",
                    hand[0], hand[1], hand[2]
                ),
            },
            4 => HandKind::OnePair,
            5 => HandKind::HighCard,
            _ => unreachable!("failed to convert hand to kind: {:?}", hand),
        }
    }
}

#[derive(Debug)]
struct Hand<T> {
    bid: u64,
    kind: HandKind,
    held: [T; 5],
}

impl From<&str> for Hand<CardKind> {
    fn from(value: &str) -> Self {
        let (hand, bid) = value.split_once(' ').unwrap();
        let bid = bid.parse().expect("failed to parse bid");

        let mut map: HashMap<CardKind, u32> = HashMap::new();
        let mut held = [CardKind::Two; 5];

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
            kind: map.into_values().collect::<Vec<_>>().as_slice().into(),
            held,
        }
    }
}

impl From<&str> for Hand<CardKindWildCard> {
    fn from(value: &str) -> Self {
        let (hand, bid) = value.split_once(' ').unwrap();
        let bid = bid.parse().expect("failed to parse bid");

        let mut map: HashMap<CardKindWildCard, u32> = HashMap::new();
        let mut held = [CardKindWildCard::Two; 5];

        for (i, card) in hand.chars().enumerate() {
            let card = card.into();
            if let Some(held) = map.get_mut(&card) {
                *held += 1;
            } else {
                map.insert(card, 1);
            }
            held[i] = card;
        }

        let kind = if let Some(j) = map.remove(&CardKindWildCard::Jack) {
            if j == 5 {
                HandKind::FiveOfKind
            } else {
                let mut kinds: Vec<HandKind> = Vec::new();
                let mut hand = map.into_iter().collect::<VecDeque<_>>();

                for _ in 0..hand.len() {
                    let card = hand.pop_front().unwrap();

                    let mut map: HashMap<_, _> = [(card.0, card.1 + j)].into();
                    for c in &hand {
                        map.insert(c.0, c.1);
                    }
                    hand.push_back(card);

                    kinds.push(map.into_values().collect::<Vec<_>>().as_slice().into());
                }

                kinds.into_iter().max().unwrap()
            }
        } else {
            map.into_values().collect::<Vec<_>>().as_slice().into()
        };

        Self { bid, kind, held }
    }
}

fn solve<'a, T>(input: &'a str) -> u64
where
    T: PartialEq + PartialOrd + Ord,
    Hand<T>: From<&'a str>,
{
    let mut result = 0;

    let mut hands: Vec<Hand<T>> = Vec::new();
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
        result += (i + 1) as u64 * h.bid;
    }

    result
}

pub fn solve_part_1(input: &str) -> u64 {
    solve::<CardKind>(input)
}

pub fn solve_part_2(input: &str) -> u64 {
    solve::<CardKindWildCard>(input)
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

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(input()), 5905)
    }

    fn input() -> &'static str {
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
    }
}
