use aoc2023::calibration::sum;
use std::fs;

#[test]
fn day_1() {
    let msg = fs::read_to_string("data/day1.txt").unwrap();
    assert_eq!(sum(&msg),54019);
}
