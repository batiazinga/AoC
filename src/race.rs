pub fn prod_num_possibilities(input: &str) -> u64 {
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    let mut lines = input.lines();
    let line_times = lines.next().unwrap();
    let mut first_digit_index = line_times.find(|c: char| c.is_digit(10)).unwrap();
    line_times[first_digit_index..]
        .split_whitespace()
        .map(|str_num| str_num.parse::<u64>().unwrap())
        .for_each(|time| times.push(time));

    let line_dists = lines.next().unwrap();
    first_digit_index = line_dists.find(|c: char| c.is_digit(10)).unwrap();
    line_dists[first_digit_index..]
        .split_whitespace()
        .map(|str_num| str_num.parse::<u64>().unwrap())
        .for_each(|dist| distances.push(dist));

    let mut prod: u64 = 1;
    for i in 0..times.len() {
        let (lower, upper) = range(times[i], distances[i]);
        prod *= upper - lower + 1;
    }
    prod
}

pub fn num_possibilities(input: &str) -> u64 {
    let mut lines = input.lines();
    let line_times = lines.next().unwrap();
    let mut first_digit_index = line_times.find(|c: char| c.is_digit(10)).unwrap();
    let mut str_time: String = "".to_string();
    line_times[first_digit_index..]
        .split_whitespace()
        .for_each(|s| str_time.push_str(s));
    let time = str_time.parse::<u64>().unwrap();

    let line_dists = lines.next().unwrap();
    first_digit_index = line_dists.find(|c: char| c.is_digit(10)).unwrap();
    let mut str_dist: String = "".to_string();
    line_dists[first_digit_index..]
        .split_whitespace()
        .for_each(|s| str_dist.push_str(s));
    let dist = str_dist.parse::<u64>().unwrap();

    let (lower, upper) = range(time, dist);
    upper - lower + 1
}

fn distance(time: u64, speed: u64) -> u64 {
    (time - speed) * speed
}
fn range(time: u64, dist: u64) -> (u64, u64) {
    let f_time = time as f64;
    let f_dist = dist as f64;
    let f_range = (f_time * f_time - 4.0 * f_dist).sqrt();
    let f_lower = f_time / 2.0 - f_range / 2.0;
    let f_upper = f_time / 2.0 + f_range / 2.0;

    let mut lower = f_lower.ceil() as u64;
    if distance(time, lower) == dist {
        lower += 1;
    }
    let mut upper = f_upper.floor() as u64;
    if distance(time, upper) == dist {
        upper -= 1;
    }

    (lower, upper)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_time() {
        let time: u64 = 7;
        let distance: u64 = 9;
        let (lower, upper) = range(time, distance);
        assert_eq!(upper - lower + 1, 4);
    }

    #[test]
    fn test_even_time() {
        let time: u64 = 30;
        let distance: u64 = 200;
        let (lower, upper) = range(time, distance);
        assert_eq!(upper - lower + 1, 9);
    }

    #[test]
    fn test_prod_num_possibilities() {
        assert_eq!(
            prod_num_possibilities(
                "Time:      7  15   30
        Distance:  9  40  200"
            ),
            288
        );
    }

    #[test]
    fn test_num_possibilities() {
        assert_eq!(
            num_possibilities(
                "Time:      7  15   30
        Distance:  9  40  200"
            ),
            71503
        );
    }
}
