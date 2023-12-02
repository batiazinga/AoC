pub fn sum_codes(msg: &str) -> u32 {
    let mut total = 0;
    for line in msg.lines() {
        total += code_from_line(line);
    }
    total
}

fn code_from_line(line: &str) -> u32 {
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;
    let mut found_first_digit = false;
    let mut s = line;
    while let (Some(digit), remaining_line) = next_digit(s) {
        last_digit = digit;
        if !found_first_digit {
            first_digit = digit;
            found_first_digit = true;
        }
        s = remaining_line;
    }
    first_digit * 10 + last_digit
}

fn next_digit(line: &str) -> (Option<u32>, &str) {
    let mut chars = line.chars();
    let mut remaining_line = line;
    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            return (Some(c.to_digit(10).unwrap()), &remaining_line[1..]);
        }
        if remaining_line.starts_with("zero") {
            return (Some(0), &remaining_line[1..]);
        }
        if remaining_line.starts_with("one") {
            return (Some(1), &remaining_line[1..]);
        }
        if remaining_line.starts_with("two") {
            return (Some(2), &remaining_line[1..]);
        }
        if remaining_line.starts_with("three") {
            return (Some(3), &remaining_line[1..]);
        }
        if remaining_line.starts_with("four") {
            return (Some(4), &remaining_line[1..]);
        }
        if remaining_line.starts_with("five") {
            return (Some(5), &remaining_line[1..]);
        }
        if remaining_line.starts_with("six") {
            return (Some(6), &remaining_line[1..]);
        }
        if remaining_line.starts_with("seven") {
            return (Some(7), &remaining_line[1..]);
        }
        if remaining_line.starts_with("eight") {
            return (Some(8), &remaining_line[1..]);
        }
        if remaining_line.starts_with("nine") {
            return (Some(9), &remaining_line[1..]);
        }

        remaining_line = &remaining_line[c.len_utf8()..];
    }
    (None, "")
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn sum_game_ids(msg: &str) -> u32 {
    let mut total = 0;
    for line in msg.lines() {
        let g = Game::parse(line);
        if g.is_possible() {
            total += g.id();
        }
    }
    total
}

pub fn sum_min_cubes_powers(msg: &str) -> u32 {
    let mut total = 0;
    for line in msg.lines() {
        let g = Game::parse(line);
        let cubes = g.min_cubes();
        total += cubes.0 * cubes.1 * cubes.2;
    }
    total
}

pub struct Draw {
    r: u32,
    g: u32,
    b: u32,
}

impl Draw {
    pub fn new(r: u32, g: u32, b: u32) -> Draw {
        Draw { r, g, b }
    }

    pub fn parse(s: &str) -> Draw {
        let mut r: u32 = 0;
        let mut g: u32 = 0;
        let mut b: u32 = 0;

        s.split(",").for_each(|str_cubes| {
            let s = str_cubes.trim();
            let mut iter = s.split_whitespace();
            let num: u32 = iter.next().unwrap().parse().unwrap();
            let color: &str = iter.next().unwrap();
            if color == "red" {
                r += num;
            } else if color == "green" {
                g += num;
            } else {
                b += num;
            }
        });

        Draw { r, g, b }
    }

    pub fn is_possible(&self) -> bool {
        if self.r > MAX_RED || self.g > MAX_GREEN || self.b > MAX_BLUE {
            return false;
        }
        true
    }
}

pub struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    pub fn parse(msg: &str) -> Game {
        let s = msg.strip_prefix("Game ").unwrap();
        let column_index = s.find(":").unwrap();
        let id: u32 = (&s[0..column_index]).parse().unwrap();
        let mut game = Game::new(id);

        let _ = &s[column_index + 1..]
            .split(";")
            .for_each(|str_draw| game.add(Draw::parse(str_draw)));

        game
    }

    pub fn new(id: u32) -> Game {
        Game {
            id,
            draws: Vec::new(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn add(&mut self, d: Draw) {
        self.draws.push(d);
    }

    pub fn add_rgb(&mut self, r: u32, g: u32, b: u32) {
        self.draws.push(Draw::new(r, g, b));
    }

    pub fn is_possible(&self) -> bool {
        for draw in self.draws.as_slice() {
            if !draw.is_possible() {
                return false;
            }
        }
        true
    }

    pub fn min_cubes(&self) -> (u32, u32, u32) {
        let mut min_r: u32 = 0;
        let mut min_g: u32 = 0;
        let mut min_b: u32 = 0;

        for draw in self.draws.as_slice() {
            if draw.r > min_r {
                min_r = draw.r;
            }
            if draw.g > min_g {
                min_g = draw.g;
            }
            if draw.b > min_b {
                min_b = draw.b;
            }
        }

        (min_r, min_g, min_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_digit() {
        let (mut next, remaining) = next_digit("eightwothree");
        assert_eq!(next.unwrap(), 8);
        assert_eq!(remaining, "ightwothree");
        (next, _) = next_digit(remaining);
        assert_eq!(next.unwrap(), 2);
    }

    #[test]
    fn test_code_from_line() {
        assert_eq!(code_from_line("1abc2"), 12);
        assert_eq!(code_from_line("pqr3stu8vwx"), 38);
        assert_eq!(code_from_line("a1b2c3d4e5f"), 15);
        assert_eq!(code_from_line("treb7uchet"), 77);

        assert_eq!(code_from_line("two1nine"), 29);
        assert_eq!(code_from_line("eightwothree"), 83);
        assert_eq!(code_from_line("abcone2threexyz"), 13);
        assert_eq!(code_from_line("xtwone3four"), 24);
        assert_eq!(code_from_line("4nineeightseven2"), 42);
        assert_eq!(code_from_line("zoneight234"), 14);
        assert_eq!(code_from_line("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_code() {
        assert_eq!(
            sum_codes("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"),
            142
        );
        assert_eq!(sum_codes("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"), 281);
    }

    #[test]
    fn test_is_draw_possible() {
        assert!(Draw::new(3, 4, 1).is_possible());
        assert!(!Draw::new(8, 6, 15).is_possible());
        assert!(!Draw::new(11, 14, 3).is_possible());
        assert!(!Draw::new(13, 11, 0).is_possible());
    }

    #[test]
    fn test_is_game_possible() {
        let mut game = Game::new(1);
        game.add_rgb(3, 4, 1);
        game.add_rgb(5, 8, 3);
        assert!(game.is_possible());

        game.add_rgb(8, 14, 7);
        assert!(!game.is_possible());
    }

    #[test]
    fn test_parse_game() {
        let g1 = Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(g1.id(), 1);
        assert!(g1.is_possible());

        let g2 = Game::parse("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        assert_eq!(g2.id(), 2);
        assert!(g2.is_possible());

        let g3 =
            Game::parse("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        assert_eq!(g3.id(), 3);
        assert!(!g3.is_possible());

        let g4 =
            Game::parse("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        assert_eq!(g4.id(), 4);
        assert!(!g4.is_possible());

        let g5 = Game::parse("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(g5.id(), 5);
        assert!(g5.is_possible());
    }

    #[test]
    fn test_sum_game_ids() {
        assert_eq!(
            sum_game_ids(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        )
    }
}
