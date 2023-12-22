use crate::Solution;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Day17;

impl Solution for Day17 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part one
        let mut priority_queue = vec![(
            Node {
                x: 0,
                y: 0,
                up_count: 0,
                down_count: 0,
                left_count: 0,
                right_count: 0,
            },
            Info {
                cost_to_node: 0,
                prev_node: None,
            },
        )];
        let mut visited_nodes: HashSet<Node> = HashSet::new();

        while !priority_queue.is_empty() {
            // Find and remove the smallest node
            let (node_to_eval, info) = priority_queue
                .iter()
                .min_by_key(|(node, info)| info.cost_to_node)
                .unwrap()
                .clone();
            priority_queue.retain(|(node, info)| *node != node_to_eval);
            visited_nodes.insert(node_to_eval);

            // Add node to visited nodes

            // Check if we are at the end

            // Get the reachable nodes

            // For nodes in reachable nodes:
            // Filter out ones not in the map
            // Check if in priority queue
            // If yes, update if quicker
            // If not, add it to the priorty queue
        }

        0.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

// Todo, maybe replace direction count with an "entered direction" + "count" variables, to make code simpler.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    x: isize,
    y: isize,
    up_count: usize,
    down_count: usize,
    left_count: usize,
    right_count: usize,
}
impl Node {
    fn get_reachable_nodes(&self) -> Vec<Node> {
        let mut reachable_nodes = vec![];

        // Up
        if (self.up_count < 3) && self.down_count == 0 {
            reachable_nodes.push(Node {
                x: self.x,
                y: self.y - 1,
                up_count: self.up_count + 1,
                down_count: 0,
                left_count: 0,
                right_count: 0,
            })
        }

        // Down
        if self.down_count < 3 && self.up_count == 0 {
            reachable_nodes.push(Node {
                x: self.x,
                y: self.y + 1,
                up_count: 0,
                down_count: self.down_count + 1,
                left_count: 0,
                right_count: 0,
            })
        }

        // Left
        if self.left_count < 3 && self.right_count == 0 {
            reachable_nodes.push(Node {
                x: self.x - 1,
                y: self.y,
                up_count: 0,
                down_count: 0,
                left_count: self.left_count + 1,
                right_count: 0,
            })
        }

        // Right
        if self.right_count < 3 && self.left_count == 0 {
            reachable_nodes.push(Node {
                x: self.x + 1,
                y: self.y,
                up_count: 0,
                down_count: 0,
                left_count: 0,
                right_count: self.right_count + 1,
            })
        }

        reachable_nodes
    }
}

#[derive(Debug, Clone)]
pub struct Info {
    cost_to_node: i64,
    prev_node: Option<Node>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day17_part1_case1() {
        assert_eq!(
            Day17::solve_part_one(
                "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            ),
            "102".to_string()
        )
    }

    #[test]
    fn check_day17_part2_case1() {
        assert_eq!(
            Day17::solve_part_two(
                "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            ),
            "0".to_string()
        )
    }

    #[test]
    fn check_day17_both_case1() {
        assert_eq!(
            Day17::solve(
                "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
                false
            ),
            ("0".to_string(), "0".to_string())
        )
    }
}
