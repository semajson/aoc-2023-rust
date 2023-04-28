use advent_of_code_template::{day16::Day16, Solution};
fn parsing() {
    Day16::parse_input(include_str!("../inputs/16"));
}

fn part_one() {
    Day16::solve_part_one(include_str!("../inputs/16"));
}

fn part_two() {
    Day16::solve_part_two(include_str!("../inputs/16"));
}

iai::main!(parsing, part_one, part_two,);
