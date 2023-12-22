use aoc2023::lavaduct::dig;
use aoc2023::lavaduct::read_dig_plan;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day18.txt").unwrap();
    let instructions = read_dig_plan(&input);
    let dig_map = dig(instructions.as_slice());
    println!("{}", dig_map)
}
