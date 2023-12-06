pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn parse_times_and_distances(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut lines = input.lines();
    let line = lines.next().unwrap();

    let times = line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let line = lines.next().unwrap();
    let distances = line
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<_>>();

    (times, distances)
}

pub fn solve_part_1(input: &str) -> u32 {
    let (times, distances) = parse_times_and_distances(input);

    let mut result = 1;
    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
        let mut mid_point = time / 2;

        let mut res = 0;
        while mid_point * (time - mid_point) > distance {
            mid_point -= 1;
            res += 1;
        }

        if time % 2 == 0 {
            res = (res * 2) - 1;
        } else {
            res *= 2;
        }

        if res != 0 {
            result *= res;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(input()), 288);
    }

    fn input() -> &'static str {
        "Time:      7  15   30
Distance:  9  40  200"
    }
}
