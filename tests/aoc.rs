use std::fs::read_to_string;

#[test]
fn day_1_part_1() {
    let input = read_to_string("input/day-1").expect("file present");
    let result = day_1::solve_part_1(&input);

    assert_eq!(result, 54644);
}

#[test]
fn day_1_part_2() {
    let input = read_to_string("input/day-1").expect("file present");
    let result = day_1::solve_part_2(&input);

    assert_eq!(result, 53348);
}

#[test]
fn day_2_part_1() {
    let input = read_to_string("input/day-2").expect("file present");
    let result = day_2::solve_part_1(&input);

    assert_eq!(result, 0);
}
