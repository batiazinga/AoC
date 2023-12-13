#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for State {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(State::Operational),
            '#' => Ok(State::Damaged),
            '?' => Ok(State::Unknown),
            _ => Err("invalid state character"),
        }
    }
}

pub struct ConditionRecord {
    states: Vec<State>,
    group_sizes: Vec<u64>,
}

impl ConditionRecord {
    pub fn parse(input: &str) -> ConditionRecord {
        let mut record = ConditionRecord {
            states: Vec::new(),
            group_sizes: Vec::new(),
        };

        let states_end = input.find(|c: char| c.is_whitespace()).unwrap();
        input[..states_end]
            .chars()
            .map(|c| State::try_from(c).unwrap())
            .for_each(|s| record.states.push(s));

        let group_sizes_start = input.find(|c: char| c.is_digit(10)).unwrap();
        input[group_sizes_start..]
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .for_each(|n| record.group_sizes.push(n));

        record
    }

    pub fn unfold(&self) -> ConditionRecord {
        let mut record = ConditionRecord {
            states: Vec::with_capacity((self.states.len() + 1) * 5 - 1),
            group_sizes: Vec::with_capacity(self.group_sizes.len() * 5),
        };

        for i in 0..5 {
            for state in self.states.iter() {
                record.states.push(state.clone());
            }
            for group_size in self.group_sizes.iter() {
                record.group_sizes.push(*group_size);
            }
            if i != 4 {
                record.states.push(State::Unknown);
            }
        }

        record
    }

    fn num_damaged(&self) -> u64 {
        self.group_sizes.iter().sum()
    }

    fn num_known_damaged(&self) -> u64 {
        let mut count = 0;
        self.states.iter().for_each(|s| match s {
            State::Damaged => count += 1,
            _ => {}
        });
        count
    }

    fn num_unknown(&self) -> u64 {
        let mut count = 0;
        self.states.iter().for_each(|s| match s {
            State::Unknown => count += 1,
            _ => {}
        });
        count
    }

    fn is_valid_candidate(&self, candidate: &[State]) -> bool {
        let mut candidate_cursor = 0usize;
        let mut completed: Vec<State> = Vec::with_capacity(self.states.len());
        for state in &self.states {
            match state {
                State::Unknown => {
                    completed.push(candidate[candidate_cursor].clone());
                    candidate_cursor += 1;
                }
                _ => completed.push(state.clone()),
            }
        }

        group_sizes(completed.as_slice()) == self.group_sizes
    }

    pub fn count_arrangements(&self) -> u64 {
        let mut count = 0u64;
        self.generate_candidates_and_count(
            self.num_damaged() - self.num_known_damaged(),
            self.num_unknown(),
            &mut count,
        );

        count
    }

    fn generate_candidates_and_count(&self, num_damaged: u64, num_unknown: u64, counter: &mut u64) {
        self.rec_generate_candidates_and_count(num_damaged, num_unknown, 0, 0, counter);
    }

    fn rec_generate_candidates_and_count(
        &self,
        num_ones: u64,
        remaining_bits: u64,
        number: u64,
        cursor: u64,
        counter: &mut u64,
    ) {
        if cursor == self.num_unknown() {
            let candidate = to_states(number, self.num_unknown());
            if self.is_valid_candidate(candidate.as_slice()) {
                *counter += 1;
            }
            return;
        }

        if num_ones > 0 {
            let cp = number | 1 << cursor;
            self.rec_generate_candidates_and_count(
                num_ones - 1,
                remaining_bits - 1,
                cp,
                cursor + 1,
                counter,
            );
        }
        if remaining_bits > num_ones {
            self.rec_generate_candidates_and_count(
                num_ones,
                remaining_bits - 1,
                number,
                cursor + 1,
                counter,
            );
        }
    }
}

fn group_sizes(states: &[State]) -> Vec<u64> {
    let mut sizes: Vec<u64> = Vec::new();

    let mut size = 0u64;
    for s in states {
        match s {
            State::Damaged => size += 1,
            State::Operational => {
                if size != 0 {
                    sizes.push(size);
                    size = 0;
                }
            }
            State::Unknown => panic!("cannot compute group sizes with unknown states"),
        }
    }
    if size != 0 {
        sizes.push(size);
    }

    sizes
}

fn to_states(number: u64, num_bits: u64) -> Vec<State> {
    let mut states: Vec<State> = Vec::with_capacity(num_bits as usize);
    for i in 0..num_bits {
        let mask = 1 << i;
        if number & mask != 0 {
            states.push(State::Damaged);
        } else {
            states.push(State::Operational);
        }
    }
    states
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_read_condition_record() {
        let input = "???.### 1,1,3";
        let record = ConditionRecord::parse(&input);

        assert_eq!(record.num_damaged(), 5);
        assert_eq!(record.num_unknown(), 3);
    }

    #[test]
    fn test_compute_group_sizes() {
        let record = ConditionRecord::parse("#.#.### 1,1,3");
        let group_sizes = group_sizes(record.states.as_slice());
        assert_eq!(group_sizes, record.group_sizes);
    }

    #[test]
    fn test_is_valid_candidate() {
        let record = ConditionRecord::parse("???.### 1,1,3");

        assert!(record.is_valid_candidate(&[State::Damaged, State::Operational, State::Damaged]));
        assert!(!record.is_valid_candidate(&[
            State::Operational,
            State::Operational,
            State::Damaged
        ]));
    }

    #[test]
    fn test_to_states() {
        assert_eq!(
            to_states(3, 4),
            &[
                State::Damaged,
                State::Damaged,
                State::Operational,
                State::Operational
            ]
        );
    }

    #[test]
    fn test_count_arrangements_1() {
        let record = ConditionRecord::parse("???.### 1,1,3");
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn test_count_arrangements_2() {
        let record = ConditionRecord::parse(".??..??...?##. 1,1,3");
        assert_eq!(record.count_arrangements(), 4);
    }

    #[test]
    fn test_count_arrangements_3() {
        let record = ConditionRecord::parse("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn test_count_arrangements_4() {
        let record = ConditionRecord::parse("????.#...#... 4,1,1");
        assert_eq!(record.count_arrangements(), 1);
    }

    #[test]
    fn test_count_arrangements_5() {
        let record = ConditionRecord::parse("????.######..#####. 1,6,5");
        assert_eq!(record.count_arrangements(), 4);
    }

    #[test]
    fn test_count_arrangements_6() {
        let record = ConditionRecord::parse("?###???????? 3,2,1");
        assert_eq!(record.count_arrangements(), 10);
    }

    #[test]
    fn test_unfolded_count_arrangements_1() {
        let record = ConditionRecord::parse("???.### 1,1,3");
        assert_eq!(record.unfold().count_arrangements(), 1);
    }

    // #[test]
    // fn test_unfolded_count_arrangements_2() {
    //     let record = ConditionRecord::parse(".??..??...?##. 1,1,3");
    //     assert_eq!(record.unfold().count_arrangements(), 16384);
    // }

    // #[test]
    // fn test_unfolded_count_arrangements_3() {
    //     let record = ConditionRecord::parse("?#?#?#?#?#?#?#? 1,3,1,6");
    //     assert_eq!(record.unfold().count_arrangements(), 1);
    // }

    #[test]
    fn test_unfolded_count_arrangements_4() {
        let record = ConditionRecord::parse("????.#...#... 4,1,1");
        assert_eq!(record.unfold().count_arrangements(), 16);
    }

    #[test]
    fn test_unfolded_count_arrangements_5() {
        let record = ConditionRecord::parse("????.######..#####. 1,6,5");
        assert_eq!(record.unfold().count_arrangements(), 2500);
    }

    // #[test]
    // fn test_unfolded_count_arrangements_6() {
    //     let record = ConditionRecord::parse("?###???????? 3,2,1");
    //     assert_eq!(record.unfold().count_arrangements(), 506250);
    // }
}