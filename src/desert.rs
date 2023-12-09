use num::integer::lcm;
use std::collections::HashMap;

pub fn read_map(
    input: &str,
) -> (
    Vec<Direction>,
    HashMap<String, (String, String)>,
    Vec<String>,
) {
    let mut instructions: Vec<Direction> = Vec::new();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut a_nodes: Vec<String> = Vec::new();

    let mut lines = input.lines();
    let str_instructions = lines.next().unwrap();
    instructions.extend(
        str_instructions
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| match c {
                'L' => Direction::L,
                _ => Direction::R,
            }),
    );

    lines.next();
    for line in lines {
        let equ_index = line.find(|c: char| c == '=').unwrap();
        let node = line[..equ_index].trim().to_string();
        if node.chars().last().unwrap() == 'A' {
            a_nodes.push(node.clone());
        }

        let left_par_index = line.find(|c: char| c == '(').unwrap();
        let right_par_index = line.find(|c: char| c == ')').unwrap();
        let comma_index = line.find(|c: char| c == ',').unwrap();
        let left = line[left_par_index + 1..comma_index].to_string();
        let right = line[comma_index + 1..right_par_index].trim().to_string();

        map.insert(node, (left, right));
    }

    (instructions, map, a_nodes)
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    L,
    R,
}

pub fn count_steps_from_aaa_to_zzz(
    instructions: &[Direction],
    map: &HashMap<String, (String, String)>,
) -> u64 {
    count_steps_to_z("AAA", instructions, map)
}

pub fn count_steps_from_a_to_z(
    instructions: &[Direction],
    map: &HashMap<String, (String, String)>,
    a_nodes: &[String],
) -> u64 {
    let mut counts: Vec<u64> = Vec::with_capacity(a_nodes.len());

    for node in a_nodes {
        counts.push(count_steps_to_z(node, instructions, map));
    }

    let mut count = counts[0];
    for i in 1..counts.len() {
        count = lcm(count, counts[i]);
    }
    count
}

pub fn count_steps_from_a_to_z2(
    instructions: &[Direction],
    map: &HashMap<String, (String, String)>,
    a_nodes: &[String],
) -> u64 {
    let mut step_counter: u64 = 0;

    let mut instruction_cursor: usize = 0;
    let mut current_nodes: Vec<&String> = Vec::from_iter(a_nodes.iter());

    while !current_nodes.iter().all(|s| s.ends_with('Z')) {
        for i in 0..current_nodes.len() {
            let node = current_nodes[i];
            current_nodes[i] = match &instructions[instruction_cursor] {
                Direction::L => &map.get(node).unwrap().0,
                Direction::R => &map.get(node).unwrap().1,
            };
        }

        if instruction_cursor == instructions.len() - 1 {
            instruction_cursor = 0;
        } else {
            instruction_cursor += 1;
        }
        step_counter += 1;
    }

    step_counter
}

fn count_steps_to_z(
    from: &str,
    instructions: &[Direction],
    map: &HashMap<String, (String, String)>,
) -> u64 {
    let mut step_counter: u64 = 0;

    let mut instruction_cursor: usize = 0;
    let mut current_node = from;
    while !current_node.ends_with('Z') {
        let next_node = match &instructions[instruction_cursor] {
            Direction::L => map.get(current_node).unwrap().0.as_str(),
            Direction::R => map.get(current_node).unwrap().1.as_str(),
        };

        if instruction_cursor == instructions.len() - 1 {
            instruction_cursor = 0;
        } else {
            instruction_cursor += 1;
        }
        current_node = next_node;
        step_counter += 1;
    }

    step_counter
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_steps_1() {
        let instructions: Vec<Direction> = Vec::from([Direction::R, Direction::L]);
        let map: HashMap<String, (String, String)> = HashMap::from([
            ("AAA".to_string(), ("BBB".to_string(), "CCC".to_string())),
            ("BBB".to_string(), ("DDD".to_string(), "EEE".to_string())),
            ("CCC".to_string(), ("ZZZ".to_string(), "GGG".to_string())),
            ("DDD".to_string(), ("DDD".to_string(), "DDD".to_string())),
            ("EEE".to_string(), ("EEE".to_string(), "EEE".to_string())),
            ("GGG".to_string(), ("GGG".to_string(), "GGG".to_string())),
            ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
        ]);

        assert_eq!(
            count_steps_from_aaa_to_zzz(instructions.as_slice(), &map),
            2
        );
    }

    #[test]
    fn test_count_steps_2() {
        let instructions: Vec<Direction> = Vec::from([Direction::L, Direction::L, Direction::R]);
        let map: HashMap<String, (String, String)> = HashMap::from([
            ("AAA".to_string(), ("BBB".to_string(), "BBB".to_string())),
            ("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string())),
            ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
        ]);

        assert_eq!(
            count_steps_from_aaa_to_zzz(instructions.as_slice(), &map),
            6
        );
    }

    #[test]
    fn test_read_map_1() {
        let (instructions, map, a_nodes) = read_map(
            "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)",
        );

        assert_eq!(instructions.len(), 2);
        assert_eq!(instructions[0], Direction::R);
        assert_eq!(instructions[1], Direction::L);

        assert_eq!(map.len(), 7);
        assert_eq!(
            map.get(&"AAA".to_string()),
            Some(&("BBB".to_string(), "CCC".to_string()))
        );
        assert_eq!(
            map.get(&"BBB".to_string()),
            Some(&("DDD".to_string(), "EEE".to_string()))
        );
        assert_eq!(
            map.get(&"CCC".to_string()),
            Some(&("ZZZ".to_string(), "GGG".to_string()))
        );
        assert_eq!(
            map.get(&"DDD".to_string()),
            Some(&("DDD".to_string(), "DDD".to_string()))
        );
        assert_eq!(
            map.get(&"EEE".to_string()),
            Some(&("EEE".to_string(), "EEE".to_string()))
        );
        assert_eq!(
            map.get(&"GGG".to_string()),
            Some(&("GGG".to_string(), "GGG".to_string()))
        );
        assert_eq!(
            map.get(&"ZZZ".to_string()),
            Some(&("ZZZ".to_string(), "ZZZ".to_string()))
        );

        assert_eq!(a_nodes.len(), 1);
        assert_eq!(a_nodes[0], "AAA".to_string());
    }

    #[test]
    fn test_read_map_2() {
        let (instructions, map, a_nodes) = read_map(
            "LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)",
        );

        assert_eq!(instructions.len(), 3);
        assert_eq!(instructions[0], Direction::L);
        assert_eq!(instructions[1], Direction::L);
        assert_eq!(instructions[2], Direction::R);

        assert_eq!(map.len(), 3);
        assert_eq!(
            map.get(&"AAA".to_string()),
            Some(&("BBB".to_string(), "BBB".to_string()))
        );
        assert_eq!(
            map.get(&"BBB".to_string()),
            Some(&("AAA".to_string(), "ZZZ".to_string()))
        );
        assert_eq!(
            map.get(&"ZZZ".to_string()),
            Some(&("ZZZ".to_string(), "ZZZ".to_string()))
        );

        assert_eq!(a_nodes.len(), 1);
        assert_eq!(a_nodes[0], "AAA".to_string());
    }

    #[test]
    fn test_count_steps_from_a_to_z() {
        let (instructions, map, a_nodes) = read_map(
            "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)",
        );
        assert_eq!(
            count_steps_from_a_to_z(instructions.as_slice(), &map, a_nodes.as_slice()),
            6
        );
    }
}
