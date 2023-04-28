use advent_of_code_template::{day05::Day05, Solution};
fn parsing() {
    Day05::parse_input(include_str!("../inputs/5"));
}

fn part_one() {
    Day05::solve_part_one(include_str!("../inputs/5"));
}

fn part_two() {
    Day05::solve_part_two(include_str!("../inputs/5"));
}

iai::main!(parsing, part_one, part_two,);
