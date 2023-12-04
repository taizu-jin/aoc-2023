struct Number {
    _number: u32,
    is_winning: bool,
}

struct Card {
    _id: usize,
    numbers: Vec<Number>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (id, rest) = value.split_once(':').expect("failed to split at ':'");
        let id = id
            .trim_start_matches("Card")
            .trim()
            .parse::<usize>()
            .expect("failed to parse id as a number");
        let (winning_numbers, card_numbers) = rest
            .split_once('|')
            .expect("failed to split numbers at '|'");

        let winning_numbers = winning_numbers
            .trim()
            .split(' ')
            .filter_map(|n| {
                if n.is_empty() {
                    None
                } else {
                    Some(
                        n.parse::<u32>()
                            .unwrap_or_else(|_| panic!("failed to parse as a number '{}'", n)),
                    )
                }
            })
            .collect::<Vec<_>>();

        let numbers = card_numbers
            .trim()
            .split(' ')
            .filter_map(|n| {
                if n.is_empty() {
                    None
                } else {
                    let number = n
                        .parse::<u32>()
                        .unwrap_or_else(|_| panic!("failed to parse as a number '{}'", n));

                    Some(Number {
                        _number: number,
                        is_winning: winning_numbers.contains(&number),
                    })
                }
            })
            .collect::<Vec<_>>();

        Self { _id: id, numbers }
    }
}

pub fn solve_part_1<'a>(input: impl Iterator<Item = &'a str>) -> u32 {
    let mut result = 0;
    for line in input {
        let sum = Card::from(line)
            .numbers
            .iter()
            .filter(|n| n.is_winning)
            .enumerate()
            .map(|(i, _)| i as u32 + 1)
            .reduce(|acc, _| acc * 2)
            .unwrap_or(0);
        result += sum;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::solve_part_1;

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
}
