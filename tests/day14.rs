use aoc2023::rocks::Dish;
use std::fs;

#[test]
fn day_14_1() {
    let input = fs::read_to_string("data/day14.txt").unwrap();
    let d = Dish::parse(&input);
    assert_eq!(d.load_after_one_slide_north_fast(), 109755);
}

#[test]
fn day_14_2() {
    let input = fs::read_to_string("data/day14.txt").unwrap();
    let d = Dish::parse(&input);
    assert_eq!(d.load_after_cycles_fast(1_000_000_000), 90928);
}
