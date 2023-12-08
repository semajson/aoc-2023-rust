use crate::Solution;
use regex::Regex;

// Lowest to highest
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
enum HandType {
    Unknown,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn strength(card: &char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("Error"),
    }
}

#[derive(Clone, Debug)]
pub struct Hand {
    hand: Vec<char>,
    bid: isize,
    hand_type: HandType,
    fives: Vec<String>,
    fours: Vec<String>,
    threes: Vec<String>,
    pairs: Vec<String>,
    ones: Vec<String>,
    jokers: bool,
}
impl Hand {
    pub fn new(hand: &str, bid: &str) -> Hand {
        assert!(hand.len() == 5);
        let bid = bid.parse::<isize>().unwrap();
        let hand = hand.to_string();
        let hand: Vec<char> = hand.chars().collect();

        Hand {
            hand,
            bid,
            jokers: false,
            hand_type: HandType::Unknown,
            fives: vec![],
            fours: vec![],
            threes: vec![],
            pairs: vec![],
            ones: vec![],
        }
    }
    pub fn analyse(&mut self) {
        self.find_matches();
        self.set_type();
    }
    fn find_matches(&mut self) {
        // SOrt is very important here
        let mut sorted_hand = self.hand.clone();
        sorted_hand.sort_by(|a, b| a.cmp(&b));

        let mut current_match = vec![sorted_hand[0]];

        for i in 1..sorted_hand.len() {
            let next_char = sorted_hand[i];
            if current_match[0] == next_char {
                current_match.push(next_char)
            } else {
                self.add_match(current_match);
                current_match = vec![next_char];
            }
        }
        self.add_match(current_match);

        self.set_type();
    }

    fn add_match(&mut self, found_match: Vec<char>) {
        let found_match = found_match
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");

        match found_match.len() {
            1 => self.ones.push(found_match),
            2 => self.pairs.push(found_match),
            3 => self.threes.push(found_match),
            4 => self.fours.push(found_match),
            5 => self.fives.push(found_match),
            _ => panic!("Unexpected length"),
        }
    }

    fn set_type(&mut self) {
        if self.jokers {
            self.hand_type = self.get_type_jokers();
        } else {
            self.hand_type = self.get_type();
        }
    }
    fn get_type(&self) -> HandType {
        if self.fives.len() == 1 {
            HandType::FiveOfAKind
        } else if self.fours.len() == 1 {
            HandType::FourOfAKind
        } else if self.threes.len() == 1 && self.pairs.len() == 1 {
            HandType::FullHouse
        } else if self.threes.len() == 1 {
            HandType::ThreeOfAKind
        } else if self.pairs.len() == 2 {
            HandType::TwoPair
        } else if self.pairs.len() == 1 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
    fn get_type_jokers(&self) -> HandType {
        let num_jokers = self.hand.iter().filter(|&&x| x == 'J').count();

        if self.fives.len() == 1 {
            HandType::FiveOfAKind
        } else if self.fours.len() == 1 {
            match num_jokers {
                1 => HandType::FiveOfAKind,
                _ => HandType::FourOfAKind,
            }
        } else if self.threes.len() == 1 && self.pairs.len() == 1 {
            match num_jokers {
                0 => HandType::FullHouse,
                _ => HandType::FiveOfAKind,
            }
        } else if self.threes.len() == 1 {
            match num_jokers {
                1 => HandType::FourOfAKind,
                _ => HandType::ThreeOfAKind,
            }
        } else if self.pairs.len() == 2 {
            match num_jokers {
                2 => HandType::FourOfAKind,
                1 => HandType::FullHouse,
                _ => HandType::TwoPair,
            }
        } else if self.pairs.len() == 1 {
            match num_jokers {
                1 => HandType::ThreeOfAKind,
                _ => HandType::TwoPair,
            }
        } else {
            match num_jokers {
                1 => HandType::TwoPair,
                _ => HandType::HighCard,
            }
        }
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.hand == other.hand
    }
}
impl Eq for Hand {}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            (self.hand_type as isize).cmp(&(other.hand_type as isize))
        } else {
            // Are the same type
            // As the spec, iterate through the original string
            for i in 0..self.hand.len() {
                if self.hand[i] != other.hand[i] {
                    return strength(&self.hand[i]).cmp(&strength(&other.hand[i]));
                }
            }
            self.hand[0].cmp(&other.hand[0]) // todo make equal
        }
    }
}

#[derive(Clone, Debug)]
pub struct Day07;

impl Solution for Day07 {
    type ParsedInput = Vec<Hand>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let re = Regex::new(r"(?<hand>[0-9JKJTQA]+) (?<bid>[0-9]+)").unwrap();

        let mut hands = vec![];
        for cap in re.captures_iter(input_lines) {
            let hand = cap.name("hand").unwrap().as_str();
            let bid = cap.name("bid").unwrap().as_str();
            hands.push(Hand::new(hand, bid));
        }

        hands
    }

    fn part_one(hands: &mut Self::ParsedInput) -> String {
        for hand in hands.iter_mut() {
            // hand.order_by_matches();
            hand.analyse()
        }
        hands.sort();
        // TODO: implement part one

        let mut sum = 0;
        for (i, hand) in hands.iter_mut().enumerate() {
            sum += hand.bid * (i as isize + 1);
        }

        println!("test");
        sum.to_string()
    }

    fn part_two(hands: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day07_part1_case1() {
        assert_eq!(
            Day07::solve_part_one(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            "6440".to_string()
        )
    }

    #[test]
    fn check_day07_part1_case2() {
        assert_eq!(
            Day07::solve_part_one(
                "AAAA3 1
AAJJ2 1
AA992 1
999J2 1
999T2 1
AAAA2 1
AAAA2 1
AAA22 1"
            ),
            "5".to_string()
        )
    }

    #[test]
    fn check_day07_part2_case1() {
        assert_eq!(
            Day07::solve_part_two(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            "0".to_string()
        )
    }

    #[test]
    fn check_day07_both_case1() {
        assert_eq!(
            Day07::solve(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
                false
            ),
            ("6440".to_string(), "0".to_string())
        )
    }
}
