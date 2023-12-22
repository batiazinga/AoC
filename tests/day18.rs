use aoc2023::lavaduct::dig;
use aoc2023::lavaduct::read_dig_plan;
use std::fs;

#[test]
fn day_18_1() {
    let input = fs::read_to_string("data/day18.txt").unwrap();
    let instructions = read_dig_plan(&input);
    assert_eq!(dig(instructions.as_slice()).volume(), 45159);
}
