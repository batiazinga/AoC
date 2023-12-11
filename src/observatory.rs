use crate::grid2d::Position;
use std::fmt;

pub struct SpaceMap {
    size: (usize, usize),
    content: Vec<Vec<bool>>,
    expanded_rows: Vec<usize>,
    expanded_cols: Vec<usize>,
}

impl SpaceMap {
    pub fn parse(input: &str) -> SpaceMap {
        let mut m = SpaceMap {
            size: (0, 0),
            content: Vec::new(),
            expanded_rows: Vec::new(),
            expanded_cols: Vec::new(),
        };

        for line in input.lines() {
            let mut row: Vec<bool> = Vec::new();
            for c in line.chars() {
                let v = match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("unexpected character"),
                };
                row.push(v);
            }
            m.content.push(row);
        }
        m.size = (m.content.len(), m.content[0].len());

        for i in 0..m.size.0 {
            let mut has_galaxies = false;
            for j in 0..m.size.1 {
                if m.content[i][j] {
                    has_galaxies = true;
                    break;
                }
            }
            if !has_galaxies {
                m.expanded_rows.push(i);
            }
        }

        for i in 0..m.size.1 {
            let mut has_galaxies = false;
            for j in 0..m.size.0 {
                if m.content[j][i] {
                    has_galaxies = true;
                    break;
                }
            }
            if !has_galaxies {
                m.expanded_cols.push(i);
            }
        }

        m
    }

    fn galaxies(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();

        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                if self.content[i][j] {
                    positions.push(Position::new(i, j, self.size));
                }
            }
        }

        positions
    }

    fn num_expanded_rows(&self, start: usize, end: usize) -> usize {
        let mut sum = 0usize;
        for row in &self.expanded_rows {
            if *row >= start && *row < end {
                sum += 1;
            }
        }

        sum
    }

    fn num_expanded_cols(&self, start: usize, end: usize) -> usize {
        let mut sum = 0usize;
        for col in &self.expanded_cols {
            if *col >= start && *col < end {
                sum += 1;
            }
        }

        sum
    }

    fn manhattan_distance(&self, x: Position, y: Position, expansion: usize) -> usize {
        let (max_x, min_x) = if x.row() > y.row() {
            (x.row(), y.row())
        } else {
            (y.row(), x.row())
        };
        let x_dist = max_x - min_x + self.num_expanded_rows(min_x, max_x) * (expansion-1);

        let (max_y, min_y) = if x.col() > y.col() {
            (x.col(), y.col())
        } else {
            (y.col(), x.col())
        };
        let y_dist = max_y - min_y + self.num_expanded_cols(min_y, max_y) * (expansion-1);

        x_dist + y_dist
    }

    pub fn sum_galaxy_pair_distances(&self, expansion: usize) -> usize {
        let mut sum = 0usize;
        let galaxies = self.galaxies();
        for i in 0..galaxies.len() {
            for j in i..galaxies.len() {
                sum += self.manhattan_distance(galaxies[i], galaxies[j], expansion);
            }
        }

        sum
    }
}

impl fmt::Display for SpaceMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_string();
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                let c = match self.content[i][j] {
                    true => '#',
                    false => '.',
                };
                s.push(c);
            }
            if i < self.size.0 - 1 {
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_parse_space_map() {
        let m = SpaceMap::parse(&INPUT);
        let output = format!("{}", m);
        assert_eq!(INPUT, output);
    }

    #[test]
    fn test_galaxies() {
        let m = SpaceMap::parse(&INPUT);
        let galaxies = m.galaxies();

        assert_eq!(galaxies.len(), 9);
        let positions = [
            (0, 3),
            (1, 7),
            (2, 0),
            (4, 6),
            (5, 1),
            (6, 9),
            (8, 7),
            (9, 0),
            (9, 4),
        ];
        for i in 0..galaxies.len() {
            assert_eq!(galaxies[i].row(), positions[i].0);
            assert_eq!(galaxies[i].col(), positions[i].1);
        }
    }

    #[test]
    fn test_sum_distances_2() {
        let m = SpaceMap::parse(&INPUT);
        let d = m.sum_galaxy_pair_distances(2);
        assert_eq!(d, 374);
    }

    #[test]
    fn test_sum_distances_10() {
        let m = SpaceMap::parse(&INPUT);
        let d = m.sum_galaxy_pair_distances(10);
        assert_eq!(d, 1030);
    }

    #[test]
    fn test_sum_distances_100() {
        let m = SpaceMap::parse(&INPUT);
        let d = m.sum_galaxy_pair_distances(100);
        assert_eq!(d, 8410);
    }
}
