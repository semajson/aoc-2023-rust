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
            .filter(|x| x.chars().last().unwrap() == 'A')
            .collect::<Vec<String>>();

        let step: i32 = 0;

        let mut all_visited = vec![];

        for starting_node in starting_nodes {
            let mut node = starting_node.clone();

            let mut visited = vec![];

            let mut step = 0;
            let mut index = step % instructions.len();
            while !visited.contains(&(node.clone(), index)) {
                visited.push((node.clone(), index.clone()));

                let instruction = instructions[index];
                let branch = map.0.get(&node).unwrap();
                node = branch.get_node(&instruction);

                step += 1;
                index = step % instructions.len();
            }
            visited.push((node.clone(), index.clone()));

            all_visited.push(visited);
        }

        let mut all_z_indexes = vec![];

        for visited in all_visited {
            let mut z_indexes = vec![];
            for (index, node) in visited.iter().enumerate() {
                if (node.0.chars().last().unwrap() == 'Z') {
                    z_indexes.push(index);
                }
            }
            println!("Found {:?}", z_indexes);
            all_z_indexes.push(z_indexes);
        }

        // This is an assumption based on the seen input...
        let all_z_indexes = all_z_indexes
            .into_iter()
            .map(|x| x[0])
            .collect::<Vec<usize>>();

        let fewest_steps = all_z_indexes
            .iter()
            .fold(1, |fewest_steps, x| Integer::lcm(&fewest_steps, x));

        fewest_steps.to_string()
    }
}

// while nodes.iter().any(|x| x.chars().last().unwrap() != 'Z') {
//     let index = step % instructions.len();
//     let instruction = instructions[index];

//     let mut next_nodes = vec![];

//     for node in nodes.iter() {
//         let branch = map.0.get(node).unwrap();
//         next_nodes.push(branch.get_node(&instruction));
//     }
//     nodes = next_nodes;

//     step += 1;
// }

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
