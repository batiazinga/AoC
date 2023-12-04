use aoc2023::scratchcard::Card;
use aoc2023::scratchcard::num_cards;
use std::fs;

#[test]
fn day_4_1() {
    let msg = fs::read_to_string("data/day4.txt").unwrap();
    let sum: u32 = msg.lines().map(|line| Card::parse(&line).value()).sum();
    assert_eq!(sum, 20117);
}

#[test]
fn day_4_2() {
    let msg = fs::read_to_string("data/day4.txt").unwrap();
    let num = num_cards(msg.lines().map(|line| Card::parse(&line)));
    assert_eq!(num, 13768818);
}
