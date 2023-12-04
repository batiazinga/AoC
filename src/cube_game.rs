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

struct Draw {
    r: u32,
    g: u32,
    b: u32,
}

impl Draw {
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

struct Game {
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
    fn test_is_draw_possible() {
        assert!(Draw { r: 3, g: 4, b: 1 }.is_possible());
        assert!(!Draw { r: 8, g: 6, b: 15 }.is_possible());
        assert!(!Draw { r: 11, g: 14, b: 3 }.is_possible());
        assert!(!Draw { r: 13, g: 11, b: 0 }.is_possible());
    }

    #[test]
    fn test_is_game_possible() {
        let mut game = Game::new(1);
        game.add(Draw { r: 3, g: 4, b: 1 });
        game.add(Draw { r: 5, g: 8, b: 3 });
        assert!(game.is_possible());

        game.add(Draw { r: 8, g: 14, b: 7 });
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
