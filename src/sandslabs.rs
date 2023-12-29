use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn parse(input: &str) -> Point {
        let mut coordinates = input.split(',').map(|s| s.parse::<usize>().unwrap());
        let x = coordinates.next().unwrap();
        let y = coordinates.next().unwrap();
        let z = coordinates.next().unwrap();
        Point { x, y, z }
    }

    fn down_by(&mut self, height: usize) {
        self.z -= height;
    }
}

#[derive(Debug, PartialEq)]
pub struct Brick {
    // assume coordinates are increasing
    start: Point,
    end: Point,
}

impl Brick {
    fn fall_by(&mut self, height: usize) {
        self.start.down_by(height);
        self.end.down_by(height);
    }
}

pub fn read_bricks(input: &str) -> Vec<Brick> {
    let mut bricks = Vec::new();

    for line in input.lines() {
        let tilde_index = line.find('~').unwrap();
        bricks.push(Brick {
            start: Point::parse(&line[..tilde_index]),
            end: Point::parse(&line[tilde_index + 1..]),
        });
    }

    bricks.sort_by_key(|k| k.start.z);
    fall(&mut bricks);

    bricks
}

// assume that min_x and min_y are 0
fn xy_coverage(bricks: &[Brick]) -> (usize, usize) {
    let first_brick = &bricks[0];
    let mut max_x = first_brick.end.x;
    let mut max_y = first_brick.end.y;
    for i in 1..bricks.len() {
        let b = &bricks[i];
        if b.end.x > max_x {
            max_x = b.end.x;
        }
        if b.end.x > max_y {
            max_y = b.end.x;
        }
    }
    (max_x + 1, max_y + 1)
}

fn fall(bricks: &mut Vec<Brick>) {
    let xy_coverage = xy_coverage(bricks.as_slice());
    let mut z_map = vec![vec![0usize; xy_coverage.1]; xy_coverage.0];

    for b in bricks {
        let mut max_z = 0usize;
        for i in b.start.x..=b.end.x {
            for j in b.start.y..=b.end.y {
                if z_map[i][j] > max_z {
                    max_z = z_map[i][j];
                }
            }
        }
        b.fall_by(b.start.z - max_z - 1);

        for i in b.start.x..=b.end.x {
            for j in b.start.y..=b.end.y {
                z_map[i][j] = b.end.z;
            }
        }
    }
}

pub fn count_disintegrable_bricks(bricks: &[Brick]) -> usize {
    let mut support_of: Vec<Vec<usize>> = vec![Vec::new(); bricks.len()];
    for i in 0..bricks.len() {
        let above = &bricks[i];
        for j in 0..i {
            let below = &bricks[j];
            if above.start.z != below.end.z + 1 {
                continue;
            }
            let x_overlap = (below.start.x <= above.start.x && below.end.x >= above.start.x)
                || (below.start.x >= above.start.x && below.start.x <= above.end.x);
            let y_overlap = (below.start.y <= above.start.y && below.end.y >= above.start.y)
                || (below.start.y >= above.start.y && below.start.y <= above.end.y);
            if x_overlap && y_overlap {
                support_of[i].push(j);
            }
        }
    }

    let mut cannot_be_disintegrated: HashSet<usize> = HashSet::new();
    for supporting in support_of {
        if supporting.len() == 1 {
            cannot_be_disintegrated.insert(supporting[0]);
        }
    }

    bricks.len() - cannot_be_disintegrated.len()
}

pub fn sum_chain_reactions(bricks: &[Brick]) -> u64 {
    let mut supported_by: Vec<Vec<usize>> = vec![Vec::new(); bricks.len()];
    let mut support_of: Vec<Vec<usize>> = vec![Vec::new(); bricks.len()];
    for i in 0..bricks.len() {
        let above = &bricks[i];
        for j in 0..i {
            let below = &bricks[j];
            if above.start.z != below.end.z + 1 {
                continue;
            }
            let x_overlap = (below.start.x <= above.start.x && below.end.x >= above.start.x)
                || (below.start.x >= above.start.x && below.start.x <= above.end.x);
            let y_overlap = (below.start.y <= above.start.y && below.end.y >= above.start.y)
                || (below.start.y >= above.start.y && below.start.y <= above.end.y);
            if x_overlap && y_overlap {
                supported_by[j].push(i);
                support_of[i].push(j);
            }
        }
    }

    let mut sum = 0;
    for i in 0..bricks.len() {
        sum += chain_reaction(i, &supported_by, &support_of);
    }
    sum
}

fn chain_reaction(
    start: usize,
    supported_by: &Vec<Vec<usize>>,
    support_of: &Vec<Vec<usize>>,
) -> u64 {
    let mut fallen: HashSet<usize> = HashSet::from([start]);
    let mut queue: VecDeque<usize> = VecDeque::from([start]);

    while let Some(i) = queue.pop_front() {
        for j in &supported_by[i] {
            if support_of[*j].iter().all(|b| fallen.contains(b)) {
                fallen.insert(*j);
                queue.push_back(*j);
            }
        }
    }

    fallen.len() as u64 - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "1,0,1~1,2,1
0,1,6~2,1,6
0,0,4~0,2,4
0,2,3~2,2,3
2,0,5~2,2,5
0,0,2~2,0,2
1,1,8~1,1,9";

    #[test]
    fn test_read_bricks() {
        let bricks = read_bricks(&INPUT);

        assert_eq!(bricks.len(), 7);
        assert_eq!(
            bricks[0],
            Brick {
                start: Point { x: 1, y: 0, z: 1 },
                end: Point { x: 1, y: 2, z: 1 }
            }
        );
        assert_eq!(
            bricks[1],
            Brick {
                start: Point { x: 0, y: 0, z: 2 },
                end: Point { x: 2, y: 0, z: 2 }
            }
        );
        assert_eq!(
            bricks[2],
            Brick {
                start: Point { x: 0, y: 2, z: 2 },
                end: Point { x: 2, y: 2, z: 2 }
            }
        );
        assert_eq!(
            bricks[3],
            Brick {
                start: Point { x: 0, y: 0, z: 3 },
                end: Point { x: 0, y: 2, z: 3 }
            }
        );
        assert_eq!(
            bricks[4],
            Brick {
                start: Point { x: 2, y: 0, z: 3 },
                end: Point { x: 2, y: 2, z: 3 }
            }
        );
        assert_eq!(
            bricks[5],
            Brick {
                start: Point { x: 0, y: 1, z: 4 },
                end: Point { x: 2, y: 1, z: 4 }
            }
        );
        assert_eq!(
            bricks[6],
            Brick {
                start: Point { x: 1, y: 1, z: 5 },
                end: Point { x: 1, y: 1, z: 6 }
            }
        );
    }

    #[test]
    fn test_count_disintegrable() {
        let bricks = read_bricks(&INPUT);
        assert_eq!(count_disintegrable_bricks(bricks.as_slice()), 5);
    }

    #[test]
    fn test_sum_chain_reactions() {
        let bricks = read_bricks(&INPUT);
        assert_eq!(sum_chain_reactions(bricks.as_slice()), 7);
    }
}
