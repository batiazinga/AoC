use aoc2023::pipe::{count_enclosed_tiles, loop_size, TileMap};
use std::fs;

#[test]
fn day_7_1() {
    let input = fs::read_to_string("data/day10.txt").unwrap();
    let map = TileMap::parse(&input);
    assert_eq!(loop_size(&map) / 2, 6773);
}

#[test]
fn day_7_2() {
    let input = fs::read_to_string("data/day10.txt").unwrap();
    let map = TileMap::parse(&input);
    let (count, _) = count_enclosed_tiles(&map);
    assert_eq!(count, 493);
}
