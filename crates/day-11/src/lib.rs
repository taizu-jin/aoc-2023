use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Galaxy {
    x: i32,
    y: i32,
}

impl Galaxy {
    fn shortest_path_length(&self, other: &Galaxy) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl From<(usize, usize)> for Galaxy {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as i32,
            y: value.1 as i32,
        }
    }
}

struct Galaxies {
    galaxies: Vec<Galaxy>,
    empty_colums: HashSet<i32>,
    empty_rows: Vec<i32>,
}

impl From<&str> for Galaxies {
    fn from(value: &str) -> Self {
        let mut galaxies = Vec::new();

        let mut empty_rows = Vec::new();
        let mut empty_colums = HashSet::new();
        let width = value.lines().next().unwrap().chars().count();
        for i in 0..width {
            empty_colums.insert(i as i32);
        }

        for (y, line) in value.lines().enumerate() {
            let mut line_has_galaxy = true;

            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        line_has_galaxy = false;
                        empty_colums.remove(&(x as i32));
                        galaxies.push((x, y).into());
                    }
                    '.' => (),
                    _ => unreachable!(),
                }
            }

            if line_has_galaxy {
                empty_rows.push(y as i32);
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
    fn expand(&mut self, u32: i32) {
        for galaxy in &mut self.galaxies {
            let empty_colums_count =
                self.empty_colums.iter().filter(|c| **c < galaxy.x).count() as i32;
            let empty_row_count = self.empty_rows.iter().filter(|c| **c < galaxy.y).count() as i32;

            galaxy.x += empty_colums_count * u32 - empty_colums_count;
            galaxy.y += empty_row_count * u32 - empty_row_count;
        }
    }

    fn pair_shortest_path_length_sum(self) -> u32 {
        let mut galaxies: VecDeque<Galaxy> = self.galaxies.into();

        let mut sum = 0;

        for _ in 0..galaxies.len() {
            let galaxy = galaxies.pop_front().unwrap();
            sum += galaxies
                .iter()
                .map(|g| galaxy.shortest_path_length(g))
                .sum::<u32>();
        }

        sum
    }
}

pub fn solve(input: &str, scale: i32) -> u32 {
    let mut galaxies = Galaxies::from(input);
    galaxies.expand(scale);
    galaxies.pair_shortest_path_length_sum()
}

pub fn solve_part_1(input: &str) -> u32 {
    solve(input, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(input()), 374);
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
