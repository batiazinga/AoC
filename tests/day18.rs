use aoc2023::lavaduct::dig;
use aoc2023::lavaduct::read_dig_plan;
use aoc2023::lavaduct::read_dig_plan_correctly;
use aoc2023::lavaduct::dug_volume;
use std::fs;

#[test]
fn day_18_1() {
    let input = fs::read_to_string("data/day18.txt").unwrap();
    let instructions = read_dig_plan(&input);
    assert_eq!(dig(instructions.as_slice()).volume(), 45159);
}

#[test]
fn day_18_2() {
    let input = fs::read_to_string("data/day18.txt").unwrap();
    let instructions = read_dig_plan_correctly(&input);
    assert_eq!(dug_volume(instructions.as_slice()), 134549294799713);
}