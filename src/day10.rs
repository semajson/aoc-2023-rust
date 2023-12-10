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

    fn part_one(input_map: &mut Self::ParsedInput) -> String {
        let map = input_map.clone(); //defensive, might not need

        let starting_node = get_starting_node(&map);

        let visited = traverse_breadth_first(starting_node, &map);

        let max = visited.values().max().unwrap();

        max.to_string()
    }

    fn part_two(input_map: &mut Self::ParsedInput) -> String {
        let map = input_map.clone(); //defensive, might not need

        let starting_node = get_starting_node(&map);

        let found_loop = traverse_breadth_first(starting_node, &map)
            .into_keys()
            .collect::<HashSet<Node>>();

        let tiles_in_loop = map
            .values()
            .clone()
            .filter(|&node| !found_loop.contains(node))
            .filter(|&node| node_in_loop(node, &found_loop, &map))
            .collect::<Vec<&Node>>();

        tiles_in_loop.len().to_string()
    }
}

fn traverse_breadth_first(starting_node: Node, map: &HashMap<Coord, Node>) -> HashMap<Node, i32> {
    let mut visited = HashMap::new();
    visited.insert(starting_node.clone(), 0);

    let mut current_nodes = vec![starting_node];

    let mut steps = 0;

    while !current_nodes.is_empty() {
        steps += 1;

        let possible_next_nodes = get_possible_next_nodes(&current_nodes, map);

        let mut next_nodes = vec![];
        for node in possible_next_nodes {
            if !visited.contains_key(&node) || *visited.get(&node).unwrap() > steps {
                // Unseen node or:
                // We have found a shorter way to get to this node, so keep exploring
                next_nodes.push(node)
            }
        }

        // Update visited
        current_nodes = next_nodes;
        for node in current_nodes.iter() {
            visited.insert(node.clone(), steps);

            // println!("tet");
        }
    }
    visited
}

fn get_starting_node(map: &HashMap<Coord, Node>) -> Node {
    let starting_node = map
        .values()
        .filter(|&node| node.shape == 'S')
        .collect::<Vec<&Node>>();

    assert!(starting_node.len() == 1);
    starting_node[0].clone()
}

fn get_possible_next_nodes(current_nodes: &[Node], map: &HashMap<Coord, Node>) -> Vec<Node> {
    let next_coords = current_nodes
        .iter()
        .flat_map(|node| node.get_connections(map))
        // .flatten()
        .collect::<Vec<Coord>>();

    let possible_next_nodes = next_coords
        .iter()
        .map(|coord| map.get(coord).unwrap().clone())
        .collect::<Vec<Node>>();
    possible_next_nodes
}

fn node_in_loop(node: &Node, found_loop: &HashSet<Node>, map: &HashMap<Coord, Node>) -> bool {
    // Cheated and looked this up
    assert!(!found_loop.contains(node));

    let mut current_coord = node.coord.clone();
    let mut crosses = 0;

    while map.contains_key(&current_coord) {
        let current_node = map.get(&current_coord).unwrap();

        if found_loop.contains(current_node)
            && ((current_node.get_effective_shape(map) == '|')
                || current_node.get_effective_shape(map) == 'J'
                || current_node.get_effective_shape(map) == 'L')
        {
            crosses += 1;
        }

        current_coord.x += 1;
    }
    (crosses % 2) == 1
}

#[derive(Clone, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
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
        Coord { x, y }
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
    pub fn get_adjacent_neighbors(&self) -> Vec<Coord> {
        vec![
            Coord {
                x: self.x + 1,
                y: self.y,
            },
            Coord {
                x: self.x - 1,
                y: self.y,
            },
            Coord {
                x: self.x,
                y: self.y + 1,
            },
            Coord {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct Node {
    coord: Coord,
    shape: char,
}
impl Node {
    pub fn get_connections(&self, map: &HashMap<Coord, Node>) -> Vec<Coord> {
        // Note:
        // x goes from left to right (increasing +ve)
        //y goes from top to bottom (increasing + ve)
        match self.get_effective_shape(map) {
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
            '.' => vec![], // Don't panic here to allow for the S logic
            _ => panic!("Error, didn't get connected coords for {:?}", self),
        }
    }

    fn get_effective_shape(&self, map: &HashMap<Coord, Node>) -> char {
        match self.shape {
            'S' => self.workout_s_shape(map),
            _ => self.shape,
        }
    }

    fn workout_s_shape(&self, map: &HashMap<Coord, Node>) -> char {
        // Special case for S
        assert!(self.shape == 'S');

        let neighbors = self.coord.get_neighbors();

        let mut connected_coords = vec![];

        for neighbor in neighbors {
            if map.contains_key(&neighbor) {
                let neighbor_connections = map.get(&neighbor).unwrap().get_connections(map);

                if neighbor_connections.contains(&self.coord) {
                    connected_coords.push(neighbor);
                }
            }
        }
        connected_coords.sort();

        let all_shapes = vec!['|', '-', 'L', 'J', '7', 'F'];
        let possible_node = all_shapes
            .into_iter()
            .map(|shape| Node {
                coord: self.coord.clone(),
                shape,
            })
            .collect::<Vec<Node>>();

        let node = possible_node
            .clone()
            .into_iter()
            .filter(|node| {
                let mut coords = node.get_connections(map).clone();
                coords.sort();
                coords == connected_coords
            })
            .collect::<Vec<Node>>();

        assert!(node.len() == 1);
        node[0].shape
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
        assert_eq!(
            Day10::solve_part_two(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            ),
            "4".to_string()
        )
    }

    #[test]
    fn check_day10_part2_case2() {
        assert_eq!(
            Day10::solve_part_two(
                "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
            ),
            "4".to_string()
        )
    }

    #[test]
    fn check_day10_part2_case3() {
        assert_eq!(
            Day10::solve_part_two(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            "8".to_string()
        )
    }

    #[test]
    fn check_day10_part2_case4() {
        assert_eq!(
            Day10::solve_part_two(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            "10".to_string()
        )
    }

    #[test]
    fn check_day10_both_case1() {
        assert_eq!(Day10::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
