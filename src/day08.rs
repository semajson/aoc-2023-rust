use crate::Solution;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Branch {
    left: String,
    right: String,
}
impl Branch {
    fn get_node(&self, instruction: &char) -> String {
        match instruction {
            'L' => self.left.to_string(),
            'R' => self.right.to_string(),
            _ => panic!("Error {:?}", self),
        }
    }
}

pub struct Map(HashMap<String, Branch>);

#[derive(Clone, Debug)]
pub struct Day08;

impl Solution for Day08 {
    type ParsedInput = (Vec<char>, Map);

    fn parse_input(input: &str) -> Self::ParsedInput {
        let re = Regex::new(r"(?<instructions>^[RL]+)").unwrap();
        let Some(cap) = re.captures(input) else {
            panic!("No match")
        };
        let instructions = cap
            .name("instructions")
            .unwrap()
            .as_str()
            .chars()
            .collect::<Vec<char>>();

        let re =
            Regex::new(r"(?<key>[A-Z]{3}) = \((?<left>[A-Z]{3}), (?<right>[A-Z]{3})\)").unwrap();
        let mut map = HashMap::new();
        for cap in re.captures_iter(input) {
            let key = cap.name("key").unwrap().as_str().to_string();
            let left = cap.name("left").unwrap().as_str().to_string();
            let right = cap.name("right").unwrap().as_str().to_string();

            map.insert(key, Branch { left, right });
        }

        (instructions, Map(map))
    }

    fn part_one((instructions, map): &mut Self::ParsedInput) -> String {
        let mut node = "AAA".to_string();

        let mut step = 0;

        while node != "ZZZ" {
            let index = step % instructions.len();
            let instruction = instructions[index];

            let branch = map.0.get(&node).unwrap();
            node = branch.get_node(&instruction);

            step += 1;
        }

        step.to_string()
    }

    fn part_two((instructions, map): &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day08_part1_case1() {
        assert_eq!(
            Day08::solve_part_one(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            "2".to_string()
        )
    }

    #[test]
    fn check_day08_part2_case1() {
        assert_eq!(
            Day08::solve_part_two(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            "0".to_string()
        )
    }

    #[test]
    fn check_day08_both_case1() {
        assert_eq!(
            Day08::solve(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
                false
            ),
            ("0".to_string(), "0".to_string())
        )
    }
}
