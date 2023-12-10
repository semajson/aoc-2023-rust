use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day09;

impl Solution for Day09 {
    type ParsedInput = Vec<Vec<isize>>;

    fn parse_input(input: &str) -> Self::ParsedInput {
        let input_lines = input
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        input_lines
            .iter()
            .map(|x| {
                x.split_whitespace()
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .collect::<Vec<Vec<isize>>>()
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        let mut sum = 0;
        for data_series in input {
            sum += next_value(data_series);
        }
        sum.to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        let mut sum = 0;
        for data_series in input {
            sum += prev_value(data_series);
        }
        sum.to_string()
    }
}

fn next_value(data_series: &Vec<isize>) -> isize {
    if is_all_zeros(data_series) {
        return 0;
    }

    let differences = get_differences(data_series);

    data_series.last().unwrap() + next_value(&differences)
}

fn prev_value(data_series: &Vec<isize>) -> isize {
    if is_all_zeros(data_series) {
        return 0;
    }

    let differences = get_differences(data_series);

    data_series.first().unwrap() - prev_value(&differences)
}

fn is_all_zeros(data_series: &Vec<isize>) -> bool {
    let num_zeros = data_series
        .iter()
        .filter(|&&x| x == 0)
        .collect::<Vec<&isize>>()
        .len();

    data_series.len() == num_zeros
}

fn get_differences(data_series: &Vec<isize>) -> Vec<isize> {
    let mut differences = vec![];

    for i in 0..(data_series.len() - 1) {
        differences.push(data_series[i + 1] - data_series[i]);
    }
    differences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day09_part1_case1() {
        assert_eq!(
            Day09::solve_part_one(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            "114".to_string()
        )
    }

    #[test]
    fn check_day09_part2_case1() {
        assert_eq!(
            Day09::solve_part_two(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            "2".to_string()
        )
    }

    #[test]
    fn check_day09_both_case1() {
        assert_eq!(Day09::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
