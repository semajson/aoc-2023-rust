use crate::Solution;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Day10;

impl Solution for Day10 {
    type ParsedInput = HashMap<Coord, Node>;

    fn parse_input(input: &str) -> Self::ParsedInput {
        // No regex today...
        let input_lines = input.lines().collect::<Vec<&str>>();

        let grid = input_lines
            .iter()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut map = HashMap::new();
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let shape = grid[y][x];
                let coord = Coord::from_usize(x, y);
                map.insert(coord.clone(), Node { coord, shape });
            }
        }

        map
    }

    fn part_one(map: &mut Self::ParsedInput) -> String {
        let map = map.clone(); //defensive, might not need

        // Get the starting node
        let starting_node = map
            .values()
            .filter(|&node| node.shape == 'S')
            .collect::<Vec<&Node>>();

        assert!(starting_node.len() == 1);
        let starting_node = starting_node[0].clone();

        // Now traverse the map
        let mut visited = HashMap::new();
        let mut steps = 0;
        visited.insert(starting_node.clone(), steps);
        let mut current_nodes = vec![starting_node];

        while !current_nodes.is_empty() {
            steps += 1;

            let next_coords = current_nodes
                .iter()
                .map(|node| node.get_connections(&map))
                .flatten()
                .collect::<Vec<Coord>>();

            let possible_next_nodes = next_coords
                .iter()
                .map(|coord| map.get(&coord).unwrap().clone())
                .collect::<Vec<Node>>();

            let mut next_nodes = vec![];
            for node in possible_next_nodes {
                if !visited.contains_key(&node) {
                    // Unseen node, lets visit it:
                    next_nodes.push(node)
                } else if *visited.get(&node).unwrap() > steps {
                    // We have found a shorter way to get to this node, so keep exploring
                    next_nodes.push(node)
                }
            }

            // Update visited
            current_nodes = next_nodes;
            for node in current_nodes.iter() {
                visited.insert(node.clone(), steps);
            }
        }

        let max = visited.values().max().unwrap();

        println!("test");
        max.to_string()
    }

    fn part_two(map: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Coord {
    x: isize,
    y: isize,
}
impl Coord {
    pub fn from_usize(x: usize, y: usize) -> Coord {
        Coord {
            x: x as isize,
            y: y as isize,
        }
    }
    pub fn from(x: isize, y: isize) -> Coord {
        Coord { x: x, y: y }
    }

    pub fn get_neighbors(&self) -> Vec<Coord> {
        let mut neighbors = vec![];
        for x in -1..=1 {
            for y in -1..=1 {
                if !(x == 0 && y == 0) {
                    neighbors.push(Coord {
                        x: self.x + x,
                        y: self.y + y,
                    })
                }
            }
        }

        neighbors
    }

    pub fn get_neighbors_set(&self) -> HashSet<Coord> {
        HashSet::from_iter(self.get_neighbors())
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Node {
    coord: Coord,
    shape: char,
}
impl Node {
    pub fn get_connections(&self, map: &HashMap<Coord, Node>) -> Vec<Coord> {
        // Note:
        // x goes from left to right (increasing +ve)
        //y goes from top to bottom (increasing + ve)
        match self.shape {
            '|' => vec![
                Coord::from(self.coord.x, self.coord.y + 1),
                Coord::from(self.coord.x, self.coord.y - 1),
            ],
            '-' => vec![
                Coord::from(self.coord.x + 1, self.coord.y),
                Coord::from(self.coord.x - 1, self.coord.y),
            ],
            'L' => vec![
                Coord::from(self.coord.x, self.coord.y - 1),
                Coord::from(self.coord.x + 1, self.coord.y),
            ],
            'J' => vec![
                Coord::from(self.coord.x, self.coord.y - 1),
                Coord::from(self.coord.x - 1, self.coord.y),
            ],
            '7' => vec![
                Coord::from(self.coord.x, self.coord.y + 1),
                Coord::from(self.coord.x - 1, self.coord.y),
            ],
            'F' => vec![
                Coord::from(self.coord.x, self.coord.y + 1),
                Coord::from(self.coord.x + 1, self.coord.y),
            ],
            'S' => self.get_s_connections(map),
            '.' => vec![], // Don't panic here to allow for the S logic
            _ => panic!("Error, didn't get connected coords for {:?}", self),
        }
    }

    fn get_s_connections(&self, map: &HashMap<Coord, Node>) -> Vec<Coord> {
        // Special case for S
        assert!(self.shape == 'S');

        let neighbors = self.coord.get_neighbors();

        let mut connected_coords = vec![];

        for neighbor in neighbors {
            let neighbor_connections = map.get(&neighbor).unwrap().get_connections(map);

            if neighbor_connections.contains(&self.coord) {
                connected_coords.push(neighbor);
            }
        }

        connected_coords
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day10_part1_case1() {
        assert_eq!(
            Day10::solve_part_one(
                "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            "4".to_string()
        )
    }

    #[test]
    fn check_day10_part2_case1() {
        assert_eq!(Day10::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day10_both_case1() {
        assert_eq!(Day10::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
