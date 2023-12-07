use aoc2023::almanac::Almanac;
use aoc2023::almanac::min_location_ex1;
use std::fs;

#[test]
fn day_5_1() {
    let input = fs::read_to_string("data/day5.txt").unwrap();
    let min = min_location_ex1(&Almanac::parse(&input));
    assert_eq!(min, 51580674);
}

