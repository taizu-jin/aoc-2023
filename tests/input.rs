use std::fs::read_to_string;

#[test]
fn day_1_part_1() {
    let input = read_to_string("input/day-1").expect("file present");
    let result = day_1::solve(&input);

    assert_eq!(result, 0);
}
