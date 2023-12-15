use aoc2023::reflection::{summarize, fix_smudges_and_summarize};
use std::fs;

#[test]
fn day_13_1() {
    let input = fs::read_to_string("data/day13.txt").unwrap();
    assert_eq!(summarize(&input), 34918);
}

#[test]
fn day_13_2() {
    let input = fs::read_to_string("data/day13.txt").unwrap();
    assert_eq!(fix_smudges_and_summarize(&input), 33054);
}