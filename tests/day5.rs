use aoc2023::almanac::min_location_ex1;
use aoc2023::almanac::min_location_ex2;
use aoc2023::almanac::Almanac;
use std::fs;

#[test]
fn day_5_1() {
    let input = fs::read_to_string("data/day5.txt").unwrap();
    let min = min_location_ex1(&Almanac::parse(&input));
    assert_eq!(min, 51580674);
}

#[test]
fn day_5_2() {
    let input = fs::read_to_string("data/day5.txt").unwrap();
    let min = min_location_ex2(&Almanac::parse(&input));
    assert_eq!(min, 0);
}
