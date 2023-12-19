use std::os::unix::fs::MetadataExt;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day14;

impl Solution for Day14 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        // et input_lines = input_lines.to_string();
        input
            .lines()
            .map(String::from)
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        let mut grid = input.clone();
        tilt_north(&mut grid);

        calc_north_load(&grid).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                let mut new_y = y;
                for potential_new_y in (0..y).rev() {
                    match grid[potential_new_y][x] {
                        '#' => break,
                        'O' => break,
                        '.' => new_y = potential_new_y,
                        _ => panic!("Unexpected value {:?}", potential_new_y),
                    }
                }
                grid[y][x] = '.';
                grid[new_y][x] = 'O';
            }
        }
    }
}

fn calc_north_load(grid: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for (index, row) in grid.iter().enumerate() {
        let num_round_rock = row.iter().filter(|x| **x == 'O').count();
        sum += num_round_rock * (grid.len() - index);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_tilt_north() {
        let mut grid = vec![vec!['.', '.'], vec!['O', '.']];
        tilt_north(&mut grid);
        assert_eq!(grid, vec![vec!['O', '.'], vec!['.', '.']]);
    }

    #[test]
    fn check_day14_part1_case1() {
        assert_eq!(
            Day14::solve_part_one(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            ),
            "136".to_string()
        )
    }

    #[test]
    fn check_day14_part2_case1() {
        assert_eq!(
            Day14::solve_part_two(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            ),
            "64".to_string()
        )
    }

    #[test]
    fn check_day14_both_case1() {
        assert_eq!(
            Day14::solve(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
                false
            ),
            ("136".to_string(), "0".to_string())
        )
    }
}
