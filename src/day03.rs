use std::collections::HashSet;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day03;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Point {
    x: isize,
    y: isize,
}
impl Point {
    pub fn get_neighbors(&self) -> Vec<Point> {
        let mut neighbors = vec![];
        for x in -1..=1 {
            for y in -1..=1 {
                if !(x == 0 && y == 0) {
                    neighbors.push(Point {
                        x: self.x + x,
                        y: self.y + y,
                    })
                }
            }
        }

        neighbors
    }

    pub fn get_neighbors_set(&self) -> HashSet<Point> {
        HashSet::from_iter(self.get_neighbors())
    }
}

#[derive(Clone, Debug)]
pub struct Number {
    value: isize,
    coords: HashSet<Point>,
}

#[derive(Clone, Debug)]
pub struct Grid(Vec<Vec<char>>);
impl Grid {
    fn y_max(&self) -> usize {
        self.0.len()
    }
    fn x_max(&self) -> usize {
        self.0[0].len()
    }
    pub fn get_all_numbers(&self) -> Vec<Number> {
        let mut numbers = Vec::new();
        for y in 0..self.y_max() {
            for x in 0..self.x_max() {
                if self.is_start_of_number(x, y) {
                    numbers.push(self.get_full_number(x, y))
                }
            }
        }
        numbers
    }

    fn is_start_of_number(&self, x: usize, y: usize) -> bool {
        self.0[y][x].is_ascii_digit() && ((x == 0) || !self.0[y][x - 1].is_ascii_digit())
    }

    fn get_full_number(&self, x: usize, y: usize) -> Number {
        let mut number_value = String::from(self.0[y][x]);
        let mut coords = HashSet::new();
        coords.insert(Point {
            x: x as isize,
            y: y as isize,
        });

        let mut next_x = x + 1;
        while (next_x < self.x_max()) && self.0[y][next_x].is_ascii_digit() {
            coords.insert(Point {
                x: next_x as isize,
                y: y as isize,
            });
            number_value.push(self.0[y][next_x]);
            next_x += 1;
        }

        Number {
            value: number_value.parse::<isize>().unwrap(),
            coords,
        }
    }

    pub fn get_part_numbers(&self) -> Vec<Number> {
        let mut part_numbers = Vec::new();

        for number in self.get_all_numbers() {
            let neighbors = self.get_number_neighbors(&number);
            if neighbors.iter().any(|neighbor| self.is_symbol(neighbor)) {
                part_numbers.push(number);
            }
        }

        part_numbers
    }

    fn get_number_neighbors(&self, number: &Number) -> HashSet<Point> {
        let mut neighbors = HashSet::new();

        for coord in number.coords.iter() {
            let coord_neighbors = self
                .get_point_neighbors(coord)
                .into_iter()
                .filter(|point| (!number.coords.contains(point)))
                .filter(|point| (!neighbors.contains(point)))
                .collect::<HashSet<Point>>();

            neighbors.extend(coord_neighbors);
        }
        neighbors
    }

    fn get_point_neighbors(&self, point: &Point) -> HashSet<Point> {
        point
            .get_neighbors_set()
            .into_iter()
            .filter(|point| self.is_point_in_grid(point))
            .collect::<HashSet<Point>>()
    }

    fn is_point_in_grid(&self, point: &Point) -> bool {
        (point.x >= 0)
            && (point.x < self.x_max() as isize)
            && (point.y >= 0)
            && (point.y < self.y_max() as isize)
    }

    fn is_symbol(&self, point: &Point) -> bool {
        let value = self.0[point.y as usize][point.x as usize];
        (!value.is_ascii_digit()) && value != '.'
    }

    fn get_ratios(&self) -> Vec<isize> {
        let mut gear_ratios = vec![];

        let potential_gears = self.get_potential_gears();

        let part_numbers = self.get_part_numbers();

        for potential_gear in potential_gears {
            let matching_part_numbers =
                self.get_surrounding_part_numbers(&potential_gear, &part_numbers);

            if matching_part_numbers.len() == 2 {
                gear_ratios.push(matching_part_numbers[0].value * matching_part_numbers[1].value);
            }
        }

        gear_ratios
    }

    fn get_surrounding_part_numbers<'c>(
        &self,
        point: &Point,
        part_numbers: &'c [Number],
    ) -> Vec<&'c Number> {
        let neighbors: HashSet<Point> = self.get_point_neighbors(point);

        let mut matching_part_numbers = vec![];

        for part_number in part_numbers.iter() {
            if !part_number.coords.is_disjoint(&neighbors) {
                matching_part_numbers.push(part_number);
            }
        }
        matching_part_numbers
    }

    fn get_potential_gears(&self) -> Vec<Point> {
        let mut potential_gears = vec![];
        for y in 0..self.y_max() {
            for x in 0..self.x_max() {
                if self.0[y][x] == '*' {
                    potential_gears.push(Point {
                        x: x as isize,
                        y: y as isize,
                    });
                }
            }
        }
        potential_gears
    }
}

impl Solution for Day03 {
    type ParsedInput = Grid;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let input_lines = input_lines.to_string();
        let test = input_lines
            .lines()
            .map(String::from)
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Grid(test)
    }

    fn part_one(grid: &mut Self::ParsedInput) -> String {
        let part_numbers = grid.get_part_numbers();

        let sum: isize = part_numbers
            .iter()
            .map(|part_number| part_number.value)
            .sum();
        sum.to_string()
    }

    fn part_two(grid: &mut Self::ParsedInput) -> String {
        let sum: isize = grid.get_ratios().iter().sum();
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(
            Day03::solve_part_one(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            "4361".to_string()
        )
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(
            Day03::solve_part_two(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            "467835".to_string()
        )
    }

    #[test]
    fn check_day03_both_case1() {
        assert_eq!(Day03::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
