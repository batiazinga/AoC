use aoc2023::sum_codes;
use std::fs;

fn main() {
    let msg = fs::read_to_string("data/day1.txt").unwrap();
    let code = sum_codes(&msg);
    println!("{code}");
}
