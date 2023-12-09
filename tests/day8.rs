use aoc2023::desert::count_steps_from_a_to_z;
use aoc2023::desert::count_steps_from_aaa_to_zzz;
use aoc2023::desert::read_map;
use std::fs;

#[test]
fn day_8_1() {
    let input = fs::read_to_string("data/day8.txt").unwrap();
    let (instructions, map, _) = read_map(&input);
    assert_eq!(
        count_steps_from_aaa_to_zzz(instructions.as_slice(), &map),
        13939
    );
}

#[test]
fn day_8_2() {
    let input = fs::read_to_string("data/day8.txt").unwrap();
    let (instructions, map, a_nodes) = read_map(&input);
    assert_eq!(
        count_steps_from_a_to_z(instructions.as_slice(), &map, a_nodes.as_slice()),
        8906539031197
    );
}
