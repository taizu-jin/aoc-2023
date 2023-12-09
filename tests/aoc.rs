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

#[test]
fn day_4_part_2() {
    let input = read_to_string("input/day-4").expect("file present");
    let result = day_4::solve_part_2(input.lines());

    assert_eq!(result, 23806951);
}

#[test]
fn day_5_part_1() {
    let input = read_to_string("input/day-5").expect("file present");
    let result = day_5::solve_part_1(&input);

    assert_eq!(result, 157211394);
}

#[test]
fn day_5_part_2() {
    let input = read_to_string("input/day-5").expect("file present");
    let result = day_5::solve_part_2(&input);

    assert_eq!(result, 50855035);
}

#[test]
fn day_6_part_1() {
    let input = read_to_string("input/day-6").expect("file present");
    let result = day_6::solve_part_1(&input);

    assert_eq!(result, 32076);
}

#[test]
fn day_6_part_2() {
    let input = read_to_string("input/day-6").expect("file present");
    let result = day_6::solve_part_2(&input);

    assert_eq!(result, 34278221);
}

#[test]
fn day_7_part_1() {
    let input = read_to_string("input/day-7").expect("file present");
    let result = day_7::solve_part_1(&input);

    assert_eq!(result, 251806792);
}

#[test]
fn day_7_part_2() {
    let input = read_to_string("input/day-7").expect("file present");
    let result = day_7::solve_part_2(&input);

    assert_eq!(result, 252113488);
}

#[test]
fn day_8_part_1() {
    let input = read_to_string("input/day-8").expect("file present");
    let result = day_8::solve_part_1(&input);

    assert_eq!(result, 22357);
}

#[test]
fn day_8_part_2() {
    let input = read_to_string("input/day-8").expect("file present");
    let result = day_8::solve_part_2(&input);

    assert_eq!(result, 10371555451871);
}

#[test]
fn day_9_part_1() {
    let input = read_to_string("input/day-9").expect("file present");
    let result = day_9::solve_part_1(&input);

    assert_eq!(result, 1725987467);
}
