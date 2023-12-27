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

    pub fn unfold(&self, n: usize) -> ConditionRecord {
        let mut record = ConditionRecord {
            states: Vec::with_capacity((self.states.len() + 1) * (n as usize) - 1),
            group_sizes: Vec::with_capacity(self.group_sizes.len() * n),
        };

        for i in 0..n {
            for state in self.states.iter() {
                record.states.push(state.clone());
            }
            for group_size in self.group_sizes.iter() {
                record.group_sizes.push(*group_size);
            }
            if i != n - 1 {
                record.states.push(State::Unknown);
            }
        }

        record
    }

    pub fn count_arrangements(&self) -> u64 {
        let mut count = 0;
        let mut groups: Vec<PositionedGroup> = Vec::with_capacity(self.group_sizes.len());
        self.rec_count_arrangements(&mut groups, 0, &mut count);

        count
    }

    fn rec_count_arrangements(
        &self,
        groups: &mut Vec<PositionedGroup>,
        cursor: usize,
        count: &mut u64,
    ) {
        if groups.len() == self.group_sizes.len() {
            *count += 1;
            return;
        }

        let group_size = self.group_sizes[groups.len()];
        let required_size = if groups.len() == self.group_sizes.len() {
            group_size
        } else {
            group_size + 1
        };
        let min_required_size: usize = self.group_sizes[groups.len()..].iter().sum::<usize>()
            + (self.group_sizes.len() - groups.len() - 1);
        for i in cursor..self.states.len() - min_required_size + 1 {
            groups.push(PositionedGroup {
                size: group_size,
                position: i,
            });

            if self.is_valid(groups.as_slice()) {
                self.rec_count_arrangements(groups, i + required_size, count);
            }

            groups.pop();
        }
    }

    fn is_valid(&self, groups: &[PositionedGroup]) -> bool {
        if groups.len() == 0 {
            return true;
        }

        for i in 0..groups[0].position {
            if self.states[i] == State::Damaged {
                return false;
            }
        }

        for i in 0..groups.len() {
            let group = &groups[i];
            for j in 0..group.size {
                if self.states[group.position + j] == State::Operational {
                    return false;
                }
            }
            if i < groups.len() - 1 {
                let next_group = &groups[i + 1];
                for j in (group.position + group.size)..next_group.position {
                    if self.states[j] == State::Damaged {
                        return false;
                    }
                }
            }
        }

        let last_group = &groups[groups.len() - 1];
        if groups.len() == self.group_sizes.len() {
            for i in last_group.position + last_group.size..self.states.len() {
                if self.states[i] == State::Damaged {
                    return false;
                }
            }
        } else {
            if self.states[last_group.position + last_group.size] == State::Damaged {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
struct PositionedGroup {
    size: usize,
    position: usize,
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

    #[test]
    fn test_unfolded_count_arrangements_6() {
        let record = ConditionRecord::parse("?###???????? 3,2,1");
        assert_eq!(record.unfold(5).count_arrangements(), 506250);
    }
}
