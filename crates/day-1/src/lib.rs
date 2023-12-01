use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct NodeIndex(usize);

#[derive(Debug, PartialEq)]
struct MapIndex(usize);

type NodeMap = HashMap<char, NodeIndex>;

#[derive(Debug, PartialEq)]
enum Node {
    Returnable(u32),
    Intermediate(MapIndex),
}

#[derive(Debug)]
struct Graph {
    maps: Vec<NodeMap>,
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Self {
            maps: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn forward() -> Self {
        let mut graph = Self::new();

        graph.add_key("one", 1);
        graph.add_key("two", 2);
        graph.add_key("three", 3);
        graph.add_key("four", 4);
        graph.add_key("five", 5);
        graph.add_key("six", 6);
        graph.add_key("seven", 7);
        graph.add_key("eight", 8);
        graph.add_key("nine", 9);
        graph.add_key("1", 1);
        graph.add_key("2", 2);
        graph.add_key("3", 3);
        graph.add_key("4", 4);
        graph.add_key("5", 5);
        graph.add_key("6", 6);
        graph.add_key("7", 7);
        graph.add_key("8", 8);
        graph.add_key("9", 9);

        graph
    }

    pub fn backward() -> Self {
        let mut graph = Self::new();

        graph.add_key("eno", 1);
        graph.add_key("owt", 2);
        graph.add_key("eerht", 3);
        graph.add_key("ruof", 4);
        graph.add_key("evif", 5);
        graph.add_key("xis", 6);
        graph.add_key("neves", 7);
        graph.add_key("thgie", 8);
        graph.add_key("enin", 9);
        graph.add_key("1", 1);
        graph.add_key("2", 2);
        graph.add_key("3", 3);
        graph.add_key("4", 4);
        graph.add_key("5", 5);
        graph.add_key("6", 6);
        graph.add_key("7", 7);
        graph.add_key("8", 8);
        graph.add_key("9", 9);

        graph
    }

    fn add_key(&mut self, key: &str, returnable: u32) {
        if key.is_empty() {
            return;
        }
        let len = key.len();

        let mut head = 0;
        for (index, char) in key.chars().enumerate() {
            let mut map_count = self.maps.len();
            let map = match self.maps.get_mut(head) {
                Some(map) => map,
                None => {
                    self.maps.push(HashMap::new());
                    map_count += 1;
                    self.maps.get_mut(head).expect("just added")
                }
            };

            match map.get(&char) {
                Some(node_index) => match self.nodes.get(node_index.0).expect("must exist") {
                    Node::Intermediate(map_index) => head = map_index.0,
                    Node::Returnable(_) => unreachable!(),
                },
                None => {
                    let node = if index == len - 1 {
                        Node::Returnable(returnable)
                    } else {
                        head = map_count;
                        Node::Intermediate(MapIndex(head))
                    };
                    self.nodes.push(node);

                    map.insert(char, NodeIndex(self.nodes.len() - 1));
                }
            }
        }
    }

    fn parse(&mut self, chars: impl Iterator<Item = char>) -> Option<u32> {
        let mut heads = Vec::new();

        for c in chars {
            heads.push(0);
            let mut offset = 0;

            for head_index in 0..heads.len() {
                let head_index = head_index - offset;

                if let Some(node) = self.get_node(c, heads[head_index]) {
                    match node {
                        Node::Returnable(result) => return Some(*result),
                        Node::Intermediate(map_index) => {
                            heads[head_index] = map_index.0;
                        }
                    }
                } else {
                    heads.remove(head_index);
                    offset += 1;
                }
            }
        }

        None
    }

    fn get_node(&mut self, char: char, index: usize) -> Option<&Node> {
        let map = self.maps.get(index).expect("always present");
        Some(&self.nodes[map.get(&char)?.0])
    }
}

pub fn solve_part_1(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let first_digit = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .expect("number is always present in input line")
            .to_digit(10)
            .expect("valid digit passed");
        let last_digit = line
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .expect("number is always present in input line")
            .to_digit(10)
            .expect("valid digit passed");

        result += first_digit * 10 + last_digit;
    }

    result
}

pub fn solve_part_2(input: &str) -> u32 {
    let mut result = 0;

    let mut forward_graph = Graph::forward();
    let mut backward_graph = Graph::backward();

    for line in input.lines() {
        let first = forward_graph
            .parse(line.chars())
            .expect("number is always present in input line");
        let last = backward_graph
            .parse(line.chars().rev())
            .expect("number is always present in input line");

        let number = first * 10 + last;
        result += number;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(solve_part_1(input), 142);
    }

    #[test]
    fn part_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(solve_part_2(input), 281);
    }
}
