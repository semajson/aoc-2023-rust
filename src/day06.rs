use crate::Solution;
use regex::Regex;

pub struct Race {
    time: isize,
    record: isize,
}
impl Race {
    fn new((time, record): (isize, isize)) -> Race {
        Race { time, record }
    }
    fn num_ways_win(&self) -> isize {
        let mut num_ways_win = 0;
        for wait in 0..=self.time {
            let remaining_time = self.time - wait;
            let speed = wait;

            if speed * remaining_time > self.record {
                num_ways_win += 1;
            }
        }
        num_ways_win
    }
}

#[derive(Clone, Debug)]
pub struct Day06;

fn parse_1(input_lines: &str) -> Vec<Race> {
    fn get_numbers_after_word(word: &str, raw: &str) -> Vec<isize> {
        let re = Regex::new(&(word.to_string() + r"\s*(?<numbers>\d+(?:\s+\d+)+)")).unwrap();
        let Some(cap) = re.captures(raw) else {
            panic!("No match")
        };

        cap.name("numbers")
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>()
    }
    let times = get_numbers_after_word("Time:", input_lines);
    let distances = get_numbers_after_word("Distance:", input_lines);

    times
        .into_iter()
        .zip(distances)
        .map(Race::new)
        .collect::<Vec<Race>>()
}

fn parse_2(input_lines: &str) -> Vec<Race> {
    fn get_number_after_word(word: &str, raw: &str) -> Vec<isize> {
        let re = Regex::new(&(word.to_string() + r"\s*(?<number>\d+(?:\s+\d+)+)")).unwrap();
        let Some(cap) = re.captures(raw) else {
            panic!("No match")
        };

        let number = cap
            .name("number")
            .unwrap()
            .as_str()
            .split_whitespace()
            .collect::<Vec<&str>>();

        vec![number.join("").parse::<isize>().unwrap()]
    }
    let times = get_number_after_word("Time:", input_lines);
    let distances = get_number_after_word("Distance:", input_lines);

    times
        .into_iter()
        .zip(distances)
        .map(Race::new)
        .collect::<Vec<Race>>()
}

impl Solution for Day06 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.to_string()
    }

    fn part_one(input_lines: &mut Self::ParsedInput) -> String {
        let races = parse_1(input_lines);

        let mut output = 1;

        for race in races {
            output *= race.num_ways_win();
        }
        output.to_string()
    }

    fn part_two(input_lines: &mut Self::ParsedInput) -> String {
        let races = parse_2(input_lines);

        let mut output = 1;

        for race in races {
            output *= race.num_ways_win();
        }
        output.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day06_part1_case1() {
        assert_eq!(
            Day06::solve_part_one(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            "288".to_string()
        )
    }

    #[test]
    fn check_day06_part2_case1() {
        assert_eq!(
            Day06::solve_part_two(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            "71503".to_string()
        )
    }

    #[test]
    fn check_day06_both_case1() {
        assert_eq!(
            Day06::solve(
                "Time:      7  15   30
    Distance:  9  40  200",
                false
            ),
            ("288".to_string(), "71503".to_string())
        )
    }
}
