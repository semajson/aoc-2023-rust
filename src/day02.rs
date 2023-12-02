use crate::Solution;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Day02;

#[derive(Clone, Debug)]
struct Cubes {
    red: i32,
    blue: i32,
    green: i32,
}
impl Cubes {
    pub fn new(line: &str) -> Cubes {
        let mut cubes = Cubes {
            red: 0,
            blue: 0,
            green: 0,
        };
        let re = Regex::new(r"(?<count>[0-9]+) (?<colour>[a-zA-Z]+)").unwrap();

        for cap in re.captures_iter(line) {
            let colour = cap.name("colour").unwrap().as_str();
            let count = cap.name("count").unwrap().as_str().parse::<i32>().unwrap();
            match colour {
                "red" => cubes.red = count,
                "green" => cubes.green = count,
                "blue" => cubes.blue = count,
                _ => panic!("Unexpected colour {}", colour),
            }
        }

        cubes
    }

    pub fn is_possible(&self, cubes_to_check: &Cubes) -> bool {
        self.red <= cubes_to_check.red
            && self.blue <= cubes_to_check.blue
            && self.green <= cubes_to_check.green
    }
    pub fn make_possible_with_fewest_cubes(&self, fewest_cubes: &mut Cubes) {
        if self.red > fewest_cubes.red {
            fewest_cubes.red = self.red
        }
        if self.green > fewest_cubes.green {
            fewest_cubes.green = self.green
        }
        if self.blue > fewest_cubes.blue {
            fewest_cubes.blue = self.blue
        }
    }
    pub fn power(&self) -> i32 {
        self.red * self.blue * self.green
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    id: i32,
    rounds: Vec<Cubes>,
}
impl Game {
    pub fn new(line: &str) -> Game {
        let re = Regex::new(r"Game (?<id>[0-9]+): (?<rounds>.+)").unwrap();
        let Some(caps) = re.captures(line) else {
            panic!("Didn't find game")
        };

        let id = caps["id"].parse::<i32>().unwrap();
        let raw_rounds = &caps["rounds"];
        let raw_rounds = raw_rounds.split("; ").collect::<Vec<&str>>();

        let mut rounds = Vec::new();
        for raw_round in raw_rounds {
            rounds.push(Cubes::new(raw_round))
        }
        Game { id, rounds }
    }

    fn cubes_is_possible(&self, cubes_to_check: &Cubes) -> bool {
        for cubes in self.rounds.iter() {
            if !cubes.is_possible(cubes_to_check) {
                return false;
            }
        }
        true
    }
    fn get_fewest_possible_cubes(&self) -> Cubes {
        let mut fewest_cubes = Cubes::new("");
        for cubes in self.rounds.iter() {
            cubes.make_possible_with_fewest_cubes(&mut fewest_cubes);
        }
        fewest_cubes
    }
    fn fewest_cubes_power(&self) -> i32 {
        let fewest_cubes = self.get_fewest_possible_cubes();
        fewest_cubes.power()
    }
}

impl Solution for Day02 {
    type ParsedInput = Vec<Game>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let input_lines = input_lines.to_string();
        input_lines.lines().map(Game::new).collect::<Vec<Game>>()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let cubes_to_check = Cubes {
            red: 12,
            green: 13,
            blue: 14,
        };
        let mut total = 0;
        for game in _parsed_input {
            if game.cubes_is_possible(&cubes_to_check) {
                total += game.id;
            }
        }
        total.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let mut sum = 0;
        for game in _parsed_input {
            sum += game.fewest_cubes_power();
        }
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day02_part1_case1() {
        assert_eq!(
            Day02::solve_part_one(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            "8".to_string()
        )
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(
            Day02::solve_part_two(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            "2286".to_string()
        )
    }

    #[test]
    fn check_day02_both_case1() {
        assert_eq!(
            Day02::solve(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                false
            ),
            ("8".to_string(), "2286".to_string())
        )
    }
}
