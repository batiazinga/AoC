use aoc2023::sum_min_cubes_powers;
use std::fs;

fn main() {
    let msg = fs::read_to_string("data/day2.txt").unwrap();
    let code = sum_min_cubes_powers(&msg);
    println!("{code}");
}
