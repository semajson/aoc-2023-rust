use advent_of_code_template::{day10::Day10, Solution};
fn parsing() {
    Day10::parse_input(include_str!("../inputs/10"));
}

fn part_one() {
    Day10::solve_part_one(include_str!("../inputs/10"));
}

fn part_two() {
    Day10::solve_part_two(include_str!("../inputs/10"));
}

iai::main!(parsing, part_one, part_two,);
