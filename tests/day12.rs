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

// with unfold(4), roughly 30mn
// long lines: 347, 462, 772, 808

// #[test]
// fn day_12_2() {
//     let input = fs::read_to_string("data/day12.txt").unwrap();
//     let mut sum = 0u64;
//     for record in read_records(&input) {
//         sum += record.unfold(5).count_arrangements();
//     }
//     assert_eq!(sum, 0);
// }
