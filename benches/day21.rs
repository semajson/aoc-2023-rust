use advent_of_code_template::{day21::Day21, Solution};
fn parsing() {
    Day21::parse_input(include_str!("../inputs/21"));
}

fn part_one() {
    Day21::solve_part_one(include_str!("../inputs/21"));
}

fn part_two() {
    Day21::solve_part_two(include_str!("../inputs/21"));
}

iai::main!(parsing, part_one, part_two,);
