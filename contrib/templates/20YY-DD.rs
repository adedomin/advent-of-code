use aoc_shared::read_input_to_string;
use std::io;

type Int = i32;
type Output = Vec<Int>;

fn parse_input(input: &str) -> Output {
    input.do_something()
}

fn part1_sol(input: &Output) -> Int {}

fn part2_sol(input: &Output) -> Int {}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
