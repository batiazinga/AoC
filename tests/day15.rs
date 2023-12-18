use aoc2023::hash::sum_hash;
use std::fs;

#[test]
fn day_15_1() {
    let input = fs::read_to_string("data/day15.txt").unwrap();
    assert_eq!(sum_hash(&input), 510792);
}
