use advent_of_code_template::{day01::Day01, Solution};
fn parsing() {
    Day01::parse_input(include_str!("../inputs/1"));
}

fn part_one() {
    Day01::solve_part_one(include_str!("../inputs/1"));
}

fn part_two() {
    Day01::solve_part_two(include_str!("../inputs/1"));
}

iai::main!(parsing, part_one, part_two,);
