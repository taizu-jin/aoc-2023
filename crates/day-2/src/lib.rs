struct GameSettings {
    blue_count: u32,
    red_count: u32,
    green_count: u32,
}

#[derive(Debug, PartialEq)]
enum CubeKind {
    Red,
    Blue,
    Green,
}

impl From<&str> for CubeKind {
    fn from(value: &str) -> Self {
        match value {
            "blue" => CubeKind::Blue,
            "red" => CubeKind::Red,
            "green" => CubeKind::Green,
            _ => unreachable!("only above matches are part of the input"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Cubes {
    count: u32,
    kind: CubeKind,
}

impl From<&str> for Cubes {
    fn from(value: &str) -> Self {
        let (count, kind) = value
            .trim()
            .split_once(' ')
            .expect("' ' always part of the input");

        Self {
            count: count.parse().expect("always valid int passed as input"),
            kind: kind.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Set(Vec<Cubes>);

impl From<&str> for Set {
    fn from(value: &str) -> Self {
        let mut set = Vec::new();

        for cubes in value.split(',') {
            set.push(cubes.into());
        }

        Self(set)
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn is_valid(&self, settings: &GameSettings) -> bool {
        for set in &self.sets {
            for cubes in &set.0 {
                match cubes.kind {
                    CubeKind::Red => {
                        if cubes.count > settings.red_count {
                            return false;
                        }
                    }
                    CubeKind::Blue => {
                        if cubes.count > settings.blue_count {
                            return false;
                        }
                    }
                    CubeKind::Green => {
                        if cubes.count > settings.green_count {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game_id, game_sets) = value.split_once(':').expect(": always part of the input");
        let game_id = game_id.trim_start_matches("Game ");

        let mut sets = Vec::new();

        for set in game_sets.split(';') {
            sets.push(set.into());
        }

        Self {
            id: game_id.parse().expect("always valid int passed as input"),
            sets,
        }
    }
}

pub fn solve_part_1(input: &str) -> u32 {
    let mut result = 0;

    let settings = GameSettings {
        blue_count: 14,
        red_count: 12,
        green_count: 13,
    };

    for line in input.lines() {
        let game = Game::from(line);
        if game.is_valid(&settings) {
            result += game.id;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game() {
        let case = Game {
            id: 1,
            sets: vec![Set(vec![Cubes {
                count: 3,
                kind: CubeKind::Red,
            }])],
        };
        let game = Game::from("Game 1: 3 red");

        assert_eq!(game, case)
    }

    #[test]
    fn part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_part_1(input), 8);
    }
}
