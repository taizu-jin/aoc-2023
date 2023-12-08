use std::collections::HashMap;

#[derive(Debug)]
struct Instructions<'a> {
    str: &'a str,
}

impl<'a> From<&'a str> for Instructions<'a> {
    fn from(str: &'a str) -> Self {
        Self { str }
    }
}

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> From<&'a str> for Node<'a> {
    fn from(value: &'a str) -> Self {
        let (left, right) = value
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();

        Self { left, right }
    }
}

#[derive(Debug)]
struct Graph<'a> {
    map: HashMap<&'a str, Node<'a>>,
    starts: Vec<&'a str>,
}

impl Graph<'_> {
    fn steps(&self, instructions: &Instructions) -> u32 {
        let mut steps = 0;
        let mut key = "AAA";

        'search: loop {
            for ins in instructions.str.chars() {
                let node = self.map.get(key).unwrap();
                if key == "ZZZ" {
                    break 'search;
                }

                match ins {
                    'L' => key = node.left,
                    'R' => key = node.right,
                    _ => unreachable!(),
                }

                steps += 1;
            }
        }

        steps
    }

    fn ghost_steps(mut self, instructions: &Instructions) -> u64 {
        let mut steps = Vec::new();

        for key in &mut self.starts {
            let mut gs = 0;

            'search: loop {
                for ins in instructions.str.chars() {
                    let node = self.map.get(key).unwrap();

                    if key.ends_with('Z') {
                        break 'search;
                    }

                    match ins {
                        'L' => *key = node.left,
                        'R' => *key = node.right,
                        _ => unreachable!(),
                    }

                    gs += 1;
                }
            }

            steps.push(gs);
        }

        steps
            .into_iter()
            .filter_map(|x| u64::try_from(x).ok())
            .reduce(lcm)
            .unwrap()
    }
}

fn lcm(x: u64, y: u64) -> u64 {
    let mut max = x.max(y);
    let mut min = y.min(x);

    let gcd = loop {
        let res = max % min;

        if res == 0 {
            break min;
        }

        max = min;
        min = res;
    };

    x * y / gcd
}

impl<'a> From<&'a str> for Graph<'a> {
    fn from(value: &'a str) -> Self {
        let mut map = HashMap::new();
        let mut starts = Vec::new();
        for line in value.lines() {
            let (key, node) = line.split_once(" = ").unwrap();
            map.insert(key, node.into());

            if key.ends_with('A') {
                starts.push(key);
            }
        }

        Self { map, starts }
    }
}

pub fn solve_part_1(input: &str) -> u32 {
    let (instructions, graph) = input.split_once("\n\n").unwrap();
    let instructions = Instructions::from(instructions);
    let graph = Graph::from(graph);

    graph.steps(&instructions)
}

pub fn solve_part_2(input: &str) -> u64 {
    let (instructions, graph) = input.split_once("\n\n").unwrap();
    let instructions = Instructions::from(instructions);
    let graph = Graph::from(graph);

    graph.ghost_steps(&instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn least_common_multiple() {
        assert_eq!(lcm(20, 15), 60);
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(13, lcm(12, 18)), 468);
    }

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(input()), 2);
        assert_eq!(solve_part_1(input_repeating()), 6);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(input_ghost()), 6);
    }

    fn input() -> &'static str {
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
    }

    fn input_repeating() -> &'static str {
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
    }

    fn input_ghost() -> &'static str {
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
    }
}
