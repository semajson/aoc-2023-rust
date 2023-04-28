use advent_of_code_template::{day03::Day03, Solution};
fn parsing() {
    Day03::parse_input(include_str!("../inputs/3"));
}

fn part_one() {
    Day03::solve_part_one(include_str!("../inputs/3"));
}

fn part_two() {
    Day03::solve_part_two(include_str!("../inputs/3"));
}

iai::main!(parsing, part_one, part_two,);
