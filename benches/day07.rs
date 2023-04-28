use advent_of_code_template::{day07::Day07, Solution};
fn parsing() {
    Day07::parse_input(include_str!("../inputs/7"));
}

fn part_one() {
    Day07::solve_part_one(include_str!("../inputs/7"));
}

fn part_two() {
    Day07::solve_part_two(include_str!("../inputs/7"));
}

iai::main!(parsing, part_one, part_two,);
