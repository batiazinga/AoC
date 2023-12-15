use aoc2023::rocks::Dish;

fn main() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let d = Dish::parse(&input);
    let c1 = d.cycle();
    println!("{}", c1);
    println!("");
    let c2 = c1.cycle();
    println!("{}", c2);
    println!("");
    let c3 = c2.cycle();
    println!("{}", c3);
}
