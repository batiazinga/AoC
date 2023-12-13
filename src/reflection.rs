pub fn summarize(input: &str) -> u64 {
    let mut summary = 0u64;

    let mut lines = input.lines();
    while let Some(mut line) = lines.next() {
        let mut m: Vec<Vec<char>> = Vec::new();
        while !line.is_empty() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            m.push(row);

            let line_opt = lines.next();
            if line_opt.is_none() {
                break;
            }
            line = line_opt.unwrap();
        }
        summary += summarize_one_map(&m);
    }

    summary
}

fn summarize_one_map(m: &Vec<Vec<char>>) -> u64 {
    let num_rows = m.len();
    if num_rows == 0 {
        return 0;
    }
    let num_columns = if num_rows > 0 { m[0].len() } else { 0 };
    if num_columns == 0 {
        return 0;
    }

    for j in 1..num_columns {
        let mut is_mirror = true;
        let max = if num_columns - j > j {
            j
        } else {
            num_columns - j
        };
        'outer: for k in 0..max {
            for i in 0..num_rows {
                if m[i][j - 1 - k] != m[i][j + k] {
                    is_mirror = false;
                    break 'outer;
                }
            }
        }
        if is_mirror {
            return j as u64;
        }
    }

    for i in 1..num_rows {
        let mut is_mirror = true;
        let max = if num_rows - i > i { i } else { num_rows - i };
        'outer: for k in 0..max {
            for j in 0..num_columns {
                if m[i - 1 - k][j] != m[i + k][j] {
                    is_mirror = false;
                    break 'outer;
                }
            }
        }
        if is_mirror {
            return 100 * i as u64;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarize() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        assert_eq!(summarize(&input), 405);
    }
}
