use std::{collections::HashMap, fmt};

#[derive(Clone, Debug, PartialEq)]
pub enum Rock {
    Rounded,
    Squared,
}

impl TryFrom<char> for Rock {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'O' => Ok(Rock::Rounded),
            '#' => Ok(Rock::Squared),
            _ => Err("invalid Rock character"),
        }
    }
}

impl Into<char> for &Rock {
    fn into(self) -> char {
        match self {
            Rock::Rounded => 'O',
            Rock::Squared => '#',
        }
    }
}

#[derive(PartialEq)]
pub struct Dish {
    size: (usize, usize),
    content: Vec<Vec<Option<Rock>>>,
}

impl Dish {
    pub fn parse(input: &str) -> Dish {
        let mut d = Dish {
            size: (0, 0),
            content: Vec::new(),
        };

        for line in input.lines() {
            let mut row: Vec<Option<Rock>> = Vec::new();
            for c in line.chars() {
                row.push(if let Ok(rock) = Rock::try_from(c) {
                    Some(rock)
                } else {
                    None
                });
            }
            d.content.push(row);
        }

        d.size = if d.content.len() == 0 {
            (0, 0)
        } else {
            (d.content.len(), d.content[0].len())
        };

        d
    }

    fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.size.0 * (self.size.1 + 1));
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                s.push(if let Some(rock) = &self.content[i][j] {
                    rock.into()
                } else {
                    '.'
                });
            }
            if i < self.size.0 - 1 {
                s.push('\n');
            }
        }
        s
    }

    pub fn load(&self) -> u64 {
        let mut load = 0usize;

        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                if let Some(rock) = &self.content[i][j] {
                    load += match rock {
                        Rock::Rounded => self.size.0 - i,
                        Rock::Squared => 0,
                    };
                }
            }
        }

        load as u64
    }

    fn slide_north(&self) -> Dish {
        let mut d = Dish {
            size: self.size,
            content: vec![vec![None; self.size.1]; self.size.0],
        };

        for j in 0..self.size.1 {
            let mut next_available_slot: Option<usize> = None;
            for i in 0..self.size.0 {
                match self.content[i][j] {
                    Some(Rock::Rounded) => {
                        let row = if let Some(index) = next_available_slot {
                            next_available_slot = Some(index + 1);
                            index
                        } else {
                            i
                        };
                        d.content[row][j] = Some(Rock::Rounded);
                    }
                    Some(Rock::Squared) => {
                        d.content[i][j] = Some(Rock::Squared);
                        next_available_slot = None;
                    }
                    None => {
                        if next_available_slot.is_none() {
                            next_available_slot = Some(i);
                        }
                    }
                }
            }
        }

        d
    }

    fn slide_west(&self) -> Dish {
        let mut d = Dish {
            size: self.size,
            content: vec![vec![None; self.size.1]; self.size.0],
        };

        for i in 0..self.size.0 {
            let mut next_available_slot: Option<usize> = None;
            for j in 0..self.size.1 {
                match self.content[i][j] {
                    Some(Rock::Rounded) => {
                        let col = if let Some(index) = next_available_slot {
                            next_available_slot = Some(index + 1);
                            index
                        } else {
                            j
                        };
                        d.content[i][col] = Some(Rock::Rounded);
                    }
                    Some(Rock::Squared) => {
                        d.content[i][j] = Some(Rock::Squared);
                        next_available_slot = None;
                    }
                    None => {
                        if next_available_slot.is_none() {
                            next_available_slot = Some(j);
                        }
                    }
                }
            }
        }

        d
    }

    fn slide_south(&self) -> Dish {
        let mut d = Dish {
            size: self.size,
            content: vec![vec![None; self.size.1]; self.size.0],
        };

        for j in 0..self.size.1 {
            let mut next_available_slot: Option<usize> = None;
            for i in 0..self.size.0 {
                match self.content[self.size.0 - i - 1][j] {
                    Some(Rock::Rounded) => {
                        let row = if let Some(index) = next_available_slot {
                            next_available_slot = Some(index - 1);
                            index
                        } else {
                            self.size.0 - i - 1
                        };
                        d.content[row][j] = Some(Rock::Rounded);
                    }
                    Some(Rock::Squared) => {
                        d.content[self.size.0 - i - 1][j] = Some(Rock::Squared);
                        next_available_slot = None;
                    }
                    None => {
                        if next_available_slot.is_none() {
                            next_available_slot = Some(self.size.0 - i - 1);
                        }
                    }
                }
            }
        }

        d
    }

    fn slide_east(&self) -> Dish {
        let mut d = Dish {
            size: self.size,
            content: vec![vec![None; self.size.1]; self.size.0],
        };

        for i in 0..self.size.0 {
            let mut next_available_slot: Option<usize> = None;
            for j in 0..self.size.1 {
                match self.content[i][self.size.1 - j - 1] {
                    Some(Rock::Rounded) => {
                        let col = if let Some(index) = next_available_slot {
                            next_available_slot = Some(index - 1);
                            index
                        } else {
                            self.size.1 - j - 1
                        };
                        d.content[i][col] = Some(Rock::Rounded);
                    }
                    Some(Rock::Squared) => {
                        d.content[i][self.size.1 - j - 1] = Some(Rock::Squared);
                        next_available_slot = None;
                    }
                    None => {
                        if next_available_slot.is_none() {
                            next_available_slot = Some(self.size.1 - j - 1);
                        }
                    }
                }
            }
        }

        d
    }

    pub fn cycle(&self) -> Dish {
        self.slide_north().slide_west().slide_south().slide_east()
    }

    pub fn load_after_cycles(&self, n: u64) -> u64 {
        let mut next = self.cycle();
        let mut count = 1u64;
        while count < n {
            count += 1;
            next = next.cycle();
        }

        next.load()
    }

    pub fn load_after_cycles_fast(&self, n: u64) -> u64 {
        // this is periodical dynamic system
        // let's just hope the period is not too large
        
        let mut to_next: HashMap<String, String> = HashMap::new();
        let mut to_step_and_load: HashMap<String, (u64, u64)> = HashMap::new();

        to_step_and_load.insert(self.to_string(), (0, self.load()));

        let mut count = 1u64;
        let mut next = self.cycle();
        to_step_and_load.insert(next.to_string(), (count, next.load()));
        to_next.insert(self.to_string(), next.to_string());

        while count < n && !to_next.contains_key(&next.to_string()) {
            count += 1;
            let tmp = next.cycle();
            if to_next.contains_key(&tmp.to_string()) {
                next = tmp;
                break;
            }
            to_step_and_load.insert(tmp.to_string(), (count, tmp.load()));
            to_next.insert(next.to_string(), tmp.to_string());

            next = tmp;
        }

        if to_next.contains_key(&next.to_string()) {
            let previous = to_step_and_load.get(&next.to_string()).unwrap();
            let periodicity = count - previous.0;
            let final_step = (n - previous.0) % periodicity;

            let mut next_str = &next.to_string();
            count = 0;
            while count < final_step {
                count += 1;
                next_str = to_next.get(next_str).unwrap();
            }

            return to_step_and_load.get(next_str).unwrap().1;
        }

        next.load()
    }

    pub fn load_after_one_slide_north_fast(&self) -> u64 {
        let mut load = 0u64;
        for j in 0..self.size.1 {
            load += self.column_load_after_slide_north(j);
        }
        load
    }

    fn column_load_after_slide_north(&self, column: usize) -> u64 {
        let mut load = 0usize;
        let mut next_available_slot: Option<usize> = None;
        for i in 0..self.size.0 {
            match self.content[i][column] {
                Some(Rock::Rounded) => {
                    let row = if let Some(index) = next_available_slot {
                        next_available_slot = Some(index + 1);
                        index
                    } else {
                        i
                    };
                    load += self.size.0 - row;
                }
                Some(Rock::Squared) => next_available_slot = None,
                None => {
                    if next_available_slot.is_none() {
                        next_available_slot = Some(i);
                    }
                }
            }
        }

        load as u64
    }
}

impl fmt::Display for Dish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const INPUT: &'static str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_column_load() {
        let d = Dish::parse(&INPUT);

        assert_eq!(d.column_load_after_slide_north(0), 34);
        assert_eq!(d.column_load_after_slide_north(1), 27);
        assert_eq!(d.column_load_after_slide_north(2), 17);
        assert_eq!(d.column_load_after_slide_north(3), 10);
        assert_eq!(d.column_load_after_slide_north(4), 8);
        assert_eq!(d.column_load_after_slide_north(5), 7);
        assert_eq!(d.column_load_after_slide_north(6), 7);
        assert_eq!(d.column_load_after_slide_north(7), 14);
        assert_eq!(d.column_load_after_slide_north(8), 0);
        assert_eq!(d.column_load_after_slide_north(9), 12);
    }

    #[test]
    fn test_load_after_one_slide_north() {
        let d = Dish::parse(&INPUT);
        assert_eq!(d.slide_north().load(), 136);
    }

    #[test]
    fn test_load_after_one_slide_north_fast() {
        let d = Dish::parse(&INPUT);
        assert_eq!(
            d.load_after_one_slide_north_fast(),
            d.load_after_one_slide_north_fast()
        );
    }

    #[test]
    fn test_load_after_one_slide_west() {
        let d = Dish::parse(&INPUT);
        assert_eq!(d.slide_west().load(), 104);
    }

    #[test]
    fn test_load_after_cycles() {
        let d = Dish::parse(&INPUT);
        // relax
        assert_eq!(d.load_after_cycles(1), 87);
        assert_eq!(d.load_after_cycles(2), 69);
        // first period
        assert_eq!(d.load_after_cycles(3), 69);
        assert_eq!(d.load_after_cycles(4), 69);
        assert_eq!(d.load_after_cycles(5), 65);
        assert_eq!(d.load_after_cycles(6), 64);
        assert_eq!(d.load_after_cycles(7), 65);
        assert_eq!(d.load_after_cycles(8), 63);
        assert_eq!(d.load_after_cycles(9), 68);
        // second period
        assert_eq!(d.load_after_cycles(10), 69);
        assert_eq!(d.load_after_cycles(11), 69);
        assert_eq!(d.load_after_cycles(12), 65);
        assert_eq!(d.load_after_cycles(13), 64);
        assert_eq!(d.load_after_cycles(14), 65);
        assert_eq!(d.load_after_cycles(15), 63);
        assert_eq!(d.load_after_cycles(16), 68);
        // third period
        assert_eq!(d.load_after_cycles(17), 69);
        assert_eq!(d.load_after_cycles(18), 69);
        assert_eq!(d.load_after_cycles(19), 65);
        assert_eq!(d.load_after_cycles(20), 64);
        assert_eq!(d.load_after_cycles(21), 65);
        assert_eq!(d.load_after_cycles(22), 63);
        assert_eq!(d.load_after_cycles(23), 68);
    }

    #[test]
    fn test_load_after_cycles_fast() {
        let d = Dish::parse(&INPUT);
        assert_eq!(d.load_after_cycles_fast(14), 65);
        assert_eq!(d.load_after_cycles_fast(23), 68);

        assert_eq!(d.load_after_cycles_fast(1_000_000_000), 64);
    }

    #[test]
    fn test_load_after_cycles_real_data() {
        let input = fs::read_to_string("data/day14.txt").unwrap();
        let d = Dish::parse(&input);

        // relax
        assert_eq!(d.load_after_cycles(1), 100_634);
        assert_eq!(d.load_after_cycles(2), 100_117);
        assert_eq!(d.load_after_cycles(3), 99_816);
        assert_eq!(d.load_after_cycles(4), 99_539);
        assert_eq!(d.load_after_cycles(5), 99_160);
        assert_eq!(d.load_after_cycles(6), 98_916);
        assert_eq!(d.load_after_cycles(7), 98_661);
        assert_eq!(d.load_after_cycles(8), 98_377);
        assert_eq!(d.load_after_cycles(9), 98_100);
        assert_eq!(d.load_after_cycles(10), 97_772);
        assert_eq!(d.load_after_cycles(11), 97_531);
        assert_eq!(d.load_after_cycles(12), 97_244);
        assert_eq!(d.load_after_cycles(13), 97_068);
        assert_eq!(d.load_after_cycles(14), 96_863);
        assert_eq!(d.load_after_cycles(15), 96_736);
        assert_eq!(d.load_after_cycles(16), 96_595);
        assert_eq!(d.load_after_cycles(17), 96_431);
        assert_eq!(d.load_after_cycles(18), 96_286);
        assert_eq!(d.load_after_cycles(19), 96_155);
        assert_eq!(d.load_after_cycles(20), 96_027);
        assert_eq!(d.load_after_cycles(21), 95_904);
        assert_eq!(d.load_after_cycles(22), 95_800);
        assert_eq!(d.load_after_cycles(23), 95_741);
    }
}
