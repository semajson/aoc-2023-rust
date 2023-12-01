use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day01;

fn get_nums(line: &str) -> String {
    let just_nums = line
        .chars()
        .filter(|char| char.is_ascii_digit())
        .collect::<String>();
    just_nums
}

fn replace_words_with_nums(line: &str) -> String {
    let line = line.replace("one", "o1e");
    let line = line.replace("two", "t2o");
    let line = line.replace("three", "t3e");
    let line = line.replace("four", "f4r");
    let line = line.replace("five", "f5e");
    let line = line.replace("six", "s6x");
    let line = line.replace("seven", "s7n");
    let line = line.replace("eight", "e8t");
    line.replace("nine", "n9e")
}

fn calc_calibration_value(nums: &str) -> i32 {
    let first_digit = nums.chars().next().unwrap();
    let last_digit = nums.chars().last().unwrap();
    let mut number = first_digit.to_string();
    number.push(last_digit);
    number.parse::<i32>().unwrap()
}

impl Solution for Day01 {
    type ParsedInput = Vec<String>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let input_lines = input_lines.to_string();
        input_lines
            .lines()
            .map(String::from)
            .collect::<Vec<String>>()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let mut total = 0;
        for line in _parsed_input {
            let just_nums = get_nums(line);
            total += calc_calibration_value(&just_nums);
        }
        total.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let mut total = 0;
        for line in _parsed_input {
            let line = replace_words_with_nums(line);
            let just_nums = get_nums(&line);
            total += calc_calibration_value(&just_nums);
        }
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(Day01::solve_part_one(""), "".to_string())
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(Day01::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day01_both_case1() {
        assert_eq!(Day01::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
