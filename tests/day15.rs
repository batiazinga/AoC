use aoc2023::hash::{sum_hash, focusing_power};
use std::fs;

#[test]
fn day_15_1() {
    let input = fs::read_to_string("data/day15.txt").unwrap();
    assert_eq!(sum_hash(&input), 510792);
}

#[test]
fn day_15_2() {
    let input = fs::read_to_string("data/day15.txt").unwrap();
    assert_eq!(focusing_power(&input), 269410);
}
