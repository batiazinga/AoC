use std::{collections::HashSet, fmt};

pub struct LossMap {
    content: Vec<Vec<u64>>,
    num_rows: usize,
    num_cols: usize,
}

impl LossMap {
    pub fn parse(input: &str) -> LossMap {
        let mut m = LossMap {
            content: Vec::new(),
            num_rows: 0,
            num_cols: 0,
        };

        let mut is_first_line: bool = true;
        for line in input.lines() {
            if is_first_line {
                let mut row: Vec<u64> = Vec::new();
                for c in line.chars() {
                    row.push(c.to_digit(10).unwrap() as u64);
                }
                m.num_cols = row.len();
                m.content.push(row);
                is_first_line = false;
                continue;
            }

            let mut row: Vec<u64> = Vec::with_capacity(m.num_cols);
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as u64);
            }
            m.content.push(row);
        }
        m.num_rows = m.content.len();

        m
    }

    pub fn shortest_path(&self) -> u64 {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((0, 0));
        (self.content[0][1] + self.shortest_path_from(&mut visited, (0, 1), (0, 1), 1))
            .min(self.content[1][0] + self.shortest_path_from(&mut visited, (1, 0), (1, 0), 1))
    }

    fn shortest_path_from(
        &self,
        visited: &mut HashSet<(i32, i32)>,
        pos: (i32, i32),
        latest_incr: (i32, i32),
        dist_since_last_turn: u8,
    ) -> u64 {
        if visited.contains(&pos) {
            return u64::MAX;
        }
        visited.insert(pos);

        if pos == (self.num_rows as i32 - 1, self.num_cols as i32 - 1) {
            visited.remove(&pos);
            return 0;
        }

        let mut dist = u64::MAX;

        if dist_since_last_turn < 3 {
            let next = (pos.0 + latest_incr.0, pos.1 + latest_incr.1);
            if next.0 >= 0
                && (next.0 as usize) < self.num_rows
                && next.1 >= 0
                && (next.1 as usize) < self.num_cols
            {
                let shortest_path =
                    self.shortest_path_from(visited, next, latest_incr, dist_since_last_turn + 1);
                if shortest_path != u64::MAX {
                    dist = self.content[next.0 as usize][next.1 as usize] + shortest_path;
                }
            }
        }

        let mut incr = (latest_incr.1, latest_incr.0);
        let mut next = (pos.0 + incr.0, pos.1 + incr.1);
        if next.0 >= 0
            && (next.0 as usize) < self.num_rows
            && next.1 >= 0
            && (next.1 as usize) < self.num_cols
        {
            let shortest_path = self.shortest_path_from(visited, next, incr, 1);
            if shortest_path != u64::MAX {
                dist = dist.min(self.content[next.0 as usize][next.1 as usize] + shortest_path);
            }
        }

        incr = (-incr.0, -incr.1);
        next = (pos.0 + incr.0, pos.1 + incr.1);
        if next.0 >= 0
            && (next.0 as usize) < self.num_rows
            && next.1 >= 0
            && (next.1 as usize) < self.num_cols
        {
            let shortest_path = self.shortest_path_from(visited, next, incr, 1);
            if shortest_path != u64::MAX {
                dist = dist.min(self.content[next.0 as usize][next.1 as usize] + shortest_path);
            }
        }

        visited.remove(&pos);
        dist
    }
}

impl fmt::Display for LossMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.num_rows == 0 {
            return write!(f, "");
        }

        let mut s = String::with_capacity(self.num_rows * (self.num_cols + 1) - 1);
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                s.push(char::from_digit(self.content[i][j] as u32, 10).unwrap());
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

    const INPUT: &'static str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_parse_map() {
        let c = LossMap::parse(&INPUT);
        assert_eq!(format!("{}", c), INPUT);
    }

    // #[test]
    // fn test_shortest_path() {
    //     let m = LossMap::parse(&INPUT);
    //     assert_eq!(m.shortest_path(), 102);
    // }

    #[test]
    fn test_shortest_path_small() {
        let m = LossMap::parse("21111111\n22288222");
        assert_eq!(m.shortest_path(), 15);
    }
}