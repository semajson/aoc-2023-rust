use regex::Regex;
use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day18;

impl Solution for Day18 {
    type ParsedInput = Vec<Command>;

    fn parse_input(input: &str) -> Self::ParsedInput {
        let re = Regex::new(r"(?<direction>[RDLU]) (?<steps>[0-9]+) \((?<colour>#[\da-z]+)\)")
        .unwrap();

        let mut commands = vec![];
        for cap in re.captures_iter(input) {
            let direction = cap.name("direction").unwrap().as_str().chars().next().unwrap();
            let steps: usize = cap.name("steps").unwrap().as_str().parse().unwrap();
            let colour = cap.name("colour").unwrap().as_str().to_string();

            commands.push(Command {
                direction,
                steps,
                colour,
            });
        }

        commands
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part one
        println!("tst");
        0.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

pub struct Command {
    direction: char,
    steps: usize,
    colour: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day18_part1_case1() {
        assert_eq!(Day18::solve_part_one("R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"), "0".to_string())
    }

    #[test]
    fn check_day18_part2_case1() {
        assert_eq!(Day18::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day18_both_case1() {
        assert_eq!(Day18::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
