use aoc2023::sandslabs::count_disintegrable_bricks;
use aoc2023::sandslabs::sum_chain_reactions;
use aoc2023::sandslabs::read_bricks;
use std::fs;

#[test]
fn day_22_1() {
    let input = fs::read_to_string("data/day22.txt").unwrap();
    let bricks = read_bricks(&input);
    assert_eq!(count_disintegrable_bricks(bricks.as_slice()), 413);
}

#[test]
fn day_22_2() {
    let input = fs::read_to_string("data/day22.txt").unwrap();
    let bricks = read_bricks(&input);
    assert_eq!(sum_chain_reactions(bricks.as_slice()), 41610);
}
