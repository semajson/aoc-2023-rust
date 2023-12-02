use crate::Solution;

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
        let colours_raw = line.split(", ").collect::<Vec<&str>>();
        for colour_raw in colours_raw {
            let colour = colour_raw.split(' ').collect::<Vec<&str>>();
            let num = colour[0].parse::<i32>().unwrap();
            match colour[1] {
                "red" => cubes.red = num,
                "green" => cubes.green = num,
                "blue" => cubes.blue = num,
                _ => panic!("Unexpected input {}", colour[1]),
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
        let line = line.replace("Game ", "");
        let line = line.split(": ").collect::<Vec<&str>>();
        let id = line[0].parse::<i32>().unwrap();
        let raw_rounds = line[1].split("; ").collect::<Vec<&str>>();
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
        let mut fewest_cubes = Cubes {
            red: 0,
            blue: 0,
            green: 0,
        };
        for cubes in self.rounds.iter() {
            cubes.make_possible_with_fewest_cubes(&mut fewest_cubes);
        }
        fewest_cubes
    }
    fn power(&self) -> i32 {
        let fewest_cubes = self.get_fewest_possible_cubes();
        fewest_cubes.power()
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
