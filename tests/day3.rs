use aoc2023::EngineSchematic;
use std::fs;

#[test]
fn day_3_1() {
    let msg = fs::read_to_string("data/day3.txt").unwrap();
    let sum: u32 = EngineSchematic::parse(&msg).part_numbers().sum();
    assert_eq!(sum, 528799);
}

#[test]
fn day_3_2() {
    let msg = fs::read_to_string("data/day3.txt").unwrap();
    let sum: u32 = EngineSchematic::parse(&msg).gear_ratios().sum();
    assert_eq!(sum, 84907174);
}
