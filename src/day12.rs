use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day12;

impl Solution for Day12 {
    type ParsedInput = Vec<Vec<char>>;

    // Todo - this is wrong.
    // Use regex
    fn parse_input(input: &str) -> Self::ParsedInput {
        let input_lines = input
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        input_lines
            .iter()
            .map(|x| parse_input_line(x))
            .collect::<Vec<Vec<char>>>()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part one
        0.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

pub fn parse_input_line(input_line: &str) -> Vec<char> {
    input_line.chars().collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_contiguous_damaged_groups_cas1() {
        assert_eq!(
            get_contiguous_damaged_groups(&parse_input_line("#.#.###")),
            vec![1, 1, 3]
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
