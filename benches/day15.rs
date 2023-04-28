use advent_of_code_template::{day15::Day15, Solution};
fn parsing() {
    Day15::parse_input(include_str!("../inputs/15"));
}

fn part_one() {
    Day15::solve_part_one(include_str!("../inputs/15"));
}

fn part_two() {
    Day15::solve_part_two(include_str!("../inputs/15"));
}

iai::main!(parsing, part_one, part_two,);
