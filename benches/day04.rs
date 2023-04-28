use advent_of_code_template::{day04::Day04, Solution};
fn parsing() {
    Day04::parse_input(include_str!("../inputs/4"));
}

fn part_one() {
    Day04::solve_part_one(include_str!("../inputs/4"));
}

fn part_two() {
    Day04::solve_part_two(include_str!("../inputs/4"));
}

iai::main!(parsing, part_one, part_two,);
