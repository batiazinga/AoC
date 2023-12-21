use std::collections::HashSet;
use std::fmt;

pub struct EnergyMap {
    content: Vec<Vec<char>>,
    num_rows: usize,
    num_cols: usize,
}

impl EnergyMap {
    pub fn num_energized(&self) -> u64 {
        let mut count = 0u64;

        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                if self.content[i][j] == '#' {
                    count += 1;
                }
            }
        }

        count
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
                s.push(self.content[i][j]);
            }
            if i < self.num_rows - 1 {
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}

pub struct Contraption {
    content: Vec<Vec<char>>,
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
                let mut row: Vec<char> = Vec::new();
                for c in line.chars() {
                    row.push(c);
                }
                c.num_cols = row.len();
                c.content.push(row);
                is_first_line = false;
                continue;
            }

            let mut row: Vec<char> = Vec::with_capacity(c.num_cols);
            for c in line.chars() {
                row.push(c);
            }
            c.content.push(row);
        }
        c.num_rows = c.content.len();

        c
    }

    pub fn trace_beam(&self) -> EnergyMap {
        self.trace_beam_from((0, 0), (0, 1))
    }

    fn trace_beam_from(&self, pos: (i32, i32), incr: (i32, i32)) -> EnergyMap {
        let mut m = EnergyMap {
            content: vec![vec!['.'; self.num_cols]; self.num_rows],
            num_rows: self.num_rows,
            num_cols: self.num_cols,
        };

        let mut visited: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        self.rec_trace_beam(&mut m, &mut visited, pos, incr);

        m
    }

    fn rec_trace_beam(
        &self,
        m: &mut EnergyMap,
        visited: &mut HashSet<(i32, i32, i32, i32)>,
        pos: (i32, i32),
        incr: (i32, i32),
    ) {
        if visited.contains(&(pos.0, pos.1, incr.0, incr.1)) {
            return;
        }
        visited.insert((pos.0, pos.1, incr.0, incr.1));

        let mut position = pos;
        let mut increment = incr;
        loop {
            if self.content[position.0 as usize][position.1 as usize] == '-' && increment.0 != 0 {
                self.rec_trace_beam(m, visited, position, (0, 1));
                self.rec_trace_beam(m, visited, position, (0, -1));
                break;
            }
            if self.content[position.0 as usize][position.1 as usize] == '|' && increment.1 != 0 {
                self.rec_trace_beam(m, visited, position, (-1, 0));
                self.rec_trace_beam(m, visited, position, (1, 0));
                break;
            }

            m.content[position.0 as usize][position.1 as usize] = '#';

            increment = match self.content[position.0 as usize][position.1 as usize] {
                '\\' => (increment.1, increment.0),
                '/' => (-increment.1, -increment.0),
                _ => increment,
            };

            let next = (position.0 + increment.0, position.1 + increment.1);
            if next.0 == -1
                || next.0 as usize == self.num_rows
                || next.1 == -1
                || next.1 as usize == self.num_cols
            {
                break;
            }

            position = next;
        }
    }

    pub fn max_energized(&self) -> u64 {
        let mut max = 0u64;

        for j in 0..self.num_cols {
            let count = self.trace_beam_from((0, j as i32), (1, 0)).num_energized();
            if count > max {
                max = count;
            }
        }
        for j in 0..self.num_cols {
            let count = self
                .trace_beam_from(((self.num_rows as i32) - 1, j as i32), (-1, 0))
                .num_energized();
            if count > max {
                max = count;
            }
        }
        for i in 0..self.num_rows {
            let count = self.trace_beam_from((i as i32, 0), (0, 1)).num_energized();
            if count > max {
                max = count;
            }
        }
        for i in 0..self.num_rows {
            let count = self
                .trace_beam_from((i as i32, (self.num_cols as i32) - 1), (0, -1))
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
                s.push(self.content[i][j]);
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
