use crate::Solution;
use num::Integer;
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

#[derive(Clone, Debug)]
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
            Regex::new(r"(?<key>[A-Z0-9]{3}) = \((?<left>[A-Z0-9]{3}), (?<right>[A-Z0-9]{3})\)")
                .unwrap();
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
        let starting_nodes = map
            .clone()
            .0
            .into_keys()
            .filter(|x| x.ends_with('A'))
            .collect::<Vec<String>>();

        let all_visited = starting_nodes
            .iter()
            .map(|x| get_all_visited(x, instructions, map))
            .collect::<Vec<Vec<(String, usize)>>>();

        let mut all_z_indexes = vec![];

        for visited in all_visited {
            let mut z_indexes = vec![];
            for (index, node) in visited.iter().enumerate() {
                if node.0.ends_with('Z') {
                    z_indexes.push(index);
                }
            }

            // This assumes there is only 1 Z node visited by each starting node.
            // This is a big assumption, but was true in the input...
            // Note, if we didn't assume this:
            // - all_z_indexes would be a Vec<Vec<usize>>.
            // - then need to find all combinations of the lcm to get the lowest one.
            // This would add trivial compute time, but not trivial developer time...
            assert!(z_indexes.len() == 1);
            all_z_indexes.push(z_indexes[0]);
        }

        let fewest_steps = all_z_indexes
            .iter()
            .fold(1, |fewest_steps, x| Integer::lcm(&fewest_steps, x));

        fewest_steps.to_string()
    }
}

fn get_all_visited(
    starting_node: &str,
    instructions: &Vec<char>,
    map: &Map,
) -> Vec<(String, usize)> {
    let mut node = starting_node.to_owned();

    let mut visited = vec![];

    let mut step = 0;
    let mut index = step % instructions.len();

    while !visited.contains(&(node.clone(), index)) {
        visited.push((node.clone(), index));

        let instruction = instructions[index];
        let branch = map.0.get(&node).unwrap();
        node = branch.get_node(&instruction);

        step += 1;
        index = step % instructions.len();
    }
    visited.push((node.clone(), index));
    visited
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
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            "6".to_string()
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
