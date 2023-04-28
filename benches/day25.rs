use advent_of_code_template::{day25::Day25, Solution};
fn parsing() {
    Day25::parse_input(include_str!("../inputs/25"));
}

fn part_one() {
    Day25::solve_part_one(include_str!("../inputs/25"));
}

fn part_two() {
    Day25::solve_part_two(include_str!("../inputs/25"));
}

iai::main!(parsing, part_one, part_two,);
