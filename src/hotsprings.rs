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
    group_sizes: Vec<usize>,
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
            .map(|s| s.parse::<usize>().unwrap())
            .for_each(|n| record.group_sizes.push(n));

        record
    }

    pub fn unfold(&self, n: u64) -> ConditionRecord {
        let mut record = ConditionRecord {
            states: Vec::with_capacity((self.states.len() + 1) * (n as usize) - 1),
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

    fn num_damaged(&self) -> usize {
        self.group_sizes.iter().sum()
    }

    fn num_known_damaged(&self) -> usize {
        let mut count = 0;
        self.states.iter().for_each(|s| match s {
            State::Damaged => count += 1,
            _ => {}
        });
        count
    }

    fn num_unknown(&self) -> usize {
        let mut count = 0;
        self.states.iter().for_each(|s| match s {
            State::Unknown => count += 1,
            _ => {}
        });
        count
    }

    fn is_valid(&self, candidate: &[State]) -> bool {
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

    fn is_valid_so_far(&self, candidate: &[State]) -> bool {
        let mut candidate_cursor = 0usize;
        let mut partial: Vec<State> = Vec::with_capacity(self.states.len());
        for state in &self.states {
            if state == &State::Unknown && candidate_cursor < candidate.len() {
                partial.push(candidate[candidate_cursor].clone());
                candidate_cursor += 1;
            } else {
                partial.push(state.clone());
            }
        }

        let candiate_group_sizes = group_sizes(partial.as_slice());
        for i in 0..candiate_group_sizes.len() {
            if i < candiate_group_sizes.len() - 1 {
                if candiate_group_sizes[i] != self.group_sizes[i] {
                    return false;
                }
            } else {
                if candiate_group_sizes[i] > self.group_sizes[i] {
                    return false;
                }
            }
        }
        true
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

    fn generate_candidates_and_count(
        &self,
        num_damaged_left: usize,
        num_unknown: usize,
        counter: &mut u64,
    ) {
        let mut candidate: Vec<State> = vec![State::Unknown; num_unknown as usize];
        self.rec_generate_candidates_and_count(num_damaged_left, &mut candidate, 0, counter);
    }

    fn rec_generate_candidates_and_count(
        &self,
        num_damaged_left: usize,
        candidate: &mut Vec<State>,
        cursor: usize,
        counter: &mut u64,
    ) {
        if cursor == candidate.len() {
            if self.is_valid(candidate.as_slice()) {
                *counter += 1;
            }
            return;
        }

        if !self.is_valid_so_far(candidate.as_slice()) {
            return;
        }

        if num_damaged_left > 0 {
            candidate[cursor] = State::Damaged;
            self.rec_generate_candidates_and_count(
                num_damaged_left - 1,
                candidate,
                cursor + 1,
                counter,
            );
            candidate[cursor] = State::Unknown;
        }
        if candidate.len() - cursor > num_damaged_left as usize {
            candidate[cursor] = State::Operational;
            self.rec_generate_candidates_and_count(
                num_damaged_left,
                candidate,
                cursor + 1,
                counter,
            );
            candidate[cursor] = State::Unknown;
        }
    }
}

fn group_sizes(states: &[State]) -> Vec<usize> {
    let mut sizes: Vec<usize> = Vec::new();

    let mut size = 0usize;
    for s in states {
        match s {
            State::Damaged => size += 1,
            State::Operational => {
                if size != 0 {
                    sizes.push(size);
                    size = 0;
                }
            }
            State::Unknown => {
                break;
            }
        }
    }
    if size != 0 {
        sizes.push(size);
    }

    sizes
}

pub fn read_records(input: &str) -> Vec<ConditionRecord> {
    let mut records: Vec<ConditionRecord> = Vec::new();

    for line in input.lines() {
        records.push(ConditionRecord::parse(line));
    }

    records
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
        assert_eq!(
            group_sizes(&[State::Damaged, State::Operational, State::Damaged]),
            &[1, 1]
        );
        assert_eq!(
            group_sizes(&[
                State::Damaged,
                State::Operational,
                State::Damaged,
                State::Operational,
                State::Damaged,
                State::Damaged,
                State::Damaged
            ]),
            &[1, 1, 3]
        );
        assert_eq!(
            group_sizes(&[
                State::Operational,
                State::Damaged,
                State::Operational,
                State::Damaged,
                State::Unknown,
                State::Damaged
            ]),
            &[1, 1]
        );
    }

    #[test]
    fn test_is_valid_candidate() {
        let record = ConditionRecord::parse("???.### 1,1,3");

        assert!(record.is_valid(&[State::Damaged, State::Operational, State::Damaged]));
        assert!(!record.is_valid(&[State::Operational, State::Operational, State::Damaged]));
    }

    #[test]
    fn test_is_valid_so_far() {
        let record = ConditionRecord::parse("?#?#?#?#?#?#?#? 1,3,1,6");

        assert!(record.is_valid_so_far(&[State::Operational]));
        assert!(!record.is_valid_so_far(&[State::Damaged]));
        assert!(record.is_valid_so_far(&[State::Operational, State::Operational]));
        assert!(!record.is_valid_so_far(&[State::Operational, State::Damaged]));
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
        assert_eq!(record.unfold(5).count_arrangements(), 1);
    }

    #[test]
    fn test_unfolded_count_arrangements_2() {
        let record = ConditionRecord::parse(".??..??...?##. 1,1,3");
        assert_eq!(record.unfold(5).count_arrangements(), 16384);
    }

    #[test]
    fn test_unfolded_count_arrangements_3() {
        let record = ConditionRecord::parse("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(record.unfold(5).count_arrangements(), 1);
    }

    #[test]
    fn test_unfolded_count_arrangements_4() {
        let record = ConditionRecord::parse("????.#...#... 4,1,1");
        assert_eq!(record.unfold(5).count_arrangements(), 16);
    }

    #[test]
    fn test_unfolded_count_arrangements_5() {
        let record = ConditionRecord::parse("????.######..#####. 1,6,5");
        assert_eq!(record.unfold(5).count_arrangements(), 2500);
    }

    // #[test]
    // fn test_unfolded_count_arrangements_6() {
    //     let record = ConditionRecord::parse("?###???????? 3,2,1");
    //     assert_eq!(record.unfold(5).count_arrangements(), 506250);
    // }
}
