use aoc2023::reflection::summarize;
use std::fs;

#[test]
fn day_12_1() {
    let input = fs::read_to_string("data/day13.txt").unwrap();
    assert_eq!(summarize(&input), 0);
}