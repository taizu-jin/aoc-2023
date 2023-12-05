use std::marker::PhantomData;

trait FromStrToVec
where
    Self: Sized,
{
    fn to_vec(value: &str) -> Vec<Self>;
}

struct SingleSeed(u64);

impl From<&str> for SingleSeed {
    fn from(value: &str) -> Self {
        Self(value.parse().expect("parse single seed to number"))
    }
}

impl FromStrToVec for SingleSeed {
    fn to_vec(value: &str) -> Vec<Self> {
        let mut vec = Vec::new();
        for seeds in value.split_whitespace() {
            vec.push(Self::from(seeds));
        }
        vec
    }
}

struct SeedRange(std::ops::Range<u64>);

impl FromStrToVec for SeedRange {
    fn to_vec(value: &str) -> Vec<Self> {
        let mut vec = Vec::new();
        for chunk in value.split_whitespace().collect::<Vec<&str>>().chunks(2) {
            let start = chunk[0].parse().expect("parse seed start to number");
            let length = chunk[1]
                .parse::<u64>()
                .expect("parse seed length to number");
            vec.push(Self(std::ops::Range {
                start,
                end: start + length,
            }))
        }
        vec
    }
}

#[derive(Debug)]
struct Range<T> {
    destination: std::ops::Range<u64>,
    source: std::ops::Range<u64>,
    _marker: PhantomData<T>,
}

impl Range<SingleSeed> {
    fn destination(&self, source: u64) -> Option<u64> {
        if !self.source.contains(&source) {
            return None;
        }

        let offset = source - self.source.start;
        Some(self.destination.start + offset)
    }
}

impl Range<SeedRange> {
    /// Returns destination range if found and ranges that did not fit.
    fn destination(
        &self,
        source: std::ops::Range<u64>,
    ) -> (Option<std::ops::Range<u64>>, Vec<std::ops::Range<u64>>) {
        if !self.source.contains(&source.start) && !self.source.contains(&(source.end)) {
            return (None, vec![source]);
        }

        let mut leftovers = Vec::new();

        let mut offest = 0;
        if source.start < self.source.start {
            leftovers.push(std::ops::Range {
                start: source.start,
                end: self.source.start,
            })
        } else {
            offest = source.start - self.source.start;
        }

        let length = if source.end > self.source.end {
            leftovers.push(std::ops::Range {
                start: self.source.end,
                end: source.end,
            });
            self.source.end - self.source.start - offest
        } else {
            source.end - self.source.start - offest
        };

        (
            Some(std::ops::Range {
                start: self.destination.start + offest,
                end: self.destination.start + offest + length,
            }),
            leftovers,
        )
    }
}

impl<T> From<&str> for Range<T> {
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
            _marker: PhantomData,
        }
    }
}

#[derive(Debug)]
struct Map<T> {
    ranges: Vec<Range<T>>,
    _marker: PhantomData<T>,
}

impl<'a, T, K> From<T> for Map<K>
where
    T: Iterator<Item = &'a str>,
{
    fn from(value: T) -> Self {
        let mut ranges = Vec::new();
        for line in value {
            ranges.push(line.into());
        }

        Self {
            ranges,
            _marker: PhantomData,
        }
    }
}

impl Map<SingleSeed> {
    fn destination(&self, source: u64) -> u64 {
        for range in &self.ranges {
            if let Some(destination) = range.destination(source) {
                return destination;
            }
        }

        source
    }
}

impl Map<SeedRange> {
    fn destinations(&self, source: std::ops::Range<u64>) -> Vec<std::ops::Range<u64>> {
        let mut sources = vec![source];
        let mut destinations = Vec::new();

        for range in &self.ranges {
            let mut leftovers = Vec::new();

            while let Some(source) = sources.pop() {
                {
                    let (dest, mut lo) = range.destination(source);
                    leftovers.append(&mut lo);

                    if let Some(dest) = dest {
                        destinations.push(dest);
                    }
                }
            }

            sources.append(&mut leftovers);
        }

        destinations.append(&mut sources);
        destinations
    }
}

#[derive(Debug)]
struct Almanac<T> {
    seeds: Vec<T>,
    maps: Vec<Map<T>>,
}

impl<'a, T, K> From<T> for Almanac<K>
where
    T: Iterator<Item = &'a str>,
    K: FromStrToVec,
{
    fn from(mut value: T) -> Self {
        let seeds_raw = value
            .next()
            .expect("get seed line")
            .split_once(':')
            .expect("split seeds at ':'")
            .1;

        let seeds = K::to_vec(seeds_raw);

        let mut maps = Vec::new();
        for section in value {
            let mut lines = section.lines();
            lines.next(); // Skip label
            maps.push(Map::from(lines));
        }

        Self { seeds, maps }
    }
}

impl Almanac<SingleSeed> {
    fn find_lowest_location(&self) -> u64 {
        let mut location = u64::MAX;

        for seed in &self.seeds {
            let mut target = seed.0;

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

impl Almanac<SeedRange> {
    fn find_lowest_location(self) -> u64 {
        let mut location = u64::MAX;

        for range in self.seeds {
            let mut ranges = vec![range.0];

            for map in &self.maps {
                let mut destinations = Vec::new();

                while let Some(range) = ranges.pop() {
                    destinations.append(&mut map.destinations(range));
                }

                ranges.append(&mut destinations);
            }

            if let Some(lowest_range) = ranges.iter().min_by(|l, r| l.start.cmp(&r.start)) {
                if lowest_range.start < location {
                    location = lowest_range.start
                }
            }
        }

        location
    }
}

pub fn solve_part_1(input: &str) -> u64 {
    Almanac::<SingleSeed>::from(input.split("\n\n")).find_lowest_location()
}

pub fn solve_part_2(input: &str) -> u64 {
    Almanac::<SeedRange>::from(input.split("\n\n")).find_lowest_location()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(input()), 35);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(input()), 46);
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
