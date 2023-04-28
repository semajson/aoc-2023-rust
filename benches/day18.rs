use advent_of_code_template::{day18::Day18, Solution};
fn parsing() {
    Day18::parse_input(include_str!("../inputs/18"));
}

fn part_one() {
    Day18::solve_part_one(include_str!("../inputs/18"));
}

fn part_two() {
    Day18::solve_part_two(include_str!("../inputs/18"));
}

iai::main!(parsing, part_one, part_two,);
