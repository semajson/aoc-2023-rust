use advent_of_code_template::{day02::Day02, Solution};
fn parsing() {
    Day02::parse_input(include_str!("../inputs/2"));
}

fn part_one() {
    Day02::solve_part_one(include_str!("../inputs/2"));
}

fn part_two() {
    Day02::solve_part_two(include_str!("../inputs/2"));
}

iai::main!(parsing, part_one, part_two,);
