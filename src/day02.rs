use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day02;

#[derive(Clone, Debug)]
struct Round {
    red: i32,
    blue: i32,
    green: i32,
}
impl Round {
    pub fn possible(&self, round: &Round) -> bool {
        if self.red > round.red || self.blue > round.blue || self.green > round.green {
            false
        } else {
            true
        }
    }
    pub fn fewest(&self, round: &mut Round) -> () {
        if self.red > round.red {
            round.red = self.red
        }
        if self.green > round.green {
            round.green = self.green
        }
        if self.blue > round.blue {
            round.blue = self.blue
        }
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    id: i32,
    rounds: Vec<Round>,
}
impl Game {
    pub fn new(line: &str) -> Game {
        let line = line.replace("Game ", "");
        let line = line.split(": ").collect::<Vec<&str>>();
        let id = line[0].parse::<i32>().unwrap();
        let raw_rounds = line[1].split("; ").collect::<Vec<&str>>();
        let mut rounds = Vec::new();
        for raw_round in raw_rounds {
            let mut round = Round {
                red: 0,
                blue: 0,
                green: 0,
            };
            let colours_raw = raw_round.split(", ").collect::<Vec<&str>>();
            for colour_raw in colours_raw {
                let colour = colour_raw.split(" ").collect::<Vec<&str>>();
                let num = colour[0].parse::<i32>().unwrap();
                match colour[1] {
                    "red" => round.red = num,
                    "green" => round.green = num,
                    "blue" => round.blue = num,
                    _ => panic!("Unexpected input {}", colour[1]),
                }
            }
            rounds.push(round)
        }
        Game { id, rounds }
    }
    fn round_possible(&self, round_to_check: &Round) -> bool {
        for round in self.rounds.iter() {
            if !round.possible(round_to_check) {
                return false;
            }
        }
        true
    }
    fn fewest_cubes(&self) -> Round {
        let mut fewest_cube = Round {
            red: 0,
            blue: 0,
            green: 0,
        };
        for round in self.rounds.iter() {
            round.fewest(&mut fewest_cube);
        }
        fewest_cube
    }
    fn power(&self) -> i32 {
        let fewest_cube = self.fewest_cubes();
        fewest_cube.red * fewest_cube.blue * fewest_cube.green
    }
}

impl Solution for Day02 {
    type ParsedInput = Vec<Game>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let input_lines = input_lines.to_string();
        let input_lines = input_lines
            .lines()
            .map(String::from)
            .collect::<Vec<String>>();

        let mut games = Vec::new();
        for line in input_lines {
            games.push(Game::new(&line));
        }
        games
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let round_to_check = Round {
            red: 12,
            green: 13,
            blue: 14,
        };
        let mut total = 0;
        for game in _parsed_input {
            if game.round_possible(&round_to_check) {
                total += game.id;
            }
        }
        // TODO: implement part one
        total.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let mut sum = 0;
        for game in _parsed_input {
            sum += game.power();
        }
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day02_part1_case1() {
        assert_eq!(Day02::solve_part_one(""), "2061".to_string())
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(Day02::solve_part_two(""), "72596".to_string())
    }

    #[test]
    fn check_day02_both_case1() {
        assert_eq!(
            Day02::solve("", false),
            ("2061".to_string(), "72596".to_string())
        )
    }
}
