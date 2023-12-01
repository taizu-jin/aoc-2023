pub fn solve(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let first_digit = line.chars().find(|c| c.is_numeric());
        let last_digit = line.chars().rev().find(|c| c.is_numeric());

        if let (Some(first_digit), Some(last_digit)) = (first_digit, last_digit) {
            result += format!("{}{}", first_digit, last_digit)
                .parse::<u32>()
                .unwrap();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(solve(input), 142);
    }
}
