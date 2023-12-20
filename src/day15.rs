use crate::Solution;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Day15;

impl Solution for Day15 {
    type ParsedInput = Vec<Step>;

    fn parse_input(input: &str) -> Self::ParsedInput {
        let re = Regex::new(r"(?<instruction>[a-zA-Z]+)(?<operator>[-=])(?<focal_length>[0-9]*)")
            .unwrap();

        let mut steps = vec![];
        for cap in re.captures_iter(input) {
            let instruction = cap.name("instruction").unwrap().as_str().to_string();

            let operator = cap.name("operator").unwrap().as_str();
            let operator = operator.chars().collect::<Vec<char>>()[0];

            let raw_focal_length = cap.name("focal_length").unwrap().as_str();

            let focal_length: Option<usize> = if raw_focal_length.is_empty() {
                None
            } else {
                Some(raw_focal_length.parse::<usize>().unwrap())
            };

            let step = Step {
                label: instruction,
                operator,
                focal_length,
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
        let mut boxes: Vec<Vec<Lense>> = vec![vec![]; 256];
        for step in steps {
            boxes = step.do_operation(boxes);
        }
        let mut power = 0;

        for (box_num, my_box) in boxes.iter().enumerate() {
            for (lense_index, lense) in my_box.iter().enumerate() {
                power += (1 + box_num) * (1 + lense_index) * lense.focal_length;
            }
        }

        power.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Lense {
    label: String,
    focal_length: usize,
}

pub struct Step {
    label: String,
    operator: char,
    focal_length: Option<usize>,
}
impl Step {
    fn raw_string(&self) -> String {
        let mut raw_string = self.label.clone();

        raw_string.push(self.operator);

        if let Some(value) = self.focal_length {
            raw_string.push_str(&value.to_string());
        }

        raw_string
    }

    fn do_operation(&self, boxes: Vec<Vec<Lense>>) -> Vec<Vec<Lense>> {
        match self.operator {
            '-' => self.remove_lense(boxes),
            '=' => self.add_lense(boxes),
            _ => panic!("Unexpected operation {:?}", self.operator),
        }
    }
    fn add_lense(&self, mut boxes: Vec<Vec<Lense>>) -> Vec<Vec<Lense>> {
        let box_num = hash(&self.label) as usize;

        let new_lense = Lense {
            label: self.label.clone(),
            focal_length: self.focal_length.unwrap(),
        };

        let current_lense_index = boxes[box_num]
            .iter()
            .position(|lense| lense.label == new_lense.label);

        if let Some(lense_index) = current_lense_index {
            boxes[box_num][lense_index] = new_lense;
        } else {
            boxes[box_num].push(new_lense);
        };

        boxes
    }
    fn remove_lense(&self, mut boxes: Vec<Vec<Lense>>) -> Vec<Vec<Lense>> {
        let box_num = hash(&self.label) as usize;

        boxes[box_num] = boxes[box_num]
            .clone()
            .into_iter()
            .filter(|x| x.label != self.label)
            .collect::<Vec<Lense>>();
        boxes
    }
}

fn hash(string: &str) -> u32 {
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
            "145".to_string()
        )
    }

    #[test]
    fn check_day15_both_case1() {
        assert_eq!(Day15::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
