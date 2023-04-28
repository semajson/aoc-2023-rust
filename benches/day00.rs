use advent_of_code_template::{day00::Day00, Solution};
fn parsing() {
    Day00::parse_input(include_str!("../inputs/0"));
}

fn part_one() {
    Day00::solve_part_one(include_str!("../inputs/0"));
}

fn part_two() {
    Day00::solve_part_two(include_str!("../inputs/0"));
}

iai::main!(parsing, part_one, part_two,);
