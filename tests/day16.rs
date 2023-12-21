use aoc2023::beam_tracer::Contraption;
use std::fs;

#[test]
fn day_16_1() {
    let input = fs::read_to_string("data/day16.txt").unwrap();
    let c = Contraption::parse(&input);
    assert_eq!(c.trace_beam().num_energized(), 7798);
}

#[test]
fn day_16_2() {
    let input = fs::read_to_string("data/day16.txt").unwrap();
    let c = Contraption::parse(&input);
    assert_eq!(c.max_energized(), 8026);
}
