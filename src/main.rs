use aoc2023::hotsprings::ConditionRecord;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day12.txt").unwrap();
    let mut sum = 0u64;
    for line in input.lines() {
        let count = ConditionRecord::parse(line).unfold(2).count_arrangements();
        sum += count;
        println!("{}", count);
    }
    println!("{}", sum);
}
