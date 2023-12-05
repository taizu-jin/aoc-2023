#[derive(Debug)]
struct Range {
    destination: std::ops::Range<u64>,
    source: std::ops::Range<u64>,
}

impl Range {
    fn destination(&self, source: u64) -> Option<u64> {
        if !self.source.contains(&source) {
            return None;
        }

        let offset = source - self.source.start;
        Some(self.destination.start + offset)
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let mut input = value.split_whitespace();
        let destination = input
            .next()
            .expect("has destination value")
            .parse()
            .expect("destination is valid number");
        let source = input
            .next()
            .expect("has source value")
            .parse()
            .expect("source is valid number");
        let length = input
            .next()
            .expect("has length value")
            .parse::<u64>()
            .expect("length is valid number");

        let destination = std::ops::Range {
            start: destination,
            end: destination + length,
        };

        let source = std::ops::Range {
            start: source,
            end: source + length,
        };

        Self {
            destination,
            source,
        }
    }
}

#[derive(Debug)]
struct Map(Vec<Range>);

impl<'a, T> From<T> for Map
where
    T: Iterator<Item = &'a str>,
{
    fn from(value: T) -> Self {
        let mut ranges = Vec::new();
        for line in value {
            ranges.push(line.into());
        }

        Self(ranges)
    }
}

impl Map {
    fn destination(&self, source: u64) -> u64 {
        for range in &self.0 {
            if let Some(destination) = range.destination(source) {
                return destination;
            }
        }

        source
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl<'a, T> From<T> for Almanac
where
    T: Iterator<Item = &'a str>,
{
    fn from(mut value: T) -> Self {
        let seeds_raw = value
            .next()
            .expect("get seed line")
            .split_once(':')
            .expect("split seeds at ':'")
            .1
            .split_whitespace();

        let mut seeds = Vec::new();
        for seed in seeds_raw {
            seeds.push(seed.parse().expect("parsed seed number"));
        }

        let mut maps = Vec::new();
        for section in value {
            let mut lines = section.lines();
            lines.next(); // Skip label
            maps.push(Map::from(lines));
        }

        Self { seeds, maps }
    }
}

impl Almanac {
    fn find_lowest_location(&self) -> u64 {
        let mut location = u64::MAX;

        for seed in &self.seeds {
            let mut target = *seed;

            for map in &self.maps {
                target = map.destination(target);
            }

            if target < location {
                location = target
            }
        }

        location
    }
}

pub fn solve_part_1(input: &str) -> u64 {
    Almanac::from(input.split("\n\n")).find_lowest_location()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(input()), 35);
    }

    fn input() -> &'static str {
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
    }
}
