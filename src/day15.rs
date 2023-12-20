use crate::Solution;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Day15;

impl Solution for Day15 {
    type ParsedInput = Vec<Step>;

    fn parse_input(input: &str) -> Self::ParsedInput {
        let re = Regex::new(r"(?<instruction>[a-zA-Z]+)(?<operator>[-=])(?<operator_value>[0-9]*)")
            .unwrap();

        let mut steps = vec![];
        for cap in re.captures_iter(input) {
            let instruction = cap.name("instruction").unwrap().as_str().to_string();

            let operator = cap.name("operator").unwrap().as_str();
            let operator = operator.chars().collect::<Vec<char>>()[0];

            let raw_operator_value = cap.name("operator_value").unwrap().as_str();
            let operator_value: Option<usize>;
            if raw_operator_value.len() > 0 {
                operator_value = Some(raw_operator_value.parse::<usize>().unwrap());
            } else {
                operator_value = None;
            }

            let step = Step {
                instruction,
                operator,
                operator_value,
            };

            steps.push(step);
        }

        steps
    }

    fn part_one(steps: &mut Self::ParsedInput) -> String {
        let sum = steps
            .iter()
            .map(|step| hash(&step.raw_string()))
            .sum::<u32>();

        sum.to_string()
    }

    fn part_two(steps: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

pub struct Step {
    instruction: String,
    operator: char,
    operator_value: Option<usize>,
}
impl Step {
    fn raw_string(&self) -> String {
        let mut raw_string = self.instruction.clone();

        raw_string.push(self.operator);

        if let Some(value) = self.operator_value {
            raw_string.push_str(&value.to_string());
        }

        raw_string
    }
}

fn hash(string: &String) -> u32 {
    let mut current_value = 0;

    for char in string.chars() {
        current_value += char as u32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_check_1() {
        assert_eq!(hash(&"HASH".to_string()), 52)
    }
    #[test]
    fn hash_check_2() {
        assert_eq!(hash(&"rn=1".to_string()), 30)
    }

    #[test]
    fn check_day15_part1_case1() {
        assert_eq!(
            Day15::solve_part_one("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            "1320".to_string()
        )
    }

    #[test]
    fn check_day15_part2_case1() {
        assert_eq!(
            Day15::solve_part_two("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            "0".to_string()
        )
    }

    #[test]
    fn check_day15_both_case1() {
        assert_eq!(Day15::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
