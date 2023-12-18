pub fn sum_hash(input: &str) -> u64 {
    input.split(|c| c == ',').map(|s| hash(s) as u64).sum()
}

pub fn focusing_power(input: &str) -> u64 {
    let mut boxes: Vec<LensBox> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(LensBox::new());
    }

    input.split(|c| c == ',').for_each(|s| {
        if let Some(index) = s.find('=') {
            let label = String::from(&s[..index]);
            let focal: u64 = s[index + 1..].parse().unwrap();
            boxes[hash(&s[..index]) as usize].add(label, focal);
            return;
        }
        if let Some(index) = s.find('-') {
            boxes[hash(&s[..index]) as usize].remove(&s[..index]);
            return;
        }
    });

    let mut pwr = 0u64;
    for i in 0..256 {
        pwr += (i as u64 + 1) * boxes[i].focusing_power();
    }
    pwr
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

struct LensBox {
    lenses: Vec<(String, u64)>,
}

impl LensBox {
    fn new() -> LensBox {
        LensBox { lenses: Vec::new() }
    }

    fn add(&mut self, label: String, focal: u64) {
        if let Some(item) = self.lenses.iter_mut().find(|lens| &lens.0 == &label) {
            item.1 = focal;
            return;
        }
        self.lenses.push((label, focal));
    }

    fn remove(&mut self, s: &str) {
        let mut shift = false;
        for i in 0..self.lenses.len() {
            if shift {
                self.lenses[i - 1].0 = self.lenses[i].0.clone();
                self.lenses[i - 1].1 = self.lenses[i].1;
            }
            if self.lenses[i].0 == s {
                shift = true;
            }
        }
        if shift {
            self.lenses.pop();
        }
    }

    fn focusing_power(&self) -> u64 {
        let mut pwr = 0u64;
        for i in 0..self.lenses.len() {
            pwr += (i as u64 + 1) * self.lenses[i].1;
        }
        pwr
    }
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
        assert_eq!(hash("rn"), 0);
    }

    #[test]
    fn test_sum_hash() {
        assert_eq!(
            sum_hash("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }

    #[test]
    fn test_empty_box() {
        let b = LensBox::new();

        assert_eq!(b.focusing_power(), 0);
    }

    #[test]
    fn test_add_new_lens() {
        let mut b = LensBox::new();
        b.add("rn".to_string(), 1);

        assert_eq!(b.focusing_power(), 1);
    }

    #[test]
    fn test_replace_lens() {
        let mut b = LensBox::new();
        b.add("rn".to_string(), 1);
        b.add("rn".to_string(), 3);

        assert_eq!(b.focusing_power(), 3);
    }

    #[test]
    fn test_add_two_lenses() {
        let mut b = LensBox::new();
        b.add("rn".to_string(), 1);
        b.add("cm".to_string(), 2);

        assert_eq!(b.focusing_power(), 5);
    }

    #[test]
    fn test_remove_existing_lens() {
        let mut b = LensBox::new();
        b.add("rn".to_string(), 1);
        b.remove("rn");

        assert_eq!(b.focusing_power(), 0);
    }

    #[test]
    fn test_remove_non_existing_lens() {
        let mut b = LensBox::new();
        b.add("rn".to_string(), 1);
        b.remove("cm");

        assert_eq!(b.focusing_power(), 1);
    }

    #[test]
    fn test_focusing_power() {
        assert_eq!(
            focusing_power("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145,
        )
    }
}
