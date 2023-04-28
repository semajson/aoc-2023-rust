use advent_of_code_template::{day14::Day14, Solution};
fn parsing() {
    Day14::parse_input(include_str!("../inputs/14"));
}

fn part_one() {
    Day14::solve_part_one(include_str!("../inputs/14"));
}

fn part_two() {
    Day14::solve_part_two(include_str!("../inputs/14"));
}

iai::main!(parsing, part_one, part_two,);
