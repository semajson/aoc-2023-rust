use crate::Solution;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug)]

pub struct Rule {
    dest_start: isize,
    src_start: isize,
    range_len: isize,
}
impl Rule {
    pub fn new(raw: &str) -> Rule {
        let re = Regex::new(r"(?<dest_start>\d+) (?<src_start>\d+) (?<range_len>\d+)").unwrap();
        let Some(cap) = re.captures(raw) else {
            panic!("No match")
        };

        fn get_group_from_cap(group: &str, cap: &regex::Captures) -> isize {
            cap.name(group).unwrap().as_str().parse::<isize>().unwrap()
        }

        let dest_start = get_group_from_cap("dest_start", &cap);
        let src_start = get_group_from_cap("src_start", &cap);
        let range_len = get_group_from_cap("range_len", &cap);

        Rule {
            dest_start,
            src_start,
            range_len,
        }
    }
    pub fn is_num_in_rule(&self, num: isize) -> bool {
        (num - self.src_start >= 0) && ((num - self.src_start) < self.range_len)
    }
    pub fn output(&self, num: isize) -> isize {
        self.dest_start + num - self.src_start
    }
}

pub struct Map(Vec<Rule>);
impl Map {
    pub fn new(raw: &str) -> Map {
        Map(raw
            .split('\n')
            .filter(|&x| !x.is_empty())
            .map(Rule::new)
            .collect::<Vec<Rule>>())
    }
    pub fn get_output(&self, input: isize) -> isize {
        for rule in self.0.iter() {
            if rule.is_num_in_rule(input) {
                return rule.output(input);
            }
        }

        // No rules apply, just return the input
        input
    }
}

pub struct Seeds(Vec<isize>);

#[derive(Clone, Debug, PartialEq, Hash, Eq)]

pub struct Day05;

fn get_final_value(mut input: isize, maps: &HashMap<String, (String, Map)>) -> isize {
    let mut src = "seed";

    while src != "location" {
        let (next_src, map) = maps.get(src).unwrap();
        input = map.get_output(input);
        src = next_src;
    }
    input
}

impl Solution for Day05 {
    type ParsedInput = (Vec<isize>, HashMap<String, (String, Map)>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let re = Regex::new(r"seeds:(?<seeds>( [0-9]+)+)").unwrap();
        let Some(cap) = re.captures(input_lines) else {
            panic!("Didn't find the seeds")
        };
        let seeds = cap
            .name("seeds")
            .unwrap()
            .as_str()
            .split_whitespace()
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        let re = Regex::new(
            r"(?<source>[a-z]+)-to-(?<destination>[a-z]+) map:
(?<map>([0-9]+ [0-9]+ [0-9]+
?)+)",
        )
        .unwrap();

        let mut maps = HashMap::new();

        for cap in re.captures_iter(input_lines) {
            let src = cap.name("source").unwrap().as_str().to_string();
            let dest = cap.name("destination").unwrap().as_str().to_string();
            let map = Map::new(cap.name("map").unwrap().as_str());

            maps.insert(src, (dest, map));
        }

        (seeds, maps)
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let mut src = "seed";
        let (inputs, maps) = _parsed_input;
        let mut inputs = inputs.clone();

        while src != "location" {
            let (next_src, map) = maps.get(src).unwrap();
            inputs = inputs.into_iter().map(|x| map.get_output(x)).collect();
            src = next_src;
        }
        inputs.iter().min().unwrap().to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let (seed_ranges, maps) = _parsed_input;

        let mut min = 100000000000000000;
        for i in 0..(seed_ranges.len() / 2) {
            let start = seed_ranges[i * 2];
            let range = seed_ranges[i * 2 + 1];

            for j in 0..range {
                let output = get_final_value(start + j, maps);
                if output < min {
                    min = output;
                }
            }
        }

        min.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(
            Day05::solve_part_one(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            "35".to_string()
        )
    }

    #[test]
    fn check_day05_part2_case1() {
        assert_eq!(
            Day05::solve_part_two(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            "46".to_string()
        )
    }

    #[test]
    fn check_day05_both_case1() {
        assert_eq!(
            Day05::solve(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
                false
            ),
            ("35".to_string(), "46".to_string())
        )
    }
}
