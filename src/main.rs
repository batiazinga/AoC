use aoc2023::hotsprings::read_records;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("data/day12.txt").unwrap();

    let mut line = 0usize;
    let now = Instant::now();
    for record in read_records(&input) {
        line += 1;
        let count = record.unfold(4).count_arrangements();
        println!("line {}: {}", line, count);
    }
    let elapsed = now.elapsed();
    println!("elapsed time is {}ms", elapsed.as_millis());
}
