use advent_of_code_template::{day11::Day11, Solution};
fn parsing() {
    Day11::parse_input(include_str!("../inputs/11"));
}

fn part_one() {
    Day11::solve_part_one(include_str!("../inputs/11"));
}

fn part_two() {
    Day11::solve_part_two(include_str!("../inputs/11"));
}

iai::main!(parsing, part_one, part_two,);
