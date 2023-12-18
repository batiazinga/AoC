pub fn sum_hash(input: &str) -> u64 {
    input.split(|c| c == ',').map(|s| hash(s) as u64).sum()
}

fn hash(input: &str) -> u8 {
    let mut h = 0u16;
    for b in input.bytes() {
        h += b as u16;
        h *= 17;
        h %= 256;
    }

    h as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn test_sum_hash() {
        assert_eq!(
            sum_hash("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }
}
