use crate::Solution;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Day12;

impl Solution for Day12 {
    type ParsedInput = Vec<(Vec<char>, Vec<usize>)>;

    // Todo - this is wrong.
    // Use regex

    fn parse_input(input: &str) -> Self::ParsedInput {
        // let input_lines = input
        //     .lines()
        //     .map(|x| x.to_string())
        //     .collect::<Vec<String>>();

        // input_lines
        //     .iter()
        //     .map(|x| parse_row(x))
        //     .collect::<Vec<Vec<char>>>()

        let mut parsed_input = vec![];

        let re = Regex::new(r"(?<row>[?#.]+) (?<groups>[0-9,]+)").unwrap();

        for cap in re.captures_iter(input) {
            let row = cap.name("row").unwrap().as_str();
            let row = parse_row(&row);

            let groups = cap.name("groups").unwrap().as_str();
            let groups = parse_groups(&groups);

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
        // TODO: implement part two
        0.to_string()
    }
}

pub fn parse_row(input_line: &str) -> Vec<char> {
    input_line.chars().collect()
}
pub fn parse_groups(input_group: &str) -> Vec<usize> {
    input_group
        .split(",")
        .map(|x| x.to_string().parse::<usize>().unwrap())
        .collect()
}

pub fn get_contiguous_damaged_groups(input: &Vec<char>) -> Vec<usize> {
    let mut groups: Vec<usize> = vec![];

    let mut current_group_let = 0;
    for spring in input {
        match spring {
            '#' => {
                current_group_let = current_group_let + 1;
            }
            '.' => {
                groups.push(current_group_let);
                current_group_let = 0;
            }
            _ => panic!("Unexpected input {:?}", spring),
        };
    }
    if current_group_let != 0 {
        groups.push(current_group_let);
    }

    groups
}

pub fn get_arrangements(row: &Vec<char>, groups: &Vec<usize>) -> usize {
    get_arrangements_recursive(groups, row, &vec![], 0)
}

fn get_arrangements_recursive(
    expected_groups: &Vec<usize>,
    remaining_row: &[char],
    found_groups: &Vec<usize>,
    current_group: usize,
) -> usize {
    if !expected_groups.starts_with(&found_groups) {
        0
    } else if remaining_row.is_empty() {
        let mut all_found_groups = found_groups.clone();
        if current_group != 0 {
            all_found_groups.push(current_group);
        }

        if expected_groups == &all_found_groups {
            1
        } else {
            0
        }
    } else {
        let (current_spring, remaining_row) = remaining_row.split_first().unwrap();
        match current_spring {
            '#' => get_arrangements_recursive(
                expected_groups,
                remaining_row,
                found_groups,
                current_group + 1,
            ),
            '.' => {
                let mut new_found_group = found_groups.clone();
                if current_group != 0 {
                    new_found_group.push(current_group);
                }
                get_arrangements_recursive(expected_groups, remaining_row, &new_found_group, 0)
            }
            '?' => {
                let branch_1 = get_arrangements_recursive(
                    expected_groups,
                    remaining_row,
                    found_groups,
                    current_group + 1,
                );

                let mut new_found_group = found_groups.clone();
                if current_group != 0 {
                    new_found_group.push(current_group);
                }
                let branch_2 =
                    get_arrangements_recursive(expected_groups, remaining_row, &new_found_group, 0);

                branch_1 + branch_2
            }
            _ => panic!("Error unexpected spring {:?}", current_spring),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_contiguous_damaged_groups_cas1() {
        assert_eq!(
            get_contiguous_damaged_groups(&parse_row("#.#.###")),
            vec![1, 1, 3]
        )
    }

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
        assert_eq!(Day12::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day12_both_case1() {
        assert_eq!(Day12::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
