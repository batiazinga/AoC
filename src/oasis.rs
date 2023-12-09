pub fn extrapolate_next(values: &[i64]) -> i64 {
    let mut diff: Vec<i64> = Vec::with_capacity(values.len() - 1);
    for i in 1..values.len() {
        diff.push(values[i] - values[i - 1]);
    }
    if values.iter().all(|n| *n == 0) {
        return values.last().unwrap().clone();
    }

    let next = extrapolate_next(diff.as_slice());
    values.last().unwrap() + next
}

pub fn extrapolate_previous(values: &[i64]) -> i64 {
    let mut diff: Vec<i64> = Vec::with_capacity(values.len() - 1);
    for i in 1..values.len() {
        diff.push(values[i] - values[i - 1]);
    }
    if values.iter().all(|n| *n == 0) {
        return values.first().unwrap().clone();
    }

    let previous = extrapolate_previous(diff.as_slice());
    values.first().unwrap() - previous
}

fn read_series(input: &str) -> Vec<Vec<i64>> {
    let mut series: Vec<Vec<i64>> = Vec::new();
    for line in input.lines() {
        let mut numbers: Vec<i64> = Vec::new();
        line.split_whitespace()
            .map(|str_num| str_num.parse::<i64>().unwrap())
            .for_each(|n| numbers.push(n));
        series.push(numbers);
    }

    series
}

pub fn sum_extrapolated_next_values(input: &str) -> i64 {
    let series = read_series(input);

    series
        .iter()
        .map(|values| extrapolate_next(values.as_slice()))
        .sum()
}

pub fn sum_extrapolated_previous_values(input: &str) -> i64 {
    let series = read_series(input);

    series
        .iter()
        .map(|values| extrapolate_previous(values.as_slice()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrapolate_constant() {
        let values: Vec<i64> = Vec::from([3, 3, 3, 3]);
        assert_eq!(extrapolate_next(values.as_slice()), 3);
    }

    #[test]
    fn test_extrapolate_constant_step() {
        let values: Vec<i64> = Vec::from([0, 3, 6, 9, 12, 15]);
        let next = extrapolate_next(values.as_slice());
        assert_eq!(next, 18);
    }

    #[test]
    fn test_extrapolate_linearly_increasing_step() {
        let values: Vec<i64> = Vec::from([10, 13, 16, 21, 30, 45]);
        let next = extrapolate_next(values.as_slice());
        assert_eq!(next, 68);
    }

    #[test]
    fn test_sum_extrapolated_next_values() {
        assert_eq!(
            sum_extrapolated_next_values(
                "0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45"
            ),
            114
        )
    }

    #[test]
    fn test_sum_extrapolated_previous_values() {
        assert_eq!(
            sum_extrapolated_previous_values(
                "0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45"
            ),
            2
        )
    }
}
