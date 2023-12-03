use aoc2023::sum_codes;
use std::fs;

#[test]
fn day_1() {
    let msg = fs::read_to_string("data/day1.txt").unwrap();
    assert_eq!(sum_codes(&msg),54019);
}
