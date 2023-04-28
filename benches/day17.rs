use advent_of_code_template::{day17::Day17, Solution};
fn parsing() {
    Day17::parse_input(include_str!("../inputs/17"));
}

fn part_one() {
    Day17::solve_part_one(include_str!("../inputs/17"));
}

fn part_two() {
    Day17::solve_part_two(include_str!("../inputs/17"));
}

iai::main!(parsing, part_one, part_two,);
