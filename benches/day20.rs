use advent_of_code_template::{day20::Day20, Solution};
fn parsing() {
    Day20::parse_input(include_str!("../inputs/20"));
}

fn part_one() {
    Day20::solve_part_one(include_str!("../inputs/20"));
}

fn part_two() {
    Day20::solve_part_two(include_str!("../inputs/20"));
}

iai::main!(parsing, part_one, part_two,);
