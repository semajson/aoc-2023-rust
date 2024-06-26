use crate::Solution;
use regex::Regex;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Day04;

pub struct Card {
    // number: usize,
    winning_numbers: HashSet<usize>,
    my_numbers: HashSet<usize>,
}
impl Card {
    pub fn get_my_winning_numbers(&self) -> HashSet<usize> {
        self.winning_numbers
            .intersection(&self.my_numbers)
            .cloned()
            .collect()
    }
    pub fn num_matches(&self) -> usize {
        self.get_my_winning_numbers().len()
    }
}

impl Solution for Day04 {
    type ParsedInput = Vec<Card>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let re = Regex::new(
            r"Card [ ]*(?<number>[0-9]+): (?<winning_numbers>[0-9 ]+) \| (?<my_numbers>[0-9 ]+)",
        )
        .unwrap();

        let mut cards = vec![];

        // TODO, remove get_hashset and go full regex to get this!
        fn get_hashset(raw: &str) -> HashSet<usize> {
            raw.split_whitespace()
                .filter(|&x| !x.is_empty())
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<HashSet<usize>>()
        }

        for cap in re.captures_iter(input_lines) {
            let _number = cap
                .name("number")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            let my_numbers = get_hashset(cap.name("my_numbers").unwrap().as_str());
            let winning_numbers = get_hashset(cap.name("winning_numbers").unwrap().as_str());

            cards.push(Card {
                my_numbers,
                winning_numbers,
            })
        }

        cards
    }

    fn part_one(cards: &mut Self::ParsedInput) -> String {
        let mut total_points = 0;
        for card in cards {
            let num_matches = card.num_matches();

            if num_matches > 0 {
                total_points += 2_i64.pow(num_matches as u32 - 1)
            }
        }
        total_points.to_string()
    }

    fn part_two(cards: &mut Self::ParsedInput) -> String {
        let mut copies = vec![1; cards.len()];
        for (i, card) in cards.iter().enumerate() {
            let num_matches = card.num_matches();
            for j in 1..=num_matches {
                assert!((i + j) < cards.len());
                copies[i + j] += copies[i];
            }
        }
        let sum: usize = copies.iter().sum();
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(
            Day04::solve_part_one(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            "13".to_string()
        )
    }

    #[test]
    fn check_day04_part1_case3() {
        assert_eq!(
            Day04::solve_part_one(
                "Card 1: 41 42 43 44 45 | 41 42 43 44 45  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"
            ),
            "18".to_string()
        )
    }

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(
            Day04::solve_part_two(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            "30".to_string()
        )
    }

    #[test]
    fn check_day04_both_case1() {
        assert_eq!(Day04::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
