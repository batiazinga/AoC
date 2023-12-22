use crate::grid2d::Direction;
use crate::grid2d::Position;
use std::fmt;

pub struct Instruction {
    dir: Direction,
    num_steps: usize,
}

impl Instruction {
    pub fn direction(&self) -> Direction {
        self.dir
    }

    pub fn num_steps(&self) -> usize {
        self.num_steps
    }
}

fn direction_from_str(s: &str) -> Option<Direction> {
    match s {
        "R" => Some(Direction::East),
        "D" => Some(Direction::South),
        "L" => Some(Direction::West),
        "U" => Some(Direction::North),
        _ => None,
    }
}

fn direction_from_str_digit(s: &str) -> Option<Direction> {
    match s {
        "0" => Some(Direction::East),
        "1" => Some(Direction::South),
        "2" => Some(Direction::West),
        "3" => Some(Direction::North),
        _ => None,
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Lagoon {
    Border,
    Inside,
    Outside,
}

pub struct DigMap {
    content: Vec<Vec<Lagoon>>,
    num_rows: usize,
    num_cols: usize,
}

impl DigMap {
    pub fn volume(&self) -> u64 {
        let mut count = 0u64;
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                let state = self.content[i][j];
                if state == Lagoon::Border || state == Lagoon::Inside {
                    count += 1;
                }
            }
        }
        count
    }

    fn dig_border(&mut self, position: &Position) {
        self.content[position.row()][position.col()] = Lagoon::Border;
    }

    fn dig_to_south(&mut self, position: &Position) {
        for i in position.row()..self.num_rows {
            let state = self.content[i][position.col()];
            if state == Lagoon::Border || state == Lagoon::Inside {
                break;
            }
            self.content[i][position.col()] = Lagoon::Inside;
        }
    }

    fn undig_to_south(&mut self, position: &Position) {
        for i in position.row()..self.num_rows {
            let state = self.content[i][position.col()];
            if state == Lagoon::Border || state == Lagoon::Outside {
                break;
            }
            self.content[i][position.col()] = Lagoon::Outside;
        }
    }

    fn dig_at(&mut self, position: &Position, direction: Direction) {
        // /!\ I assumed that we follow the border in clockwise direction.
        self.dig_border(position);
        if let Some(next_pos_south) = position.to(Direction::South) {
            match direction {
                Direction::East => self.dig_to_south(&next_pos_south),
                Direction::West => self.undig_to_south(&next_pos_south),
                _ => {}
            }
        }
    }
}

impl fmt::Display for DigMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.num_rows == 0 {
            return write!(f, "");
        }

        let mut s = String::with_capacity(self.num_rows * (self.num_cols + 1) - 1);
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                s.push(match self.content[i][j] {
                    Lagoon::Border => '#',
                    Lagoon::Inside => '#',
                    Lagoon::Outside => '.',
                });
            }
            if i < self.num_rows - 1 {
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}

pub fn read_dig_plan(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let dir = direction_from_str(parts.next().unwrap()).unwrap();
        let num_steps: usize = parts.next().unwrap().parse().unwrap();
        instructions.push(Instruction { dir, num_steps });
    }

    instructions
}

pub fn read_dig_plan_correctly(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in input.lines() {
        let sharp_idx = line.find('#').unwrap();
        let parenth_idx = line.find(')').unwrap();
        instructions.push(Instruction {
            dir: direction_from_str_digit(&line[parenth_idx - 1..parenth_idx]).unwrap(),
            num_steps: usize::from_str_radix(&line[sharp_idx + 1..parenth_idx - 1], 16).unwrap(),
        })
    }

    instructions
}

fn calibrate_grid(instructions: &[Instruction]) -> (usize, usize, usize, usize) {
    let mut p = (0i64, 0i64);
    let mut min = (0i64, 0i64);
    let mut max = (0i64, 0i64);

    for instruction in instructions {
        match instruction.direction() {
            Direction::East => p.1 += instruction.num_steps() as i64,
            Direction::South => p.0 += instruction.num_steps() as i64,
            Direction::West => p.1 -= instruction.num_steps() as i64,
            Direction::North => p.0 -= instruction.num_steps() as i64,
        }
        if p.0 > max.0 {
            max.0 = p.0;
        }
        if p.1 > max.1 {
            max.1 = p.1;
        }
        if p.0 < min.0 {
            min.0 = p.0;
        }
        if p.1 < min.1 {
            min.1 = p.1;
        }
    }

    (
        (-min.0) as usize,
        (-min.1) as usize,
        (max.0 - min.0) as usize + 1,
        (max.1 - min.1) as usize + 1,
    )
}

pub fn dig(instructions: &[Instruction]) -> DigMap {
    let calibration = calibrate_grid(instructions);
    let mut m = DigMap {
        content: vec![vec![Lagoon::Outside; calibration.3]; calibration.2],
        num_rows: calibration.2,
        num_cols: calibration.3,
    };

    let mut pos = Position::new(calibration.0, calibration.1, (calibration.2, calibration.3));
    for instruction in instructions {
        m.dig_at(&pos, instruction.direction());
        let mut counter = 0usize;
        while counter < instruction.num_steps() {
            pos = pos.to(instruction.direction()).unwrap();

            m.dig_at(&pos, instruction.direction());

            counter += 1;
        }
    }

    m
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    const OUTPUT_MAP: &'static str = "#######
#######
#######
..#####
..#####
#######
#####..
#######
.######
.######";

    #[test]
    fn test_read_instructions() {
        let instructions = read_dig_plan(&INPUT);

        assert_eq!(instructions.len(), 14);

        assert_eq!(instructions[0].direction(), Direction::East);
        assert_eq!(instructions[0].num_steps(), 6);

        assert_eq!(instructions[1].direction(), Direction::South);
        assert_eq!(instructions[1].num_steps(), 5);

        assert_eq!(instructions[2].direction(), Direction::West);
        assert_eq!(instructions[2].num_steps(), 2);

        assert_eq!(instructions[13].direction(), Direction::North);
        assert_eq!(instructions[13].num_steps(), 2);
    }

    #[test]
    fn test_read_instructions_correctly() {
        let instructions = read_dig_plan_correctly(&INPUT);

        assert_eq!(instructions.len(), 14);

        assert_eq!(instructions[0].direction(), Direction::East);
        assert_eq!(instructions[0].num_steps(), 461937);

        assert_eq!(instructions[1].direction(), Direction::South);
        assert_eq!(instructions[1].num_steps(), 56407);

        assert_eq!(instructions[2].direction(), Direction::East);
        assert_eq!(instructions[2].num_steps(), 356671);

        assert_eq!(instructions[6].direction(), Direction::West);
        assert_eq!(instructions[6].num_steps(), 577262);

        assert_eq!(instructions[13].direction(), Direction::North);
        assert_eq!(instructions[13].num_steps(), 500254);
    }

    #[test]
    fn test_min_grid_size() {
        let instructions = read_dig_plan(&INPUT);
        assert_eq!(calibrate_grid(instructions.as_slice()), (0, 0, 10, 7));
    }

    #[test]
    fn test_count_dug() {
        let instructions = read_dig_plan(&INPUT);
        let dig_map = dig(instructions.as_slice());
        assert_eq!(format!("{}", dig_map), OUTPUT_MAP);
        assert_eq!(dig_map.volume(), 62);
    }
}
