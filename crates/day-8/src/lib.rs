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
}

impl Graph<'_> {
    fn steps(&self, instructions: &Instructions, start: &str, end: &str) -> u32 {
        let mut steps = 0;
        let mut key = start;
        'search: loop {
            for ins in instructions.str.chars() {
                let node = self.map.get(key).unwrap();
                if key == end {
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
}

impl<'a> From<&'a str> for Graph<'a> {
    fn from(value: &'a str) -> Self {
        let mut map = HashMap::new();
        for line in value.lines() {
            let (key, node) = line.split_once(" = ").unwrap();
            map.insert(key, node.into());
        }

        Self { map }
    }
}

pub fn solve_part_1(input: &str) -> u32 {
    let (instructions, graph) = input.split_once("\n\n").unwrap();
    let instructions = Instructions::from(instructions);
    let graph = Graph::from(graph);

    graph.steps(&instructions, "AAA", "ZZZ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(input()), 2);
        assert_eq!(solve_part_1(input_repeating()), 6);
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
}
