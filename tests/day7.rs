use aoc2023::camelcard::read_bids;
use aoc2023::camelcard::total_winnings;
use std::fs;

#[test]
fn day_7_1() {
    let input = fs::read_to_string("data/day7.txt").unwrap();
    let mut bids = read_bids(&input);
    assert_eq!(total_winnings(&mut bids), 250120186);
}

// #[test]
// fn day_7_2() {
//     let input = fs::read_to_string("data/day7.txt").unwrap();
// }
