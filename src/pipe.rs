use crate::grid2d::{Direction, Position};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Tile {
    Ground,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Tile::Ground),
            '|' => Ok(Tile::NorthSouth),
            '-' => Ok(Tile::EastWest),
            'L' => Ok(Tile::NorthEast),
            'J' => Ok(Tile::NorthWest),
            '7' => Ok(Tile::SouthWest),
            'F' => Ok(Tile::SouthEast),
            'S' => Ok(Tile::Start),
            _ => Err("invalid pipe character"),
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Ground => '.',
            Tile::NorthSouth => '|',
            Tile::EastWest => '-',
            Tile::NorthEast => 'L',
            Tile::NorthWest => 'J',
            Tile::SouthWest => '7',
            Tile::SouthEast => 'F',
            Tile::Start => 'S',
        }
    }
}

pub struct TileMap {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
    size: (usize, usize),
}

impl TileMap {
    pub fn parse(input: &str) -> TileMap {
        let mut m = TileMap {
            tiles: Vec::new(),
            start: (0, 0),
            size: (0, 0),
        };

        let mut row = 0usize;
        let mut column = 0usize;
        for line in input.lines() {
            let mut pipe_line: Vec<Tile> = Vec::new();
            for c in line.chars() {
                let p = Tile::try_from(c).unwrap();
                if p == Tile::Start {
                    m.start = (row, column);
                }
                pipe_line.push(Tile::try_from(c).unwrap());
                column += 1;
            }
            m.tiles.push(pipe_line);
            row += 1;
            column = 0;
        }

        m.size = (m.tiles.len(), m.tiles[0].len());
        m
    }

    fn get(&self, p: Position) -> Tile {
        self.tiles[p.row()][p.col()]
    }

    fn start(&self) -> Position {
        Position::new(self.start.0, self.start.1, self.size)
    }

    fn start_tile(&self) -> Tile {
        let mut north = false;
        if self.start.0 > 0 {
            let north_tile = &self.tiles[self.start.0 - 1][self.start.1];
            north = *north_tile == Tile::NorthSouth
                || *north_tile == Tile::SouthEast
                || *north_tile == Tile::SouthWest;
        }

        let mut east = false;
        if self.start.1 < self.size.1 - 1 {
            let east_tile = &self.tiles[self.start.0][self.start.1 + 1];
            east = *east_tile == Tile::EastWest
                || *east_tile == Tile::NorthWest
                || *east_tile == Tile::SouthWest;
        }

        let mut south = false;
        if self.start.0 < self.size.0 - 1 {
            let south_tile = &self.tiles[self.start.0 + 1][self.start.1];
            south = *south_tile == Tile::NorthEast
                || *south_tile == Tile::NorthSouth
                || *south_tile == Tile::NorthWest;
        }

        let mut west = false;
        if self.start.1 > 0 {
            let west_tile = &self.tiles[self.start.0][self.start.1 - 1];
            west = *west_tile == Tile::SouthEast
                || *west_tile == Tile::EastWest
                || *west_tile == Tile::NorthEast;
        }

        if north && east {
            return Tile::NorthEast;
        }
        if north && south {
            return Tile::NorthSouth;
        }
        if north && west {
            return Tile::NorthWest;
        }
        if south && east {
            return Tile::SouthEast;
        }
        if south && west {
            return Tile::SouthWest;
        }
        Tile::EastWest
    }
}

fn next_position_from_start(map: &TileMap) -> Position {
    if let Some(next) = map.start().to(Direction::North) {
        let next_tile = map.get(next);
        if next_tile == Tile::SouthEast
            || next_tile == Tile::NorthSouth
            || next_tile == Tile::SouthWest
        {
            return next;
        }
    }

    if let Some(next) = map.start().to(Direction::East) {
        let next_tile = map.get(next);
        if next_tile == Tile::NorthWest
            || next_tile == Tile::EastWest
            || next_tile == Tile::SouthWest
        {
            return next;
        }
    }

    if let Some(next) = map.start().to(Direction::South) {
        let next_tile = map.get(next);
        if next_tile == Tile::NorthEast
            || next_tile == Tile::NorthSouth
            || next_tile == Tile::NorthWest
        {
            return next;
        }
    }

    if let Some(next) = map.start().to(Direction::West) {
        let next_tile = map.get(next);
        if next_tile == Tile::SouthEast
            || next_tile == Tile::EastWest
            || next_tile == Tile::NorthEast
        {
            return next;
        }
    }

    panic!("cannot leave starting point");
}

fn next_position(map: &TileMap, from: Position, position: Position) -> Position {
    let candidates = match map.get(position) {
        Tile::NorthSouth => (
            position.to(Direction::North).unwrap(),
            position.to(Direction::South).unwrap(),
        ),
        Tile::SouthEast => (
            position.to(Direction::South).unwrap(),
            position.to(Direction::East).unwrap(),
        ),
        Tile::NorthWest => (
            position.to(Direction::North).unwrap(),
            position.to(Direction::West).unwrap(),
        ),
        Tile::SouthWest => (
            position.to(Direction::South).unwrap(),
            position.to(Direction::West).unwrap(),
        ),
        Tile::NorthEast => (
            position.to(Direction::North).unwrap(),
            position.to(Direction::East).unwrap(),
        ),
        Tile::EastWest => (
            position.to(Direction::East).unwrap(),
            position.to(Direction::West).unwrap(),
        ),
        // Tile::Start => (next, next),
        _ => panic!("unexpected pipe"),
    };

    let mut next = candidates.0;
    if next == from {
        next = candidates.1;
    }

    next
}

pub fn loop_size(map: &TileMap) -> u64 {
    let mut distance = 0u64;

    let mut previous = map.start();
    let mut position = next_position_from_start(&map);
    distance += 1;

    while position != map.start() {
        let tmp = position;
        position = next_position(&map, previous, position);
        previous = tmp;
        distance += 1;
    }

    distance
}

pub fn collect_boundary(map: &TileMap) -> Vec<Vec<Option<Tile>>> {
    let mut boundary: Vec<Vec<Option<Tile>>> = Vec::with_capacity(map.size.0);
    for _ in 0..map.size.0 {
        let mut row: Vec<Option<Tile>> = Vec::with_capacity(map.size.1);
        for _ in 0..map.size.1 {
            row.push(None);
        }
        boundary.push(row);
    }

    let mut from = map.start();
    let mut position = next_position_from_start(&map);
    boundary[position.row()][position.col()] = Some(map.get(position).clone());

    while position != map.start() {
        let tmp = position;
        position = next_position(&map, from, position);
        from = tmp;
        boundary[position.row()][position.col()] = Some(map.get(position));
    }

    let start_tile = map.start_tile();
    boundary[map.start.0][map.start.1] = Some(start_tile);

    boundary
}

pub fn count_enclosed_tiles(map: &TileMap) -> (u64, Vec<Vec<Option<Tile>>>) {
    let boundary = collect_boundary(&map);

    let mut debug: Vec<Vec<Option<Tile>>> = Vec::with_capacity(boundary.len());
    for i in 0..boundary.len() {
        let mut row = Vec::with_capacity(boundary[0].len());
        for j in 0..boundary[0].len() {
            row.push(boundary[i][j]);
        }
        debug.push(row);
    }

    let mut count = 0u64;

    for row in 0..map.size.0 {
        let mut enclosed = false;
        for column in 0..map.size.1 {
            let tile = &boundary[row][column];
            if *tile == Some(Tile::SouthWest)
                || *tile == Some(Tile::NorthSouth)
                || *tile == Some(Tile::SouthEast)
            {
                if enclosed {
                    enclosed = false;
                } else {
                    enclosed = true;
                }
                continue;
            }
            if tile.is_some() {
                continue;
            }
            if enclosed {
                count += 1;
                debug[row][column] = Some(Tile::Ground);
            }
        }
    }

    (count, debug)
}

pub fn print_boundary(boundary: &Vec<Vec<Option<Tile>>>) {
    let mut s = String::new();

    for i in 0..boundary.len() {
        for j in 0..boundary[0].len() {
            match boundary[i][j] {
                None => s.push(' '),
                Some(i) => s.push(i.into()),
            }
        }
        s.push('\n');
    }

    println!("{}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_loop_size() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let map = TileMap::parse(&input);
        assert_eq!(loop_size(&map), 8);
    }

    #[test]
    fn test_complex_loop_size() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let map = TileMap::parse(&input);
        assert_eq!(loop_size(&map), 16);
    }

    #[test]
    fn test_start_tile() {
        let mut input = ".F7\n.S|\n.LJ";
        let mut a = TileMap::parse(&input);
        assert_eq!(a.start_tile(), Tile::NorthSouth);

        input = "FS7\nL-J";
        a = TileMap::parse(&input);
        assert_eq!(a.start_tile(), Tile::EastWest);

        input = "F7\nSJ";
        a = TileMap::parse(&input);
        assert_eq!(a.start_tile(), Tile::NorthEast);

        input = "F7\nLS";
        a = TileMap::parse(&input);
        assert_eq!(a.start_tile(), Tile::NorthWest);
    }

    #[test]
    fn test_count_enclosed_tiles_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let map = TileMap::parse(&input);
        let (count, _) = count_enclosed_tiles(&map);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_count_enclosed_tiles_2() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        let map = TileMap::parse(&input);
        let (count, _) = count_enclosed_tiles(&map);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_count_enclosed_tiles_3() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let map = TileMap::parse(&input);
        let (count, _) = count_enclosed_tiles(&map);
        assert_eq!(count, 8);
    }

    #[test]
    fn test_count_enclosed_tiles_4() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let map = TileMap::parse(&input);
        let (count, _) = count_enclosed_tiles(&map);
        assert_eq!(count, 10);
    }
}
