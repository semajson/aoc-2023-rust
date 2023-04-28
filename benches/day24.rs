use advent_of_code_template::{day24::Day24, Solution};
fn parsing() {
    Day24::parse_input(include_str!("../inputs/24"));
}

fn part_one() {
    Day24::solve_part_one(include_str!("../inputs/24"));
}

fn part_two() {
    Day24::solve_part_two(include_str!("../inputs/24"));
}

iai::main!(parsing, part_one, part_two,);
