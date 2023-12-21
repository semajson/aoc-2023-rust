use crate::Solution;
use ndarray::{arr1, arr2, Array1};
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Day16;

impl Solution for Day16 {
    type ParsedInput = (Vec<Vec<char>>, HashMap<Array1<isize>, char>);

    fn parse_input(input: &str) -> Self::ParsedInput {
        let input_lines = input.lines().collect::<Vec<&str>>();

        let grid = input_lines
            .iter()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut new_grid = HashMap::new();

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                new_grid.insert(arr1(&[x as isize, y as isize]), grid[y][x]);
            }
        }
        (grid, new_grid)
    }

    fn part_one((raw_grid, grid): &mut Self::ParsedInput) -> String {
        let sum = get_energized(
            Ray {
                pos: arr1(&[-1, 0]),
                velocity: arr1(&[1, 0]),
            },
            grid,
            raw_grid,
        );

        sum.to_string()
    }

    fn part_two((raw_grid, grid): &mut Self::ParsedInput) -> String {
        let mut highest_energized = 0;

        let x_len = raw_grid[0].len() as isize;
        let y_len = raw_grid.len() as isize;

        for y in 0..y_len {
            let left_start_energized = get_energized(
                Ray {
                    pos: arr1(&[-1, y]),
                    velocity: arr1(&[1, 0]),
                },
                grid,
                raw_grid,
            );

            let right_start_energized = get_energized(
                Ray {
                    pos: arr1(&[x_len, y]),
                    velocity: arr1(&[-1, 0]),
                },
                grid,
                raw_grid,
            );

            highest_energized = cmp::max(
                cmp::max(left_start_energized, right_start_energized),
                highest_energized,
            );
        }

        for x in 0..x_len {
            let top_start_energized = get_energized(
                Ray {
                    pos: arr1(&[x, -1]),
                    velocity: arr1(&[0, 1]),
                },
                grid,
                raw_grid,
            );

            let bottom_start_energized = get_energized(
                Ray {
                    pos: arr1(&[x, y_len]),
                    velocity: arr1(&[0, 1]),
                },
                grid,
                raw_grid,
            );

            highest_energized = cmp::max(
                cmp::max(top_start_energized, bottom_start_energized),
                highest_energized,
            );
        }

        highest_energized.to_string()
    }
}

#[allow(unused_variables)]
fn get_energized(
    starting_ray: Ray,
    grid: &HashMap<Array1<isize>, char>,
    raw_grid: &[Vec<char>],
) -> usize {
    let mut rays = vec![starting_ray];

    let mut energized = HashSet::new();
    let mut seen_rays = HashSet::new();
    // let mut debug = get_debug_grid(&energized, &raw_grid);

    while !rays.is_empty() {
        // debug = get_debug_grid(&energized, &raw_grid);
        rays = get_new_rays(rays, grid)
            .into_iter()
            .filter(|ray| !seen_rays.contains(ray))
            .collect();

        for ray in rays.iter() {
            energized.insert(ray.pos.clone());
            seen_rays.insert(ray.clone());
        }
    }

    // print_debug_grid(&energized, &raw_grid);

    energized.len()
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Ray {
    pos: Array1<isize>,
    velocity: Array1<isize>,
}

fn get_new_rays(rays: Vec<Ray>, grid: &HashMap<Array1<isize>, char>) -> Vec<Ray> {
    let mut new_rays = vec![];

    for ray in rays {
        let next_pos = ray.pos + ray.velocity.clone();
        if let Some(next_char) = grid.get(&next_pos) {
            match *next_char {
                '.' => new_rays.push(Ray {
                    pos: next_pos,
                    velocity: ray.velocity,
                }),
                '-' => {
                    let next_velocity_1 = arr2(&[[1, 1], [0, 0]]).dot(&ray.velocity);
                    let next_velocity_2 = arr2(&[[1, -1], [0, 0]]).dot(&ray.velocity);
                    new_rays.push(Ray {
                        pos: next_pos.clone(),
                        velocity: next_velocity_1.clone(),
                    });

                    if next_velocity_1 != next_velocity_2 {
                        new_rays.push(Ray {
                            pos: next_pos,
                            velocity: next_velocity_2,
                        })
                    }
                }
                '|' => {
                    let next_velocity_1 = arr2(&[[0, 0], [1, 1]]).dot(&ray.velocity);
                    let next_velocity_2 = arr2(&[[0, 0], [-1, 1]]).dot(&ray.velocity);
                    new_rays.push(Ray {
                        pos: next_pos.clone(),
                        velocity: next_velocity_1.clone(),
                    });
                    if next_velocity_1 != next_velocity_2 {
                        new_rays.push(Ray {
                            pos: next_pos,
                            velocity: next_velocity_2,
                        })
                    }
                }
                '/' => {
                    let next_velocity = arr2(&[[0, -1], [-1, 0]]).dot(&ray.velocity);
                    new_rays.push(Ray {
                        pos: next_pos,
                        velocity: next_velocity,
                    });
                }
                '\\' => {
                    let next_velocity = arr2(&[[0, 1], [1, 0]]).dot(&ray.velocity);
                    new_rays.push(Ray {
                        pos: next_pos,
                        velocity: next_velocity,
                    });
                }
                _ => panic!("Unexpected pattern {:?}", next_char),
            }
        }
    }

    new_rays
}

#[allow(dead_code)]
fn get_debug_grid(energized: &HashSet<Array1<isize>>, raw_grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut energized_grid = vec![vec!['.'; raw_grid[0].len()]; raw_grid.len()];
    for pos in energized {
        let x = pos[0] as usize;
        let y = pos[1] as usize;
        energized_grid[y][x] = '#';
    }
    // for row in energized_grid {
    //     println!("{:?}", row);
    // }
    energized_grid
}

#[allow(dead_code)]
fn print_debug_grid(energized: &HashSet<Array1<isize>>, raw_grid: &Vec<Vec<char>>) {
    let mut energized_grid = vec![vec!['.'; raw_grid[0].len()]; raw_grid.len()];
    for pos in energized {
        let x = pos[0] as usize;
        let y = pos[1] as usize;
        energized_grid[y][x] = '#';
    }
    for row in energized_grid {
        println!("{:?}", row);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day16_part1_case1() {
        assert_eq!(
            Day16::solve_part_one(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            ),
            "46".to_string()
        )
    }

    #[test]
    fn check_day16_part2_case1() {
        assert_eq!(
            Day16::solve_part_two(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            ),
            "51".to_string()
        )
    }

    #[test]
    fn check_day16_both_case1() {
        assert_eq!(
            Day16::solve(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
                false
            ),
            ("0".to_string(), "0".to_string())
        )
    }
}
