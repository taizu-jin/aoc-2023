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

    assert_eq!(result, 2771);
}

#[test]
fn day_2_part_2() {
    let input = read_to_string("input/day-2").expect("file present");
    let result = day_2::solve_part_2(&input);

    assert_eq!(result, 70924);
}

#[test]
fn day_3_part_1() {
    let input = read_to_string("input/day-3").expect("file present");
    let result = day_3::solve_part_1(input.lines());

    assert_eq!(result, 535235);
}

#[test]
fn day_3_part_2() {
    let input = read_to_string("input/day-3").expect("file present");
    let result = day_3::solve_part_2(input.lines());

    assert_eq!(result, 79844424);
}

#[test]
fn day_4_part_1() {
    let input = read_to_string("input/day-4").expect("file present");
    let result = day_4::solve_part_1(input.lines());

    assert_eq!(result, 20407);
}
