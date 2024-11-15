use aoc_shared::read_input_to_string;
use std::io;

struct BoardingPass(pub i32);

impl From<&str> for BoardingPass {
    fn from(value: &str) -> Self {
        BoardingPass(value.as_bytes().iter().fold(0, |acc, &chr| {
            // we pretend the string is valid and ignore F/L altogether...
            (acc << 1) + (chr == b'B' || chr == b'R') as i32
        }))
    }
}

type Output = Vec<BoardingPass>;
type Solved = i32;

fn parse_input(input: &str) -> Output {
    input
        .split_ascii_whitespace()
        .map(|s| s.into())
        .collect::<Vec<_>>()
}

fn part1_sol(input: &Output) -> Solved {
    input
        .iter()
        .map(|pass| pass.0)
        .max()
        .expect("Need at least one boarding pass")
}

fn part2_sol(input: &Output) -> Solved {
    let (sum, min, max) = input
        .iter()
        .fold((0, i32::MAX, i32::MIN), |(sum, min, max), pass| {
            let min = std::cmp::min(pass.0, min);
            let max = std::cmp::max(pass.0, max);
            (sum + pass.0, min, max)
        });
    assert!(min != i32::MAX && max != i32::MIN);
    // my seat
    ((max + min) * (max - min + 1)) / 2 - sum
}

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
