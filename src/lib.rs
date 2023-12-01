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
    let mut remaining_line=line;
    while let Some(c)=chars.next() {
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
        
        remaining_line=&remaining_line[c.len_utf8()..];
    }
    (None, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_digit() {
        let (mut next, mut remaining) = next_digit("eightwothree");
        assert_eq!(next.unwrap(), 8);
        assert_eq!(remaining, "ightwothree");
        (next, remaining) = next_digit(remaining);
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
        assert_eq!(sum_codes("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), 142);
        assert_eq!(sum_codes("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen"), 281);
    }
}
