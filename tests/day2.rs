use aoc2023::sum_game_ids;
use aoc2023::sum_min_cubes_powers;
use std::fs;

#[test]
fn day_2_1() {
    let msg = fs::read_to_string("data/day2.txt").unwrap();
    assert_eq!(sum_game_ids(&msg), 2600);
}

#[test]
fn day_2_2() {
    let msg = fs::read_to_string("data/day2.txt").unwrap();
    assert_eq!(sum_min_cubes_powers(&msg), 86036);
}
