use crate::Solution;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Day12;

impl Solution for Day12 {
    type ParsedInput = Vec<(Vec<char>, Vec<usize>)>;

    fn parse_input(input: &str) -> Self::ParsedInput {
        let mut parsed_input = vec![];

        let re = Regex::new(r"(?<row>[?#.]+) (?<groups>[0-9,]+)").unwrap();

        for cap in re.captures_iter(input) {
            let row = cap.name("row").unwrap().as_str();
            let row = parse_row(row);

            let groups = cap.name("groups").unwrap().as_str();
            let groups = parse_groups(groups);

            parsed_input.push((row, groups));
        }

        parsed_input
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let sum: usize = _parsed_input
            .iter()
            .map(|(row, groups)| get_arrangements(row, groups))
            .sum();

        sum.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let day_2_input = _parsed_input
            .iter_mut()
            .map(|(row, groups)| convert_to_day_2_input(row, groups))
            .collect::<Vec<(Vec<char>, Vec<usize>)>>();
        let sum: usize = day_2_input
            .iter()
            .map(|(row, groups)| get_arrangements(row, groups))
            .sum();

        sum.to_string()
    }
}

pub fn parse_row(input_line: &str) -> Vec<char> {
    input_line.chars().collect()
}
pub fn parse_groups(input_group: &str) -> Vec<usize> {
    input_group
        .split(',')
        .map(|x| x.to_string().parse::<usize>().unwrap())
        .collect()
}

pub fn convert_to_day_2_input(row: &[char], groups: &[usize]) -> (Vec<char>, Vec<usize>) {
    let repetitions = 5;
    let mut new_rows = vec![];
    let mut new_groups = vec![];
    // Must be a better way to do this
    for _ in 0..repetitions {
        if !new_rows.is_empty() {
            new_rows.push('?');
        }
        new_groups.extend(groups);

        new_rows.extend(row);
    }

    (new_rows, new_groups)
}

pub fn get_arrangements(row: &[char], groups: &Vec<usize>) -> usize {
    get_arrangements_recursive(groups, row, &vec![], 0, &mut HashMap::new())
}

#[allow(clippy::type_complexity)] // Allow this as trying to fix it was a lifetime nightmare. todo - understand lifetimes
fn get_arrangements_recursive<'a>(
    expected_groups: &Vec<usize>,
    remaining_row: &'a [char],
    found_groups: &Vec<usize>,
    current_group: usize,
    cache: &mut HashMap<(Vec<usize>, &'a [char], Vec<usize>, usize), usize>,
) -> usize {
    let cache_key = (
        expected_groups.clone(),
        remaining_row,
        found_groups.clone(),
        current_group,
    );
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    let result: usize;

    if !expected_groups.starts_with(found_groups) {
        result = 0;
    } else if remaining_row.is_empty() {
        let mut all_found_groups = found_groups.clone();
        if current_group != 0 {
            all_found_groups.push(current_group);
        }

        if expected_groups == &all_found_groups {
            result = 1;
        } else {
            result = 0;
        }
    } else {
        let (current_spring, remaining_row) = remaining_row.split_first().unwrap();
        result = match current_spring {
            '#' => get_arrangements_recursive(
                expected_groups,
                remaining_row,
                found_groups,
                current_group + 1,
                cache,
            ),
            '.' => {
                let mut new_found_group = found_groups.clone();
                if current_group != 0 {
                    new_found_group.push(current_group);
                }
                get_arrangements_recursive(
                    expected_groups,
                    remaining_row,
                    &new_found_group,
                    0,
                    cache,
                )
            }
            '?' => {
                let branch_1 = get_arrangements_recursive(
                    expected_groups,
                    remaining_row,
                    found_groups,
                    current_group + 1,
                    cache,
                );

                let mut new_found_group = found_groups.clone();
                if current_group != 0 {
                    new_found_group.push(current_group);
                }
                let branch_2 = get_arrangements_recursive(
                    expected_groups,
                    remaining_row,
                    &new_found_group,
                    0,
                    cache,
                );

                branch_1 + branch_2
            }
            _ => panic!("Error unexpected spring {:?}", current_spring),
        };
    }
    cache.insert(cache_key, result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_arrangements_cases_1() {
        assert_eq!(
            get_arrangements(&parse_row("???.###"), &parse_groups("1,1,3")),
            1
        )
    }

    #[test]
    fn check_day12_part1_case1() {
        assert_eq!(
            Day12::solve_part_one(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            ),
            "21".to_string()
        )
    }

    #[test]
    fn check_day12_part2_case1() {
        assert_eq!(
            Day12::solve_part_two(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            ),
            "525152".to_string()
        )
    }

    #[test]
    fn check_day12_both_case1() {
        assert_eq!(Day12::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
