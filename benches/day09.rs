use advent_of_code_template::{day09::Day09, Solution};
fn parsing() {
    Day09::parse_input(include_str!("../inputs/9"));
}

fn part_one() {
    Day09::solve_part_one(include_str!("../inputs/9"));
}

fn part_two() {
    Day09::solve_part_two(include_str!("../inputs/9"));
}

iai::main!(parsing, part_one, part_two,);
