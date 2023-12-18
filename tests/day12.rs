use aoc2023::hotsprings::ConditionRecord;
use std::fs;

#[test]
fn day_12_1() {
    let input = fs::read_to_string("data/day12.txt").unwrap();
    let mut sum = 0u64;
    for line in input.lines() {
        sum += ConditionRecord::parse(line).count_arrangements();
    }
    assert_eq!(sum, 7361);
}

// #[test]
// fn day_12_2() {
//     let input = fs::read_to_string("data/day12.txt").unwrap();
//     let mut sum = 0u64;
//     for line in input.lines() {
//         sum += ConditionRecord::parse(line).unfold(5).count_arrangements();
//     }
//     assert_eq!(sum, 0);
// }