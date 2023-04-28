use advent_of_code_template::{day22::Day22, Solution};
fn parsing() {
    Day22::parse_input(include_str!("../inputs/22"));
}

fn part_one() {
    Day22::solve_part_one(include_str!("../inputs/22"));
}

fn part_two() {
    Day22::solve_part_two(include_str!("../inputs/22"));
}

iai::main!(parsing, part_one, part_two,);
