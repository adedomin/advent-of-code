use aoc_shared::{read_input, AoCTokenizer, Token};
use std::io;

type Output = Vec<something>;
type Solved = i64;

fn parse_input(input: Vec<u8>) -> Output {
    let tokenizer = AoCTokenizer::new(&input);
}

fn part1_sol(input: Output) -> Solved {}

fn part2_sol(input: Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
