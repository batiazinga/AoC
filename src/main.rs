use aoc2023::scratchcard::Card;
use std::fs;

fn main() {
    let msg = fs::read_to_string("data/day4.txt").unwrap();
    let _sum: u32 = msg.lines().map(|line| Card::parse(&line).value()).sum();
}
