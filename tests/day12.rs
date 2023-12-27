use aoc2023::hotsprings::read_records;
use std::fs;

#[test]
fn day_12_1() {
    let input = fs::read_to_string("data/day12.txt").unwrap();
    let mut sum = 0u64;
    for record in read_records(&input) {
        sum += record.count_arrangements();
    }
    assert_eq!(sum, 7361);
}

// #[test]
// fn day_12_2() {
//     let input = fs::read_to_string("data/day12.txt").unwrap();
//     let mut sum = 0u64;
//     for record in read_records(&input) {
//         sum += record.unfold(3).count_arrangements();
//     }
//     assert_eq!(sum, 0);
// }
