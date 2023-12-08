use core::str::Lines;

pub fn min_location_ex1(a: &Almanac) -> u64 {
    a.seeds_ex1()
        .iter()
        .map(|seed| a.location(*seed))
        .min()
        .unwrap()
}

pub fn min_location_ex2(a: &Almanac) -> u64 {
    a.location_intervals()[0].start()
}

pub struct Almanac {
    seeds_ex1: Vec<u64>,
    seed_intervals: Vec<Interval>,
    seed2soil: IntervalMapping,
    soil2fertilizer: IntervalMapping,
    fertilizer2water: IntervalMapping,
    water2light: IntervalMapping,
    light2temperature: IntervalMapping,
    temperature2humidity: IntervalMapping,
    humidity2location: IntervalMapping,
}

impl Almanac {
    pub fn parse(input: &str) -> Almanac {
        let mut a = Almanac {
            seeds_ex1: Vec::new(),
            seed_intervals: Vec::new(),
            seed2soil: IntervalMapping::new(),
            soil2fertilizer: IntervalMapping::new(),
            fertilizer2water: IntervalMapping::new(),
            water2light: IntervalMapping::new(),
            light2temperature: IntervalMapping::new(),
            temperature2humidity: IntervalMapping::new(),
            humidity2location: IntervalMapping::new(),
        };
        let mut lines = input.lines();

        let str_seeds = lines.next().unwrap();
        let start_seeds = str_seeds.find(|c: char| c.is_digit(10)).unwrap();
        str_seeds[start_seeds..]
            .split_whitespace()
            .for_each(|str_num| a.seeds_ex1.push(str_num.parse().unwrap()));

        for i in 0..a.seeds_ex1.len() / 2 {
            a.seed_intervals.push(Interval {
                start: a.seeds_ex1[2 * i],
                range: a.seeds_ex1[2 * i + 1],
            });
        }
        a.seed_intervals.sort_by(|x, y| x.start().cmp(&y.start()));

        lines.next();
        lines.next();
        parse_mapping(&mut a.seed2soil, &mut lines);

        lines.next();
        parse_mapping(&mut a.soil2fertilizer, &mut lines);

        lines.next();
        parse_mapping(&mut a.fertilizer2water, &mut lines);

        lines.next();
        parse_mapping(&mut a.water2light, &mut lines);

        lines.next();
        parse_mapping(&mut a.light2temperature, &mut lines);

        lines.next();
        parse_mapping(&mut a.temperature2humidity, &mut lines);

        lines.next();
        parse_mapping(&mut a.humidity2location, &mut lines);

        a
    }

    /// Returns a slice of seeds for exercise 5.1.
    pub fn seeds_ex1(&self) -> &[u64] {
        &self.seeds_ex1.as_slice()
    }

    pub fn seed_intervals(&self) -> &[Interval] {
        &self.seed_intervals.as_slice()
    }

    pub fn soil(&self, seed: u64) -> u64 {
        self.seed2soil.get(seed)
    }

    pub fn soil_intervals(&self) -> Vec<Interval> {
        self.seed2soil.intervals(self.seed_intervals.as_slice())
    }

    pub fn fertilizer(&self, seed: u64) -> u64 {
        self.soil2fertilizer.get(self.soil(seed))
    }

    pub fn fertilizer_intervals(&self) -> Vec<Interval> {
        self.soil2fertilizer
            .intervals(self.soil_intervals().as_slice())
    }

    pub fn water(&self, seed: u64) -> u64 {
        self.fertilizer2water.get(self.fertilizer(seed))
    }

    pub fn water_intervals(&self) -> Vec<Interval> {
        self.fertilizer2water
            .intervals(self.fertilizer_intervals().as_slice())
    }

    pub fn light(&self, seed: u64) -> u64 {
        self.water2light.get(self.water(seed))
    }

    pub fn light_intervals(&self) -> Vec<Interval> {
        self.water2light
            .intervals(self.water_intervals().as_slice())
    }

    pub fn temperature(&self, seed: u64) -> u64 {
        self.light2temperature.get(self.light(seed))
    }

    pub fn temperature_intervals(&self) -> Vec<Interval> {
        self.light2temperature
            .intervals(self.light_intervals().as_slice())
    }

    pub fn humidity(&self, seed: u64) -> u64 {
        self.temperature2humidity.get(self.temperature(seed))
    }

    pub fn humidity_intervals(&self) -> Vec<Interval> {
        self.temperature2humidity
            .intervals(&self.temperature_intervals().as_slice())
    }

    pub fn location(&self, seed: u64) -> u64 {
        self.humidity2location.get(self.humidity(seed))
    }

    pub fn location_intervals(&self) -> Vec<Interval> {
        self.humidity2location
            .intervals(&self.humidity_intervals().as_slice())
    }
}

fn parse_mapping(mapping: &mut IntervalMapping, lines: &mut Lines<'_>) {
    while let Some(str_mapping) = lines.next() {
        if str_mapping.is_empty() {
            break;
        }
        let numbers: Vec<u64> = str_mapping
            .split_whitespace()
            .map(|str_num| str_num.parse::<u64>().unwrap())
            .collect();
        mapping.push(numbers[1], numbers[0], numbers[2]);
    }
}

pub struct Interval {
    start: u64,
    range: u64,
}

impl Interval {
    pub fn new(start: u64, end: u64) -> Interval {
        if end < start {
            panic!("invalid");
        }
        Interval {
            start: start,
            range: end - start,
        }
    }
    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn end(&self) -> u64 {
        self.start + self.range
    }
}

struct SingleIntervalMapping {
    src: u64,
    dst: u64,
    range: u64,
}

impl SingleIntervalMapping {
    fn get(&self, key: u64) -> Option<u64> {
        if key >= self.src && key < self.src + self.range {
            return Some(key - self.src + self.dst);
        }
        None
    }

    fn src_start(&self) -> u64 {
        self.src
    }

    fn src_end(&self) -> u64 {
        self.src + self.range
    }

    fn dst_start(&self) -> u64 {
        self.dst
    }

    fn dst_end(&self) -> u64 {
        self.dst + self.range
    }
}

struct IntervalMapping {
    intervals: Vec<SingleIntervalMapping>,
}

impl IntervalMapping {
    fn new() -> IntervalMapping {
        IntervalMapping {
            intervals: Vec::new(),
        }
    }

    fn push(&mut self, src: u64, dst: u64, range: u64) {
        self.intervals
            .push(SingleIntervalMapping { src, dst, range });
        self.intervals.sort_by(|x, y| x.src.cmp(&y.src));
    }

    fn get(&self, key: u64) -> u64 {
        for mapping in self.intervals.iter() {
            if let Some(value) = mapping.get(key) {
                return value;
            }
        }
        key
    }

    fn intervals(&self, intervals: &[Interval]) -> Vec<Interval> {
        let mut v: Vec<Interval> = Vec::new();

        let mut interval_mapping_cursor: usize = 0;

        'intervals: for interval in intervals {
            let mut start = interval.start();
            let end = interval.end();

            loop {
                if self.intervals.len() <= interval_mapping_cursor {
                    break;
                }
                let interval_mapping = &self.intervals[interval_mapping_cursor];

                if start < interval_mapping.src_start() {
                    if end <= interval_mapping.src_start() {
                        v.push(Interval::new(start, end));
                        continue 'intervals;
                    } else if end <= interval_mapping.src_end() {
                        v.push(Interval::new(start, interval_mapping.src_start()));
                        v.push(Interval {
                            start: interval_mapping.dst_start(),
                            range: end - interval_mapping.src_start(),
                        });
                        continue 'intervals;
                    } else {
                        v.push(Interval::new(start, interval_mapping.src_start()));
                        v.push(Interval {
                            start: interval_mapping.dst_start(),
                            range: interval_mapping.range,
                        });
                        start = interval_mapping.src_end();
                    }
                } else if start < interval_mapping.src_end() {
                    if end <= interval_mapping.src_end() {
                        v.push(Interval::new(
                            start + interval_mapping.dst_start() - interval_mapping.src_start(),
                            end + interval_mapping.dst_start() - interval_mapping.src_start(),
                        ));
                        continue 'intervals;
                    } else {
                        v.push(Interval::new(
                            start + interval_mapping.dst_start() - interval_mapping.src_start(),
                            interval_mapping.dst_end(),
                        ));
                        start = interval_mapping.src_end();
                    }
                }
                interval_mapping_cursor += 1;
            }

            v.push(Interval::new(start, end));
        }

        v.sort_by(|x, y| x.start().cmp(&y.start()));
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_seeds() {
        let a = Almanac::parse(&INPUT);
        let seeds = a.seeds_ex1();
        assert_eq!(seeds.len(), 4);
        assert_eq!(seeds[0], 79);
        assert_eq!(seeds[1], 14);
        assert_eq!(seeds[2], 55);
        assert_eq!(seeds[3], 13);
    }

    #[test]
    fn test_seed_intervals() {
        let a = Almanac::parse(&INPUT);
        let intervals = a.seed_intervals();
        assert_eq!(intervals.len(), 2);

        assert_eq!(intervals[0].start(), 55);
        assert_eq!(intervals[0].end(), 68);

        assert_eq!(intervals[1].start(), 79);
        assert_eq!(intervals[1].end(), 93);
    }

    #[test]
    fn test_soil() {
        let a = Almanac::parse(&INPUT);
        assert_eq!(a.soil(98), 50);
        assert_eq!(a.soil(99), 51);
        assert_eq!(a.soil(50), 52);
        assert_eq!(a.soil(97), 99);
        assert_eq!(a.soil(0), 0);
        assert_eq!(a.soil(49), 49);
        assert_eq!(a.soil(100), 100);
    }

    #[test]
    fn test_soil_intervals() {
        let a = Almanac::parse(&INPUT);
        let intervals = a.soil_intervals();

        assert_eq!(intervals.len(), 2);

        assert_eq!(intervals[0].start(), 57);
        assert_eq!(intervals[0].end(), 70);

        assert_eq!(intervals[1].start(), 81);
        assert_eq!(intervals[1].end(), 95);
    }

    #[test]
    fn test_fertilizer() {
        let a = Almanac::parse(&INPUT);
        assert_eq!(a.fertilizer(79), 81);
        assert_eq!(a.fertilizer(14), 53);
        assert_eq!(a.fertilizer(55), 57);
        assert_eq!(a.fertilizer(13), 52);
    }

    #[test]
    fn test_fertilizer_intervals() {
        let a = Almanac::parse(&INPUT);
        let intervals = a.fertilizer_intervals();

        assert_eq!(intervals.len(), 2);

        assert_eq!(intervals[0].start(), 57);
        assert_eq!(intervals[0].end(), 70);

        assert_eq!(intervals[1].start(), 81);
        assert_eq!(intervals[1].end(), 95);
    }

    #[test]
    fn test_water() {
        let a = Almanac::parse(&INPUT);
        assert_eq!(a.water(79), 81);
        assert_eq!(a.water(14), 49);
        assert_eq!(a.water(55), 53);
        assert_eq!(a.water(13), 41);
    }

    #[test]
    fn test_water_intervals() {
        let a = Almanac::parse(&INPUT);
        let intervals = a.water_intervals();

        assert_eq!(intervals.len(), 3);

        assert_eq!(intervals[0].start(), 53);
        assert_eq!(intervals[0].end(), 57);

        assert_eq!(intervals[1].start(), 61);
        assert_eq!(intervals[1].end(), 70);

        assert_eq!(intervals[2].start(), 81);
        assert_eq!(intervals[2].end(), 95);
    }

    #[test]
    fn test_light() {
        let a = Almanac::parse(&INPUT);
        assert_eq!(a.light(79), 74);
        assert_eq!(a.light(14), 42);
        assert_eq!(a.light(55), 46);
        assert_eq!(a.light(13), 34);
    }

    #[test]
    fn test_temperature() {
        let a = Almanac::parse(&INPUT);
        assert_eq!(a.temperature(79), 78);
        assert_eq!(a.temperature(14), 42);
        assert_eq!(a.temperature(55), 82);
        assert_eq!(a.temperature(13), 34);
    }

    #[test]
    fn test_humidity() {
        let a = Almanac::parse(&INPUT);
        assert_eq!(a.humidity(79), 78);
        assert_eq!(a.humidity(14), 43);
        assert_eq!(a.humidity(55), 82);
        assert_eq!(a.humidity(13), 35);
    }

    #[test]
    fn test_location() {
        let a = Almanac::parse(&INPUT);
        assert_eq!(a.location(79), 82);
        assert_eq!(a.location(14), 43);
        assert_eq!(a.location(55), 86);
        assert_eq!(a.location(13), 35);
    }

    #[test]
    fn test_min_location() {
        let a = Almanac::parse(&INPUT);
        assert_eq!(min_location_ex1(&a), 35);
        assert_eq!(min_location_ex2(&a), 46);
    }
}
