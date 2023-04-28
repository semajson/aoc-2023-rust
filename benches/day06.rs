use advent_of_code_template::{day06::Day06, Solution};
fn parsing() {
    Day06::parse_input(include_str!("../inputs/6"));
}

fn part_one() {
    Day06::solve_part_one(include_str!("../inputs/6"));
}

fn part_two() {
    Day06::solve_part_two(include_str!("../inputs/6"));
}

iai::main!(parsing, part_one, part_two,);
