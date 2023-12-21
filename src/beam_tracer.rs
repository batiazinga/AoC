use crate::grid2d::Direction;
use crate::grid2d::Position;
use std::collections::HashSet;
use std::fmt;

pub struct EnergyMap {
    content: Vec<Vec<bool>>,
    num_rows: usize,
    num_cols: usize,
}

impl EnergyMap {
    fn new(num_rows: usize, num_cols: usize) -> EnergyMap {
        EnergyMap {
            content: vec![vec![false; num_cols]; num_rows],
            num_rows,
            num_cols,
        }
    }
    pub fn num_energized(&self) -> u64 {
        let mut count = 0u64;

        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                if self.content[i][j] {
                    count += 1;
                }
            }
        }

        count
    }

    fn energize(&mut self, p: &Position) {
        self.content[p.row()][p.col()] = true;
    }
}

impl fmt::Display for EnergyMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.num_rows == 0 {
            return write!(f, "");
        }

        let mut s = String::with_capacity(self.num_rows * (self.num_cols + 1) - 1);
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                s.push(if self.content[i][j] { '#' } else { '.' });
            }
            if i < self.num_rows - 1 {
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Device {
    NorthWestMirror,
    NorthEastMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl TryFrom<char> for Device {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '\\' => Ok(Device::NorthWestMirror),
            '/' => Ok(Device::NorthEastMirror),
            '-' => Ok(Device::HorizontalSplitter),
            '|' => Ok(Device::VerticalSplitter),
            _ => Err("invalid device character"),
        }
    }
}

impl Into<char> for &Device {
    fn into(self) -> char {
        match self {
            Device::NorthWestMirror => '\\',
            Device::NorthEastMirror => '/',
            Device::HorizontalSplitter => '-',
            Device::VerticalSplitter => '|',
        }
    }
}

pub struct Contraption {
    content: Vec<Vec<Option<Device>>>,
    num_rows: usize,
    num_cols: usize,
}

impl Contraption {
    pub fn parse(input: &str) -> Contraption {
        let mut c = Contraption {
            content: Vec::new(),
            num_rows: 0,
            num_cols: 0,
        };

        let mut is_first_line: bool = true;
        for line in input.lines() {
            if is_first_line {
                let mut row: Vec<Option<Device>> = Vec::new();
                for c in line.chars() {
                    row.push(match Device::try_from(c) {
                        Ok(device) => Some(device),
                        Err(_) => None,
                    });
                }
                c.num_cols = row.len();
                c.content.push(row);
                is_first_line = false;
                continue;
            }

            let mut row: Vec<Option<Device>> = Vec::with_capacity(c.num_cols);
            for c in line.chars() {
                row.push(match Device::try_from(c) {
                    Ok(device) => Some(device),
                    Err(_) => None,
                });
            }
            c.content.push(row);
        }
        c.num_rows = c.content.len();

        c
    }

    fn get(&self, p: &Position) -> Option<Device> {
        self.content[p.row()][p.col()]
    }

    pub fn trace_beam(&self) -> EnergyMap {
        self.trace_beam_from(
            Position::new(0, 0, (self.num_rows, self.num_cols)),
            Direction::East,
        )
    }

    fn trace_beam_from(&self, pos: Position, incr: Direction) -> EnergyMap {
        let mut m = EnergyMap::new(self.num_rows, self.num_cols);

        let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
        self.rec_trace_beam(&mut m, &mut visited, pos, incr);

        m
    }

    fn rec_trace_beam(
        &self,
        m: &mut EnergyMap,
        visited: &mut HashSet<(usize, usize, Direction)>,
        pos: Position,
        incr: Direction,
    ) {
        let key = (pos.row(), pos.col(), incr);
        if visited.contains(&key) {
            return;
        }
        visited.insert(key);

        let mut position = pos;
        let mut increment = incr;
        loop {
            m.energize(&position);

            if let Some(device) = self.get(&position) {
                match device {
                    Device::HorizontalSplitter => {
                        if increment.is_south_north() {
                            self.rec_trace_beam(m, visited, position, Direction::East);
                            self.rec_trace_beam(m, visited, position, Direction::West);
                            break;
                        }
                    }
                    Device::VerticalSplitter => {
                        if increment.is_east_west() {
                            self.rec_trace_beam(m, visited, position, Direction::North);
                            self.rec_trace_beam(m, visited, position, Direction::South);
                            break;
                        }
                    }
                    Device::NorthWestMirror => {
                        increment = match increment {
                            Direction::East => Direction::South,
                            Direction::South => Direction::East,
                            Direction::West => Direction::North,
                            Direction::North => Direction::West,
                        }
                    }
                    Device::NorthEastMirror => {
                        increment = match increment {
                            Direction::East => Direction::North,
                            Direction::North => Direction::East,
                            Direction::West => Direction::South,
                            Direction::South => Direction::West,
                        }
                    }
                }
            }

            if let Some(next) = position.to(increment) {
                position = next;
            } else {
                break;
            }
        }
    }

    pub fn max_energized(&self) -> u64 {
        let mut max = 0u64;

        for j in 0..self.num_cols {
            let count = self
                .trace_beam_from(
                    Position::new(0, j, (self.num_rows, self.num_cols)),
                    Direction::South,
                )
                .num_energized();
            if count > max {
                max = count;
            }
        }
        for j in 0..self.num_cols {
            let count = self
                .trace_beam_from(
                    Position::new(self.num_rows - 1, j, (self.num_rows, self.num_cols)),
                    Direction::North,
                )
                .num_energized();
            if count > max {
                max = count;
            }
        }
        for i in 0..self.num_rows {
            let count = self
                .trace_beam_from(
                    Position::new(i, 0, (self.num_rows, self.num_cols)),
                    Direction::East,
                )
                .num_energized();
            if count > max {
                max = count;
            }
        }
        for i in 0..self.num_rows {
            let count = self
                .trace_beam_from(
                    Position::new(i, self.num_cols - 1, (self.num_rows, self.num_cols)),
                    Direction::West,
                )
                .num_energized();
            if count > max {
                max = count;
            }
        }

        max
    }
}

impl fmt::Display for Contraption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.num_rows == 0 {
            return write!(f, "");
        }

        let mut s = String::with_capacity(self.num_rows * (self.num_cols + 1) - 1);
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                s.push(match &self.content[i][j] {
                    None => '.',
                    Some(device) => device.into(),
                });
            }
            if i < self.num_rows - 1 {
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn test_parse_contraption() {
        let c = Contraption::parse(&INPUT);
        assert_eq!(format!("{}", c), INPUT);
    }

    #[test]
    fn test_trace_beam() {
        let c = Contraption::parse(&INPUT);
        assert_eq!(c.trace_beam().num_energized(), 46);
    }

    #[test]
    fn test_max_energized() {
        let c = Contraption::parse(&INPUT);
        assert_eq!(c.max_energized(), 51);
    }

    #[test]
    fn test_when_no_device_beam_is_not_altered() {
        let c = Contraption::parse("....\n....");
        let m = c.trace_beam();
        assert_eq!(format!("{}", m), "####\n....");
        assert_eq!(m.num_energized(), 4);
    }

    #[test]
    fn test_beam_is_reflected_downward() {
        let c = Contraption::parse("...\\\n....\n....");
        let m = c.trace_beam();
        assert_eq!(format!("{}", m), "####\n...#\n...#");
        assert_eq!(m.num_energized(), 6);
    }

    #[test]
    fn test_beam_is_reflected_downward_immediately() {
        let c = Contraption::parse("\\...\n....\n....");
        let m = c.trace_beam();
        assert_eq!(format!("{}", m), "#...\n#...\n#...");
        assert_eq!(m.num_energized(), 3);
    }

    #[test]
    fn test_beam_is_reflected_downward_then_leftward() {
        let c = Contraption::parse("...\\\n....\n.../");
        let m = c.trace_beam();
        assert_eq!(format!("{}", m), "####\n...#\n####");
        assert_eq!(m.num_energized(), 9);
    }

    #[test]
    fn test_beam_is_reflected_upward_immediately() {
        let c = Contraption::parse("/...\n....\n....");
        let m = c.trace_beam();
        assert_eq!(format!("{}", m), "#...\n....\n....");
        assert_eq!(m.num_energized(), 1);
    }

    #[test]
    fn test_beam_can_pass_several_times_on_same_tiles() {
        let c = Contraption::parse("...\\\n....\n\\../");
        let m = c.trace_beam();
        assert_eq!(format!("{}", m), "####\n#..#\n####");
        assert_eq!(m.num_energized(), 10);
    }

    #[test]
    fn test_horizontal_beam_is_not_split_by_horizontal_splitter() {
        let c = Contraption::parse(".-.\\\n....\n\\../");
        let m = c.trace_beam();
        assert_eq!(format!("{}", m), "####\n#..#\n####");
        assert_eq!(m.num_energized(), 10);
    }

    #[test]
    fn test_vertical_beam_is_split_by_horizontal_splitter() {
        let c = Contraption::parse("..\\.\n....\n..-.");
        let m = c.trace_beam();
        assert_eq!(format!("{}", m), "###.\n..#.\n####");
        assert_eq!(m.num_energized(), 8);
    }

    #[test]
    fn test_vertical_beam_is_not_split_by_vertical_splitter() {
        let c = Contraption::parse("...\\\n...|\n\\../");
        let m = c.trace_beam();
        assert_eq!(format!("{}", m), "####\n#..#\n####");
        assert_eq!(m.num_energized(), 10);
    }

    #[test]
    fn test_horizontal_beam_is_split_by_vertical_splitter() {
        let c = Contraption::parse("\\...\n\\.|.\n....");
        let m = c.trace_beam();
        assert_eq!(format!("{}", m), "#.#.\n###.\n..#.");
        assert_eq!(m.num_energized(), 6);
    }

    #[test]
    fn test_beam_does_not_loop_infinitely() {
        let c = Contraption::parse("-..\\\n....\n\\../");
        let m = c.trace_beam();
        assert_eq!(format!("{}", m), "####\n#..#\n####");
        assert_eq!(m.num_energized(), 10);
    }
}
