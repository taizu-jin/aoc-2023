use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Coord {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SymbolKind {
    Symbol,
    Gear(u32),
}

#[derive(Debug)]
struct Symbol(char);

#[derive(Clone, Copy, PartialEq)]
enum NumberKind {
    Number,
    PartNumber,
}

#[derive(Debug)]
struct Number(u32);

#[derive(Debug)]
struct NumberBuffer {
    coords: Vec<Coord>,
    buffer: String,
}

impl NumberBuffer {
    fn new() -> Self {
        Self {
            coords: Vec::new(),
            buffer: String::new(),
        }
    }

    fn push(&mut self, char: char, coord: Coord) {
        if char.is_ascii_digit() {
            self.buffer.push(char);
            self.coords.push(coord);
        }
    }

    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    fn yeld(&mut self) -> (Vec<Coord>, Number) {
        (
            self.coords.split_off(0),
            Number(
                self.buffer
                    .split_off(0)
                    .parse()
                    .expect("contains only ascii digits"),
            ),
        )
    }
}

struct Cache {
    symbol_map: HashMap<Coord, usize>,
    number_map: HashMap<Coord, usize>,
    symbols: Vec<(Symbol, SymbolKind)>,
    numbers: Vec<(Number, NumberKind)>,
}

impl Cache {
    fn new() -> Self {
        Self {
            symbol_map: HashMap::new(),
            number_map: HashMap::new(),
            symbols: Vec::new(),
            numbers: Vec::new(),
        }
    }

    fn push_symbol(&mut self, coord: Coord, symbol: char) {
        let index = self.symbols.len();
        self.symbol_map.insert(coord, index);
        self.symbols.push((Symbol(symbol), SymbolKind::Symbol));
    }

    fn push_number(&mut self, coords: Vec<Coord>, number: Number) {
        let index = self.numbers.len();
        for coord in coords {
            self.number_map.insert(coord, index);
        }
        self.numbers.push((number, NumberKind::Number));
    }

    fn find_part_numbers_and_gears(&mut self) {
        for (coord, index) in &self.symbol_map {
            let coords: Vec<Coord> = vec![
                (coord.x - 1, coord.y).into(),
                (coord.x, coord.y - 1).into(),
                (coord.x - 1, coord.y - 1).into(),
                (coord.x + 1, coord.y).into(),
                (coord.x, coord.y + 1).into(),
                (coord.x + 1, coord.y + 1).into(),
                (coord.x - 1, coord.y + 1).into(),
                (coord.x + 1, coord.y - 1).into(),
            ];

            let mut counted_indexes = Vec::new();

            let mut part_number_count = 0;
            let mut gear_ratio = 1;

            for coord in &coords {
                if let Some(index) = self.number_map.get(coord) {
                    if counted_indexes.contains(&index) {
                        continue;
                    }

                    counted_indexes.push(index);

                    let (number, kind) = self
                        .numbers
                        .get_mut(*index)
                        .expect("always exists if an index is found");
                    *kind = NumberKind::PartNumber;

                    part_number_count += 1;
                    gear_ratio *= number.0;
                }
            }

            if part_number_count == 2 {
                let (symbol, kind) = self
                    .symbols
                    .get_mut(*index)
                    .expect("always exists if an index is found");
                if symbol.0 == '*' {
                    *kind = SymbolKind::Gear(gear_ratio);
                }
            }
        }
    }
}

struct Crawler;

impl Crawler {
    fn crawl<'a>(input: impl Iterator<Item = &'a str>) -> Cache {
        let mut cache = Cache::new();

        for (y, line) in input.enumerate() {
            let mut buffer = NumberBuffer::new();

            for (x, char) in line.chars().enumerate() {
                match char {
                    char if char.is_ascii_digit() => {
                        buffer.push(char, (x, y).into());
                    }
                    '.' => {
                        if !buffer.is_empty() {
                            let (coords, number) = buffer.yeld();
                            cache.push_number(coords, number);
                        }
                    }
                    char => {
                        if !buffer.is_empty() {
                            let (coords, number) = buffer.yeld();
                            cache.push_number(coords, number);
                        }

                        cache.push_symbol((x, y).into(), char);
                    }
                }
            }
            if !buffer.is_empty() {
                let (coords, number) = buffer.yeld();
                cache.push_number(coords, number);
            }
        }

        cache
    }
}

pub fn solve_part_1<'a>(input: impl Iterator<Item = &'a str>) -> u32 {
    let mut cache = Crawler::crawl(input);
    cache.find_part_numbers_and_gears();
    cache
        .numbers
        .iter()
        .filter(|(_, kind)| kind == &NumberKind::PartNumber)
        .map(|(number, _)| number.0)
        .sum()
}

pub fn solve_part_2<'a>(input: impl Iterator<Item = &'a str>) -> u32 {
    let mut cache = Crawler::crawl(input);
    cache.find_part_numbers_and_gears();
    cache
        .symbols
        .iter()
        .filter_map(|(_, kind)| {
            if let SymbolKind::Gear(ratio) = kind {
                Some(ratio)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(solve_part_1(input.lines()), 4361);
    }

    #[test]
    fn part_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(solve_part_2(input.lines()), 467835);
    }
}
