#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn is_east_west(&self) -> bool {
        self == &Direction::East || self == &Direction::West
    }

    pub fn is_south_north(&self) -> bool {
        self == &Direction::South || self == &Direction::North
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Position {
    row: usize,
    column: usize,
    size: (usize, usize),
}

impl Position {
    pub fn new(row: usize, column: usize, size: (usize, usize)) -> Position {
        Position { row, column, size }
    }

    pub fn to(&self, direction: Direction) -> Option<Position> {
        match direction {
            Direction::North => {
                if self.row == 0 {
                    return None;
                }
                return Some(Position::new(self.row - 1, self.column, self.size));
            }
            Direction::East => {
                if self.column == self.size.1 - 1 {
                    return None;
                }
                return Some(Position::new(self.row, self.column + 1, self.size));
            }
            Direction::South => {
                if self.row == self.size.0 - 1 {
                    return None;
                }
                return Some(Position::new(self.row + 1, self.column, self.size));
            }
            Direction::West => {
                if self.column == 0 {
                    return None;
                }
                return Some(Position::new(self.row, self.column - 1, self.size));
            }
        };
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.column
    }
}
