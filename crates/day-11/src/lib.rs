use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Galaxy {
    x: i64,
    y: i64,
}

impl Galaxy {
    fn shortest_path_length(&self, other: &Galaxy) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl From<(usize, usize)> for Galaxy {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as i64,
            y: value.1 as i64,
        }
    }
}

struct Galaxies {
    galaxies: Vec<Galaxy>,
    empty_colums: HashSet<i64>,
    empty_rows: Vec<i64>,
}

impl From<&str> for Galaxies {
    fn from(value: &str) -> Self {
        let mut galaxies = Vec::new();

        let mut empty_rows = Vec::new();
        let mut empty_colums = HashSet::new();
        let width = value.lines().next().unwrap().chars().count();
        for i in 0..width {
            empty_colums.insert(i as i64);
        }

        for (y, line) in value.lines().enumerate() {
            let mut line_has_galaxy = true;

            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        line_has_galaxy = false;
                        empty_colums.remove(&(x as i64));
                        galaxies.push((x, y).into());
                    }
                    '.' => (),
                    _ => unreachable!(),
                }
            }

            if line_has_galaxy {
                empty_rows.push(y as i64);
            }
        }

        Self {
            galaxies,
            empty_colums,
            empty_rows,
        }
    }
}

impl Galaxies {
    fn expand(&mut self, scale: i64) {
        for galaxy in &mut self.galaxies {
            let empty_colums_count =
                self.empty_colums.iter().filter(|c| **c < galaxy.x).count() as i64;
            let empty_row_count = self.empty_rows.iter().filter(|c| **c < galaxy.y).count() as i64;

            galaxy.x += empty_colums_count * scale - empty_colums_count;
            galaxy.y += empty_row_count * scale - empty_row_count;
        }
    }

    fn pair_shortest_path_length_sum(self) -> u64 {
        let mut galaxies: VecDeque<Galaxy> = self.galaxies.into();

        let mut sum = 0;

        for _ in 0..galaxies.len() {
            let galaxy = galaxies.pop_front().unwrap();
            sum += galaxies
                .iter()
                .map(|g| galaxy.shortest_path_length(g))
                .sum::<u64>();
        }

        sum
    }
}

pub fn solve(input: &str, scale: i64) -> u64 {
    let mut galaxies = Galaxies::from(input);
    galaxies.expand(scale);
    galaxies.pair_shortest_path_length_sum()
}

pub fn solve_part_1(input: &str) -> u64 {
    solve(input, 2)
}

pub fn solve_part_2(input: &str) -> u64 {
    solve(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(input()), 374);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(input(), 10), 1030);
        assert_eq!(solve(input(), 100), 8410);
    }

    fn input() -> &'static str {
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
    }
}
