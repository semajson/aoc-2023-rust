use advent_of_code_template::{day23::Day23, Solution};
fn parsing() {
    Day23::parse_input(include_str!("../inputs/23"));
}

fn part_one() {
    Day23::solve_part_one(include_str!("../inputs/23"));
}

fn part_two() {
    Day23::solve_part_two(include_str!("../inputs/23"));
}

iai::main!(parsing, part_one, part_two,);
