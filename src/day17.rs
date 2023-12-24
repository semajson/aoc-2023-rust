use crate::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Day17;

impl Solution for Day17 {
    type ParsedInput = (HashMap<(isize, isize), i64>, (isize, isize));

    fn parse_input(input: &str) -> Self::ParsedInput {
        let input_lines = input.lines().collect::<Vec<&str>>();

        let grid = input_lines
            .iter()
            .map(|x| {
                x.chars()
                    .map(|x| x.to_digit(10).unwrap() as i64)
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>();

        let mut cost_map = HashMap::new();
        let y_len = grid.len();
        let x_len = grid[0].len();
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                cost_map.insert((x as isize, y as isize), grid[y][x] as i64);
            }
        }
        let end = ((x_len - 1) as isize, (y_len - 1) as isize);

        (cost_map, end)
    }

    fn part_one((cost_map, end_pos): &mut Self::ParsedInput) -> String {
        let start_pos = (0, 0);

        let min_cost = dijkstra_solve(start_pos, end_pos.clone(), &cost_map, false);

        min_cost.to_string()
    }

    fn part_two((cost_map, end_pos): &mut Self::ParsedInput) -> String {
        let start_pos = (0, 0);

        let min_cost = dijkstra_solve(start_pos, end_pos.clone(), &cost_map, true);

        min_cost.to_string()
    }
}

fn dijkstra_solve(
    start_pos: (isize, isize),
    end_pos: (isize, isize),
    cost_map: &HashMap<(isize, isize), i64>,
    part_2: bool,
) -> i64 {
    // Intialization
    let mut priority_queue = vec![(
        Node {
            x: start_pos.0,
            y: start_pos.1,
            up_count: 0,
            down_count: 0,
            left_count: 0,
            right_count: 0,
        },
        Info {
            total_cost_to_node: 0,
            prev_node: None,
        },
    )];
    let mut visited_nodes: HashSet<Node> = HashSet::new();

    while !priority_queue.is_empty() {
        // println!("{:?}", priority_queue);
        // Find and remove the smallest node
        let (node_to_eval, info) = priority_queue
            .iter()
            .min_by_key(|(_, info)| info.total_cost_to_node)
            .unwrap()
            .clone();
        priority_queue.retain(|(node, _)| *node != node_to_eval);
        visited_nodes.insert(node_to_eval.clone());

        // Check if we are at the end
        if ((node_to_eval.x, node_to_eval.y) == end_pos) && node_to_eval.can_stop(part_2) {
            return info.total_cost_to_node;
        }

        let reachable_nodes = node_to_eval.get_reachable_nodes(part_2);

        let reachable_nodes = reachable_nodes
            .into_iter()
            .filter(|node| !visited_nodes.contains(node))
            .filter(|node| cost_map.contains_key(&(node.x, node.y)))
            .collect::<Vec<Node>>();

        for reachable_node in reachable_nodes.into_iter() {
            let node_cost = cost_map.get(&(reachable_node.x, reachable_node.y)).unwrap();
            let total_cost_to_node = info.total_cost_to_node + node_cost;

            if let Some((existing_node, existing_info)) = priority_queue
                .iter_mut()
                .find(|(node, _)| *node == reachable_node)
            {
                if existing_info.total_cost_to_node > total_cost_to_node {
                    // Found a quicker way to a node, update it.
                    existing_info.total_cost_to_node = total_cost_to_node;
                    existing_info.prev_node = Some(node_to_eval.clone());
                }
            } else {
                // Not seen this node yet - add it to priority queue.
                priority_queue.push((
                    reachable_node.clone(),
                    Info {
                        total_cost_to_node,
                        prev_node: Some(node_to_eval.clone()),
                    },
                ));
            }
        }
    }
    panic!("Didn't get solve the maze without getting to the end!");
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
    fn get_reachable_nodes(&self, part_2: bool) -> Vec<Node> {
        if part_2 {
            self.get_reachable_nodes_part_2()
        } else {
            self.get_reachable_nodes_part_1()
        }
    }

    fn get_reachable_nodes_part_1(&self) -> Vec<Node> {
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

    fn get_reachable_nodes_part_2(&self) -> Vec<Node> {
        vec![]
    }
    fn can_stop(&self, part_2: bool) -> bool {
        !part_2 || ((self.up_count + self.down_count + self.right_count + self.left_count) > 3)
    }
}

#[derive(Debug, Clone)]
pub struct Info {
    total_cost_to_node: i64,
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
            "94".to_string()
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
