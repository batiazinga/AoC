use aoc2023::camelcard::read_bids;
use aoc2023::camelcard::total_winnings;
use aoc2023::camelcard2::read_bids2;
use aoc2023::camelcard2::total_winnings2;
use std::fs;

#[test]
fn day_7_1() {
    let input = fs::read_to_string("data/day7.txt").unwrap();
    let mut bids = read_bids(&input);
    assert_eq!(total_winnings(&mut bids), 250120186);
}

#[test]
fn day_7_2() {
    let input = fs::read_to_string("data/day7.txt").unwrap();
    let mut bids = read_bids2(&input);
    assert_eq!(total_winnings2(&mut bids), 250665248);
}
