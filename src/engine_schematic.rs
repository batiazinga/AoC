struct Number {
    line: usize,
    column: (usize, usize),
    value: u32,
}

struct Symbol {
    line: usize,
    column: usize,
    value: char,
}

pub struct EngineSchematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl EngineSchematic {
    pub fn parse(input: &str) -> EngineSchematic {
        let p = Parser::new(input);
        let mut e = EngineSchematic {
            numbers: Vec::new(),
            symbols: Vec::new(),
        };

        for token in p.tokens() {
            match token.value() {
                TokenValue::Number(n) => e.numbers.push(Number {
                    line: token.line(),
                    column: token.column(),
                    value: *n,
                }),
                TokenValue::Symbol(s) => e.symbols.push(Symbol {
                    line: token.line(),
                    column: token.column().0,
                    value: *s,
                }),
            }
        }

        e
    }

    pub fn part_numbers(&self) -> impl Iterator<Item = u32> + '_ {
        self.numbers
            .iter()
            .filter(|n| self.is_part_number(n))
            .map(|n| n.value)
    }

    pub fn gear_ratios(&self) -> impl Iterator<Item = u32> + '_ {
        self.symbols.iter().filter_map(|s| self.gear_ratio(s))
    }

    fn is_part_number(&self, number: &Number) -> bool {
        for symbol in &self.symbols {
            if number.line > 1 && symbol.line < number.line - 1 {
                continue;
            }
            if symbol.line > number.line + 1 {
                break;
            }

            if (number.column.0 <= 1 || symbol.column >= number.column.0 - 1)
                && symbol.column <= number.column.1
            {
                return true;
            }
        }
        false
    }

    fn gear_ratio(&self, symbol: &Symbol) -> Option<u32> {
        if symbol.value != '*' {
            return None;
        }

        let mut numbers: Vec<&Number> = Vec::new();
        for number in &self.numbers {
            if symbol.line > 1 && number.line < symbol.line - 1 {
                continue;
            }
            if number.line > symbol.line + 1 {
                break;
            }

            if (number.column.0 <= 1 || symbol.column >= number.column.0 - 1)
                && symbol.column <= number.column.1
            {
                numbers.push(&number);
            }
        }
        if numbers.len() != 2 {
            return None;
        }

        Some(numbers[0].value * numbers[1].value)
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    Number(u32),
    Symbol(char),
}
pub struct Token {
    line: usize,
    column: (usize, usize),
    value: TokenValue,
}

impl Token {
    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> (usize, usize) {
        self.column
    }

    pub fn value(&self) -> &TokenValue {
        &self.value
    }
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let mut p = Parser { tokens: Vec::new() };
        p.parse(input, 0, 0);
        p
    }

    fn parse(&mut self, input: &str, line: usize, column: usize) {
        if let Some(start) = input.find(|c: char| c != '.') {
            let c = input[start..].chars().next().unwrap();

            if c == '\n' {
                self.parse(&input[start + 1..], line + 1, 0);
            } else if c.is_digit(10) {
                let end = start
                    + match input[start..].find(|c: char| !c.is_digit(10)) {
                        Some(i) => i,
                        None => input[start..].len(),
                    };
                let num: u32 = input[start..end].parse().unwrap();
                self.tokens.push(Token {
                    line: line,
                    column: (column + start, column + end),
                    value: TokenValue::Number(num),
                });
                self.parse(&input[end..], line, column + end);
            } else {
                self.tokens.push(Token {
                    line: line,
                    column: (column + start, column + start + 1),
                    value: TokenValue::Symbol(c),
                });
                self.parse(&input[start + 1..], line, column + start + 1);
            }
        }
    }

    pub fn tokens(&self) -> impl Iterator<Item = &Token> {
        self.tokens.as_slice().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_string() {
        assert_eq!(Parser::new("").tokens().count(), 0);
    }

    #[test]
    fn test_parse_line_with_numbers() {
        let parser = Parser::new("36..1..3675");
        let mut tokens = parser.tokens();

        let first = tokens.next().unwrap();
        assert_eq!(first.line(), 0);
        assert_eq!(first.column(), (0, 2));
        assert_eq!(first.value(), &TokenValue::Number(36));

        let second = tokens.next().unwrap();
        assert_eq!(second.line(), 0);
        assert_eq!(second.column(), (4, 5));
        assert_eq!(second.value(), &TokenValue::Number(1));

        let third = tokens.next().unwrap();
        assert_eq!(third.line(), 0);
        assert_eq!(third.column(), (7, 11));
        assert_eq!(third.value(), &TokenValue::Number(3675));

        assert!(tokens.next().is_none());
    }

    #[test]
    fn test_parse_lines_with_numbers() {
        let parser = Parser::new("..36..1..\n78..285..");
        let mut tokens = parser.tokens();

        let first = tokens.next().unwrap();
        assert_eq!(first.line(), 0);
        assert_eq!(first.column(), (2, 4));
        assert_eq!(first.value(), &TokenValue::Number(36));

        let second = tokens.next().unwrap();
        assert_eq!(second.line(), 0);
        assert_eq!(second.column(), (6, 7));
        assert_eq!(second.value(), &TokenValue::Number(1));

        let third = tokens.next().unwrap();
        assert_eq!(third.line(), 1);
        assert_eq!(third.column(), (0, 2));
        assert_eq!(third.value(), &TokenValue::Number(78));

        let fourth = tokens.next().unwrap();
        assert_eq!(fourth.line(), 1);
        assert_eq!(fourth.column(), (4, 7));
        assert_eq!(fourth.value(), &TokenValue::Number(285));

        assert!(tokens.next().is_none());
    }

    #[test]
    fn test_parse_numbers_and_symbols() {
        let parser = Parser::new(
            "467$..114.\n\
        ...*......",
        );
        let mut tokens = parser.tokens();

        let first = tokens.next().unwrap();
        assert_eq!(first.line(), 0);
        assert_eq!(first.column(), (0, 3));
        assert_eq!(first.value(), &TokenValue::Number(467));

        let second = tokens.next().unwrap();
        assert_eq!(second.line(), 0);
        assert_eq!(second.column(), (3, 4));
        assert_eq!(second.value(), &TokenValue::Symbol('$'));

        let third = tokens.next().unwrap();
        assert_eq!(third.line(), 0);
        assert_eq!(third.column(), (6, 9));
        assert_eq!(third.value(), &TokenValue::Number(114));

        let fourth = tokens.next().unwrap();
        assert_eq!(fourth.line(), 1);
        assert_eq!(fourth.column(), (3, 4));
        assert_eq!(fourth.value(), &TokenValue::Symbol('*'));

        assert!(tokens.next().is_none());
    }

    #[test]
    fn test_no_part_number_if_empty_input() {
        let e = EngineSchematic::parse("");
        assert_eq!(e.part_numbers().count(), 0);
    }

    #[test]
    fn test_part_numbers() {
        let e = EngineSchematic::parse(
            "467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..",
        );
        let mut part_numbers = e.part_numbers();
        assert_eq!(part_numbers.next(), Some(467));
        assert_eq!(part_numbers.next(), Some(35));
        assert_eq!(part_numbers.next(), Some(633));
        assert_eq!(part_numbers.next(), Some(617));
        assert_eq!(part_numbers.next(), Some(592));
        assert_eq!(part_numbers.next(), Some(755));
        assert_eq!(part_numbers.next(), Some(664));
        assert_eq!(part_numbers.next(), Some(598));
        assert!(part_numbers.next().is_none());
    }

    #[test]
    fn test_gear_ratios() {
        let e = EngineSchematic::parse(
            "467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..",
        );
        let mut gear_ratios = e.gear_ratios();
        assert_eq!(gear_ratios.next(), Some(467 * 35));
        assert_eq!(gear_ratios.next(), Some(755 * 598));
        assert!(gear_ratios.next().is_none());
    }
}