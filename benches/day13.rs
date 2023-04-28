use advent_of_code_template::{day13::Day13, Solution};
fn parsing() {
    Day13::parse_input(include_str!("../inputs/13"));
}

fn part_one() {
    Day13::solve_part_one(include_str!("../inputs/13"));
}

fn part_two() {
    Day13::solve_part_two(include_str!("../inputs/13"));
}

iai::main!(parsing, part_one, part_two,);
