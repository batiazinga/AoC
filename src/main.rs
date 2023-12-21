use aoc2023::heatloss::LossMap;

fn main() {
    let m = LossMap::parse("21111111\n22288222");
    // let m = LossMap::parse("211\n222");
    println!("{}", m);
    println!("shortest path distance = {}",m.shortest_path());
}
