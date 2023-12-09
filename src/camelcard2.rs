use std::{cmp::Ordering, collections::BTreeMap};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Card {
    A = 14,
    K = 13,
    Q = 12,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    J = 1,
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'T' => Ok(Card::T),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            'J' => Ok(Card::J),
            _ => Err("invalid card character"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum HandType {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPair = 2,
    Pair = 1,
    HighCard = 0,
}

#[derive(Eq, Debug)]
pub struct Hand {
    cards: [Card; 5],
}

impl Hand {
    pub fn parse(hand: &str) -> Hand {
        let mut h = Hand {
            cards: [Card::Two; 5],
        };

        let mut cursor: usize = 0;
        for c in hand.chars() {
            h.cards[cursor] = Card::try_from(c).unwrap();
            cursor += 1;
        }

        h
    }

    pub fn hand_type(&self) -> HandType {
        let mut counter: BTreeMap<Card, u8> = BTreeMap::new();
        let mut count_j = 0u8;
        for card in self.cards {
            if card == Card::J {
                count_j += 1;
            } else {
                counter.entry(card).and_modify(|c| *c += 1).or_insert(1u8);
            }
        }
        if count_j==5{
            return HandType::FiveOfKind;
        }

        let mut histogram: [u8; 5] = [0; 5];
        for count in counter.values() {
            histogram[*count as usize - 1] += 1;
        }
        for i in 0..5 {
            let rev_i = histogram.len() - 1 - i;
            if histogram[rev_i] != 0 {
                histogram[rev_i] -= 1;
                histogram[rev_i + count_j as usize] += 1;
                break;
            }
        }

        if histogram[4] == 1 {
            return HandType::FiveOfKind;
        }
        if histogram[3] == 1 {
            return HandType::FourOfKind;
        }
        if histogram[2] == 1 {
            if histogram[1] == 1 {
                return HandType::FullHouse;
            }
            return HandType::ThreeOfKind;
        }
        if histogram[1] == 2 {
            return HandType::TwoPair;
        }
        if histogram[1] == 1 {
            return HandType::Pair;
        }
        HandType::HighCard
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type().ne(&other.hand_type()) {
            return false;
        }
        for i in 0..5 {
            if self.cards[i].ne(&other.cards[i]) {
                return false;
            }
        }
        true
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ord = self.hand_type().cmp(&other.hand_type());
        if ord != Ordering::Equal {
            return ord;
        }
        for i in 0..5 {
            ord = self.cards[i].cmp(&other.cards[i]);
            if ord != Ordering::Equal {
                return ord;
            }
        }
        ord
    }
}

pub fn total_winnings2(bids: &mut Vec<(Hand, u64)>) -> u64 {
    bids.sort_by(|x, y| x.0.cmp(&y.0));
    let mut winnings: u64 = 0;
    for i in 0..bids.len() {
        winnings += (i as u64 + 1) * bids[i].1
    }
    winnings
}

pub fn read_bids2(input: &str) -> Vec<(Hand, u64)> {
    let mut bids: Vec<(Hand, u64)> = Vec::new();
    for line in input.lines() {
        bids.push((Hand::parse(&line[0..5]), line[5..].trim().parse().unwrap()));
    }
    bids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_comparison() {
        assert!(Card::A > Card::K);
        assert!(Card::K > Card::Q);
        assert!(Card::Q > Card::T);
        assert!(Card::T > Card::Nine);
        assert!(Card::Nine > Card::Eight);
        assert!(Card::Eight > Card::Seven);
        assert!(Card::Seven > Card::Six);
        assert!(Card::Six > Card::Five);
        assert!(Card::Five > Card::Four);
        assert!(Card::Four > Card::Three);
        assert!(Card::Three > Card::Two);
        assert!(Card::Two > Card::J);
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(Hand::parse("22222").hand_type(), HandType::FiveOfKind);
        assert_eq!(Hand::parse("22J22").hand_type(), HandType::FiveOfKind);
        assert_eq!(Hand::parse("QQQQA").hand_type(), HandType::FourOfKind);
        assert_eq!(Hand::parse("QQQJA").hand_type(), HandType::FourOfKind);
        assert_eq!(Hand::parse("TJ555").hand_type(), HandType::FourOfKind);
        assert_eq!(Hand::parse("KTJJT").hand_type(), HandType::FourOfKind);
        assert_eq!(Hand::parse("T5TJ5").hand_type(), HandType::FullHouse);
        assert_eq!(Hand::parse("KK677").hand_type(), HandType::TwoPair);
        assert_eq!(Hand::parse("32T3K").hand_type(), HandType::Pair);
        assert_eq!(Hand::parse("42T3K").hand_type(), HandType::HighCard);
        assert_eq!(Hand::parse("42J3K").hand_type(), HandType::Pair);
        assert_eq!(Hand::parse("QJJQ2").hand_type(), HandType::FourOfKind);
        assert_eq!(Hand::parse("JJJJJ").hand_type(), HandType::FiveOfKind);
    }

    #[test]
    fn test_hand_comparison() {
        assert!(Hand::parse("KTJJT") > Hand::parse("QQQJA"));
        assert!(Hand::parse("QQQJA") > Hand::parse("T55J5"));
        assert!(Hand::parse("T55J5") > Hand::parse("KK677"));
        assert!(Hand::parse("KK677") > Hand::parse("32T3K"));
        assert!(Hand::parse("QQQQ2") > Hand::parse("JKKK2"));
        assert!(Hand::parse("KTJJT") == Hand::parse("KTJJT"));
    }

    #[test]
    fn test_total_winnings() {
        let mut bids: Vec<(Hand, u64)> = Vec::from([
            (Hand::parse("32T3K"), 765),
            (Hand::parse("T55J5"), 684),
            (Hand::parse("KK677"), 28),
            (Hand::parse("KTJJT"), 220),
            (Hand::parse("QQQJA"), 483),
        ]);
        assert_eq!(total_winnings2(&mut bids), 5905);
    }
}
