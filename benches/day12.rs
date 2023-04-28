use advent_of_code_template::{day12::Day12, Solution};
fn parsing() {
    Day12::parse_input(include_str!("../inputs/12"));
}

fn part_one() {
    Day12::solve_part_one(include_str!("../inputs/12"));
}

fn part_two() {
    Day12::solve_part_two(include_str!("../inputs/12"));
}

iai::main!(parsing, part_one, part_two,);
