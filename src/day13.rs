use crate::Solution;
use regex::Regex;
use std::collections::{btree_set::Intersection, HashSet};

#[derive(Clone, Debug)]
pub struct Day13;

impl Solution for Day13 {
    type ParsedInput = Vec<Vec<Vec<char>>>;

    fn parse_input(input: &str) -> Self::ParsedInput {
        let re = Regex::new(r"(?<pattern>(([.#]+\n?)+))").unwrap();

        let mut patterns = vec![];
        for cap in re.captures_iter(input) {
            let pattern = cap.name("pattern").unwrap().as_str();
            let pattern = pattern
                .lines()
                .map(|x| x.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

            patterns.push(pattern);
        }

        patterns
    }

    fn part_one(patterns: &mut Self::ParsedInput) -> String {
        let sum: usize = patterns.iter().cloned().map(|x| get_summary(x)).sum();

        sum.to_string()
    }

    fn part_two(patterns: &mut Self::ParsedInput) -> String {
        let sum: usize = patterns
            .iter()
            .cloned()
            .map(|x| get_summary_smudged(x))
            .sum();
        sum.to_string()
    }
}

fn new_unsmudged_index(mut pattern: Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let vertical_mirror_right_index = get_vertical_mirror_index(&pattern);
    let pattern_transpose = transpose(&pattern);
    let horizontal_mirror_right_index = get_vertical_mirror_index(&pattern_transpose);

    for y in 0..pattern.len() {
        for x in 0..pattern[0].len() {
            let value = pattern[y][x];
            let opposite_value = opposite_value(value);

            pattern[y][x] = opposite_value;

            let new_vertical_mirror_right_index = get_vertical_mirror_index(&pattern);
            let new_pattern_transpose = transpose(&pattern);
            let new_horizontal_mirror_right_index =
                get_vertical_mirror_index(&new_pattern_transpose);

            let new_vertical_mirror_right_index = new_vertical_mirror_right_index
                .into_iter()
                .filter(|x| !vertical_mirror_right_index.contains(x))
                .collect::<Vec<usize>>();

            let new_horizontal_mirror_right_index = new_horizontal_mirror_right_index
                .into_iter()
                .filter(|x| !horizontal_mirror_right_index.contains(x))
                .collect::<Vec<usize>>();

            if !new_vertical_mirror_right_index.is_empty()
                || !new_horizontal_mirror_right_index.is_empty()
            {
                return (
                    new_vertical_mirror_right_index,
                    new_horizontal_mirror_right_index,
                );
            }

            pattern[y][x] = value;
        }
    }

    panic!("Error, didn't find smudge for {:?}", pattern);
}

fn opposite_value(value: char) -> char {
    match value {
        '#' => '.',
        '.' => '#',
        _ => panic!("Unexpected value {:?}", value),
    }
}

fn get_summary_smudged(pattern: Vec<Vec<char>>) -> usize {
    let (vertical_mirror_right_index, horizontal_mirror_right_index) = new_unsmudged_index(pattern);
    assert!(
        (vertical_mirror_right_index.len() == 1 && horizontal_mirror_right_index.len() == 0)
            || (vertical_mirror_right_index.len() == 0 && horizontal_mirror_right_index.len() == 1)
    );

    // Note, mirror_right_index goes from 0 - len() -1, but in the question it goes from 1 to len()
    // So, the question is asking for stuff to the left of the mirror, but for my implementation.
    // this is to the right.
    let output = vertical_mirror_right_index.iter().sum::<usize>()
        + horizontal_mirror_right_index.iter().sum::<usize>() * 100;

    output
}

fn get_summary(pattern: Vec<Vec<char>>) -> usize {
    let vertical_mirror_right_index = get_vertical_mirror_index(&pattern);
    let pattern_transpose = transpose(&pattern);
    let horizontal_mirror_right_index = get_vertical_mirror_index(&pattern_transpose);

    assert!(
        (vertical_mirror_right_index.len() == 1 && horizontal_mirror_right_index.len() == 0)
            || (vertical_mirror_right_index.len() == 0 && horizontal_mirror_right_index.len() == 1)
    );

    // Note, mirror_right_index goes from 0 - len() -1, but in the question it goes from 1 to len()
    // So, the question is asking for stuff to the left of the mirror, but for my implementation.
    // this is to the right.
    let output = vertical_mirror_right_index.iter().sum::<usize>()
        + horizontal_mirror_right_index.iter().sum::<usize>() * 100;

    output
}

fn get_vertical_mirror_index(pattern: &Vec<Vec<char>>) -> Vec<usize> {
    let mut mirror_indexes_right = (1..pattern[0].len()).collect::<Vec<usize>>();
    for row in pattern {
        mirror_indexes_right = mirror_indexes_right
            .into_iter()
            .filter(|index| is_line_symmetry(&row, *index))
            .collect();
    }
    mirror_indexes_right
}

fn is_line_symmetry(row: &Vec<char>, right_index: usize) -> bool {
    let left = row[..right_index].iter().collect::<Vec<&char>>();
    let right = row[right_index..].iter().collect::<Vec<&char>>();
    let right_rev = right.iter().cloned().rev().collect::<Vec<&char>>();

    left.ends_with(&right_rev) || right_rev.ends_with(&left)
}

#[allow(clippy::needless_range_loop)]
fn transpose<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut new_matrix = vec![];
    for x in 0..matrix[0].len() {
        let mut new_row = vec![];
        for y in 0..matrix.len() {
            new_row.push(matrix[y][x].clone())
        }
        new_matrix.push(new_row)
    }
    new_matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_is_line_symmetry_case1() {
        assert_eq!(
            is_line_symmetry(&vec!['#', '.', '#', '#', '.', '#'], 3),
            true
        )
    }

    #[test]
    fn check_is_line_symmetry_case2() {
        assert_eq!(
            is_line_symmetry(&vec!['#', '.', '#', '#', '.', '#'], 4),
            false
        )
    }

    #[test]
    fn check_is_line_symmetry_case3() {
        assert_eq!(
            is_line_symmetry(&vec!['#', '.', '.', '#', '#', '#', '#', '#', '#', '#'], 2),
            true
        )
    }

    #[test]
    fn check_is_line_symmetry_case4() {
        assert_eq!(
            is_line_symmetry(
                &vec!['#', '#', '#', '#', '#', '#', '#', '.', '.', '#', '#'],
                8
            ),
            true
        )
    }

    #[test]
    fn check_day13_part1_case1() {
        assert_eq!(
            Day13::solve_part_one(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            "405".to_string()
        )
    }

    #[test]
    fn check_day13_part2_case1() {
        assert_eq!(
            Day13::solve_part_two(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            "400".to_string()
        )
    }

    #[test]
    fn check_day13_both_case1() {
        assert_eq!(Day13::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
