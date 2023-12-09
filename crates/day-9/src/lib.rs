use std::collections::HashMap;

struct Sequence {
    seq: Vec<i32>,
    cache: HashMap<(usize, usize), i32>,
}

impl From<&str> for Sequence {
    fn from(value: &str) -> Self {
        let seq = value
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .rev()
            .collect();

        Sequence {
            seq,
            cache: HashMap::new(),
        }
    }
}

impl Sequence {
    fn rev(&mut self) {
        self.seq.reverse()
    }

    fn next(&mut self) -> i32 {
        let mut result = 0;
        let mut j = 0;

        loop {
            let s = self.get(0, j);
            result += s;

            if s == 0 {
                let mut all_zeros = true;

                for i in 0..self.seq.len() - j {
                    if self.get(i, j) != 0 {
                        all_zeros = false;
                        break;
                    }
                }

                if all_zeros {
                    break;
                }
            }

            j += 1;
        }

        result
    }

    fn get(&mut self, i: usize, j: usize) -> i32 {
        if let Some(result) = self.cache.get(&(i, j)) {
            return *result;
        }

        let result = if j == 0 {
            self.seq[i]
        } else {
            self.get(i, j - 1) - self.get(i + 1, j - 1)
        };

        self.cache.insert((i, j), result);

        result
    }
}

pub fn solve_part_1(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        result += Sequence::from(line).next();
    }

    result
}

pub fn solve_part_2(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let mut seq = Sequence::from(line);
        seq.rev();
        result += seq.next();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(input()), 114);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(input()), 2);
    }

    #[test]
    fn negative() {
        assert_eq!(solve_part_1("10 9 2 -15 -46 -95 -166 -263 -390 -551 -750 -991 -1278 -1615 -2006 -2455 -2966 -3543 -4190 -4911 -5710"), -6591);
    }

    fn input() -> &'static str {
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
    }
}
