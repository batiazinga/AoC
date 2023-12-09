use aoc2023::oasis::sum_extrapolated_next_values;
use aoc2023::oasis::sum_extrapolated_previous_values;
use std::fs;

#[test]
fn day_9_1() {
    let input = fs::read_to_string("data/day9.txt").unwrap();
    assert_eq!(sum_extrapolated_next_values(&input), 1934898178);
}

#[test]
fn day_9_2() {
    let input = fs::read_to_string("data/day9.txt").unwrap();
    assert_eq!(sum_extrapolated_previous_values(&input), 1129);
}
