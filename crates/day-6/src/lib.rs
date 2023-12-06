fn solve(time: u64, distance: u64) -> u64 {
    let mut result = 0;
    let mut mid_point = time / 2;

    while mid_point * (time - mid_point) > distance {
        mid_point -= 1;
        result += 1;
    }

    if time % 2 == 0 {
        result = (result * 2) - 1;
    } else {
        result *= 2;
    }

    result
}

pub fn parse_times_and_distances(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut lines = input.lines();

    let line = lines.next().unwrap();
    let times = line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|n| n.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let line = lines.next().unwrap();
    let distances = line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|n| n.parse::<u64>().ok())
        .collect::<Vec<_>>();

    (times, distances)
}

pub fn solve_part_1(input: &str) -> u64 {
    let (times, distances) = parse_times_and_distances(input);

    let mut result = 1;
    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
        result *= solve(time, distance);
    }

    result
}

pub fn solve_part_2(input: &str) -> u64 {
    let mut lines = input.lines();

    let line = lines.next().unwrap();
    let time = line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let line = lines.next().unwrap();
    let distance = line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    solve(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(input()), 288);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(input()), 71503);
    }

    fn input() -> &'static str {
        "Time:      7  15   30
Distance:  9  40  200"
    }
}
