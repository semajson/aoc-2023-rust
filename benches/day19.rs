use advent_of_code_template::{day19::Day19, Solution};
fn parsing() {
    Day19::parse_input(include_str!("../inputs/19"));
}

fn part_one() {
    Day19::solve_part_one(include_str!("../inputs/19"));
}

fn part_two() {
    Day19::solve_part_two(include_str!("../inputs/19"));
}

iai::main!(parsing, part_one, part_two,);
