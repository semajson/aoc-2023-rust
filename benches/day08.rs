use advent_of_code_template::{day08::Day08, Solution};
fn parsing() {
    Day08::parse_input(include_str!("../inputs/8"));
}

fn part_one() {
    Day08::solve_part_one(include_str!("../inputs/8"));
}

fn part_two() {
    Day08::solve_part_two(include_str!("../inputs/8"));
}

iai::main!(parsing, part_one, part_two,);
