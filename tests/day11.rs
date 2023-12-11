use aoc2023::observatory::SpaceMap;
use std::fs;

#[test]
fn day_11_1() {
    let input = fs::read_to_string("data/day11.txt").unwrap();
    let m = SpaceMap::parse(&input);
    assert_eq!(m.sum_galaxy_pair_distances(2), 9329143);
}

#[test]
fn day_11_2() {
    let input = fs::read_to_string("data/day11.txt").unwrap();
    let m = SpaceMap::parse(&input);
    assert_eq!(m.sum_galaxy_pair_distances(1_000_000), 710674907809);
}
