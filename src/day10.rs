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

        let starting_node = get_starting_node(&map);

        let (visited, _, _) = traverse_loop(starting_node, map);

        let max = visited.values().max().unwrap();

        max.to_string()
    }

    fn part_two(map: &mut Self::ParsedInput) -> String {
        let map = map.clone(); //defensive, might not need

        // Find all the ground groups
        let ground_groups = get_ground_groups(&map);

        // println!("{:?}", ground_groups);

        let starting_node = get_starting_node(&map);

        let (_, perimeter_a, perimeter_b) = traverse_loop(starting_node, map);

        println!("test");

        0.to_string()
    }
}

fn get_ground_groups(map: &HashMap<Coord, Node>) -> Vec<HashSet<Node>> {
    // This is a bit messy, could be improved
    let ground_nodes = map
        .values()
        .filter(|node| node.is_ground())
        .map(|node| node.clone())
        .collect::<Vec<Node>>();

    let mut ground_groups: Vec<HashSet<Node>> = vec![];
    for node in ground_nodes {
        let adjacent_coords = node.coord.get_adjacent_neighbors();
        let ground_neighbors = adjacent_coords
            .iter()
            .filter(|coord| map.contains_key(&coord) && map.get(&coord).unwrap().is_ground())
            .map(|coord| map.get(&coord).unwrap().clone())
            .collect::<HashSet<Node>>();

        let mut found_group = ground_neighbors;
        found_group.insert(node.clone());

        // Now, add the newly found group to ground_groups.
        // Note, this might involve combining 2 existing groups.
        let mut matches = vec![found_group.clone()];

        for group in ground_groups.iter() {
            if !group.is_disjoint(&found_group) {
                matches.push(group.clone());
            }
        }
        ground_groups = ground_groups
            .into_iter()
            .filter(|group| !matches.contains(group))
            .collect();

        // Collapse into one hashset then re-add
        let new_group = matches
            .into_iter()
            .fold(HashSet::new(), |mut new_group, x| {
                new_group.extend(x);
                new_group
            });
        ground_groups.push(new_group);
    }
    ground_groups
}

fn traverse_loop(
    starting_node: Node,
    map: HashMap<Coord, Node>,
) -> (HashMap<Node, i32>, HashSet<Node>, HashSet<Node>) {
    let mut visited = HashMap::new();
    visited.insert(starting_node.clone(), 0);

    let (mut perimeter_a, mut perimeter_b) = starting_node.get_perimeter_ground_nodes(&map);

    let mut current_nodes = vec![starting_node];

    let mut steps = 0;

    while !current_nodes.is_empty() {
        steps += 1;

        let possible_next_nodes = get_possible_next_nodes(&current_nodes, &map);

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

            let (new_perimeter_c, new_perimeter_d) = node.get_perimeter_ground_nodes(&map);

            let mut added_c = false;
            let mut added_d = false;
            let mut added_to_a = false;
            let mut added_to_b = false;

            if !new_perimeter_c.is_disjoint(&perimeter_a) && !perimeter_a.is_empty() {
                assert!(!added_c);
                perimeter_a.extend(new_perimeter_c.clone());
                added_c = true;
                added_to_a = true;
            }
            if !new_perimeter_c.is_disjoint(&perimeter_b) && !perimeter_b.is_empty() {
                assert!(!added_c);
                perimeter_b.extend(new_perimeter_c.clone());
                added_c = true;
                added_to_b = true;
            }
            if !new_perimeter_d.is_disjoint(&perimeter_a) && !perimeter_a.is_empty() {
                assert!(!added_d);
                perimeter_a.extend(new_perimeter_d.clone());
                added_d = true;
                added_to_a = true;
            }
            if !new_perimeter_d.is_disjoint(&perimeter_b) && !perimeter_b.is_empty() {
                if added_d {
                    println!("test");
                }
                assert!(!added_d);

                perimeter_b.extend(new_perimeter_d.clone());
                added_d = true;
                added_to_b = true;
            }

            if !(added_c || added_d) {
                println!("crap");
            }
            assert!(added_c || added_d);

            if !added_c {
                if !added_to_b {
                    perimeter_b.extend(new_perimeter_c.clone());
                } else if !added_to_a {
                    perimeter_a.extend(new_perimeter_c.clone());
                }
            }
            if !added_d {
                if !added_to_b {
                    perimeter_b.extend(new_perimeter_d.clone()); // here
                } else if !added_to_a {
                    perimeter_a.extend(new_perimeter_d.clone());
                }
            }
            assert!(perimeter_a.is_disjoint(&perimeter_b));
            println!("tet");
        }
    }
    (visited, perimeter_a, perimeter_b)
}

fn get_starting_node(map: &HashMap<Coord, Node>) -> Node {
    let starting_node = map
        .values()
        .filter(|&node| node.shape == 'S')
        .collect::<Vec<&Node>>();

    assert!(starting_node.len() == 1);
    let starting_node = starting_node[0].clone();
    starting_node
}

fn get_possible_next_nodes(current_nodes: &Vec<Node>, map: &HashMap<Coord, Node>) -> Vec<Node> {
    let next_coords = current_nodes
        .iter()
        .map(|node| node.get_connections(map))
        .flatten()
        .collect::<Vec<Coord>>();

    let possible_next_nodes = next_coords
        .iter()
        .map(|coord| map.get(&coord).unwrap().clone())
        .collect::<Vec<Node>>();
    possible_next_nodes
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
            let neighbor_connections = map.get(&neighbor).unwrap().get_connections(map);

            if neighbor_connections.contains(&self.coord) {
                connected_coords.push(neighbor);
            }
        }
        connected_coords.sort();

        let valid_shapes = vec!['|', '-', 'L', 'J', '7', 'F'];
        let matching_shapes = valid_shapes
            .into_iter()
            .map(|shape| Node {
                coord: self.coord.clone(),
                shape,
            })
            .collect::<Vec<Node>>();

        let matching_shapes = matching_shapes
            .clone()
            .into_iter()
            .filter(|node| {
                let mut coords = node.get_connections(map).clone();
                coords.sort();
                coords == connected_coords
            })
            .collect::<Vec<Node>>();

        assert!(matching_shapes.len() == 1);
        matching_shapes[0].shape
    }

    fn is_ground(&self) -> bool {
        self.shape == '.'
    }

    pub fn get_perimeter_ground_nodes(
        &self,
        map: &HashMap<Coord, Node>,
    ) -> (HashSet<Node>, HashSet<Node>) {
        let (perimeter_a_coord, perimeter_b_coord) = self.get_perimeter_coords(map);

        let perimeter_a_nodes = perimeter_a_coord
            .iter()
            .map(|coord| map.get(coord).unwrap().clone())
            // .filter(|x| x.is_ground())
            .collect::<HashSet<Node>>();
        let perimeter_b_nodes = perimeter_b_coord
            .iter()
            .map(|coord| map.get(coord).unwrap().clone())
            // .filter(|x| x.is_ground())
            .collect::<HashSet<Node>>();
        (perimeter_a_nodes, perimeter_b_nodes)
    }
    pub fn get_perimeter_coords(&self, map: &HashMap<Coord, Node>) -> (Vec<Coord>, Vec<Coord>) {
        match self.get_effective_shape(map) {
            '|' => (
                vec![
                    Coord::from(self.coord.x + 1, self.coord.y + 1),
                    Coord::from(self.coord.x + 1, self.coord.y),
                    Coord::from(self.coord.x + 1, self.coord.y - 1),
                ],
                vec![
                    Coord::from(self.coord.x - 1, self.coord.y + 1),
                    Coord::from(self.coord.x - 1, self.coord.y),
                    Coord::from(self.coord.x - 1, self.coord.y - 1),
                ],
            ),
            '-' => (
                vec![
                    Coord::from(self.coord.x - 1, self.coord.y + 1),
                    Coord::from(self.coord.x, self.coord.y + 1),
                    Coord::from(self.coord.x + 1, self.coord.y + 1),
                ],
                vec![
                    Coord::from(self.coord.x - 1, self.coord.y - 1),
                    Coord::from(self.coord.x, self.coord.y - 1),
                    Coord::from(self.coord.x + 1, self.coord.y - 1),
                ],
            ),
            'L' => (
                vec![
                    Coord::from(self.coord.x - 1, self.coord.y - 1),
                    Coord::from(self.coord.x - 1, self.coord.y),
                    Coord::from(self.coord.x - 1, self.coord.y + 1),
                    Coord::from(self.coord.x, self.coord.y + 1),
                    Coord::from(self.coord.x + 1, self.coord.y + 1),
                ],
                vec![],
            ),
            'J' => (
                vec![
                    Coord::from(self.coord.x + 1, self.coord.y - 1),
                    Coord::from(self.coord.x + 1, self.coord.y),
                    Coord::from(self.coord.x + 1, self.coord.y + 1),
                    Coord::from(self.coord.x, self.coord.y + 1),
                    Coord::from(self.coord.x - 1, self.coord.y + 1),
                ],
                vec![],
            ),

            '7' => (
                vec![
                    Coord::from(self.coord.x - 1, self.coord.y - 1),
                    Coord::from(self.coord.x, self.coord.y - 1),
                    Coord::from(self.coord.x + 1, self.coord.y - 1),
                    Coord::from(self.coord.x + 1, self.coord.y),
                    Coord::from(self.coord.x + 1, self.coord.y + 1),
                ],
                vec![],
            ),
            'F' => (
                vec![
                    Coord::from(self.coord.x - 1, self.coord.y - 1),
                    Coord::from(self.coord.x, self.coord.y - 1),
                    Coord::from(self.coord.x + 1, self.coord.y - 1),
                    Coord::from(self.coord.x - 1, self.coord.y),
                    Coord::from(self.coord.x - 1, self.coord.y + 1),
                ],
                vec![],
            ),
            '.' => panic!("Error, trying to find perimeter of ground {:?}", self),
            _ => panic!("Error, didn't get connected coords for {:?}", self),
        }
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
            "0".to_string()
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
            "0".to_string()
        )
    }

    #[test]
    fn check_day10_both_case1() {
        assert_eq!(Day10::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
