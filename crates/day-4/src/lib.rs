use std::collections::{HashSet, VecDeque};

struct Card {
    wins: u32,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (winning_numbers, card_numbers) = value
            .split_once(':')
            .expect("failed to split at ':'")
            .1
            .split_once('|')
            .expect("failed to split numbers at '|'");

        let winning_numbers = winning_numbers
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .collect::<HashSet<_>>();

        let card_numbers = card_numbers
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .collect::<HashSet<_>>();

        let winning_count = winning_numbers.intersection(&card_numbers).count() as u32;

        Self {
            wins: winning_count,
        }
    }
}

pub fn solve_part_1<'a>(input: impl Iterator<Item = &'a str>) -> u32 {
    let mut result = 0;
    for line in input {
        let card = Card::from(line);
        if card.wins > 0 {
            result += 2u32.pow(card.wins - 1);
        }
    }
    result
}

#[allow(clippy::mut_range_bound)]
pub fn solve_part_2<'a>(input: impl Iterator<Item = &'a str>) -> u32 {
    let mut result = 0;
    let mut copies = VecDeque::<u32>::new();

    for line in input {
        let mut wins = Card::from(line).wins;
        let card_count = 1 + copies.pop_front().unwrap_or(0);

        for i in 0..wins {
            if let Some(count) = copies.get_mut(i as usize) {
                *count += card_count;
                wins -= 1;
            } else {
                break;
            }
        }

        for _ in 0..wins {
            copies.push_back(card_count);
        }

        result += card_count;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(solve_part_1(input.lines()), 13);
    }

    #[test]
    fn part_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(solve_part_2(input.lines()), 30);
    }
}
