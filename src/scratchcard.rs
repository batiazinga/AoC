use std::collections::BTreeSet;

pub struct Card {
    id: usize,
    winning: Vec<u32>,
    numbers: BTreeSet<u32>,
}

impl Card {
    pub fn parse(input: &str) -> Card {
        let first_digit_index = input.find(|c: char| c.is_digit(10)).unwrap();
        let column_index = input.find(":").unwrap();
        let content = &input[column_index + 1..];
        let separator_index = content.find('|').unwrap();
        let str_winning = &content[..separator_index];
        let str_numbers = &content[separator_index + 1..];

        let mut c = Card {
            id: input[first_digit_index..column_index].parse().unwrap(),
            winning: Vec::new(),
            numbers: BTreeSet::new(),
        };
        c.winning.extend(
            str_winning
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok()),
        );
        c.numbers.extend(
            str_numbers
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok()),
        );

        c
    }

    fn num_wins(&self) -> usize {
        self.winning
            .iter()
            .filter(|w| self.numbers.contains(w))
            .count()
    }

    pub fn value(&self) -> u32 {
        match self.num_wins() {
            0 => 0,
            i => 1 << (i - 1),
        }
    }
}

pub fn num_cards(cards: impl Iterator<Item = Card>) -> u32 {
    let mut card_numbers: Vec<u32> = Vec::new();
    for card in cards {
        if card_numbers.len() < card.id {
            card_numbers.push(0);
        }
        card_numbers[card.id - 1] += 1;

        let num_wins = card.num_wins();
        while card_numbers.len() < card.id + num_wins {
            card_numbers.push(0);
        }
        for i in 1..=card.num_wins() {
            card_numbers[card.id - 1 + i] += card_numbers[card.id - 1];
        }
    }

    card_numbers.iter().sum()
}
