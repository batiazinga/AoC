use aoc2023::race::prod_num_possibilities;
use aoc2023::race::num_possibilities;
use std::fs;

#[test]
fn day_6_1() {
    let input = fs::read_to_string("data/day6.txt").unwrap();
    assert_eq!(prod_num_possibilities(&input), 393120);
}

#[test]
fn day_6_2() {
    let input = fs::read_to_string("data/day6.txt").unwrap();
    assert_eq!(num_possibilities(&input), 36872656);
}