use aoc_shared::{read_input_to_string, try_atoi};
use std::io;

type Output = (usize, usize);
type Solved = i64;

fn parse_input(input: &str) -> Output {
    let i = input
        .split_terminator(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
        .filter_map(|num| {
            if num.is_empty() {
                None
            } else {
                try_atoi::<_, 10>(num.as_bytes())
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(i.len(), 2);
    (i[0], i[1])
}

const START: i64 = 20_151_125;
const MULT: i64 = 252_533;
const MODULO: i64 = 33_554_393;

fn part1_sol((x, y): (usize, usize)) -> Solved {
    let position = (x + y - 2) * (x + y - 1) / 2 + y - 1;
    let mut last = START;
    for _ in 0..position {
        last = last * MULT % MODULO;
    }
    last
}

// fn part2_sol(input: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(parsed_input);
    // let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    // print!("Part2: {part2}");
    println!();
    Ok(())
}
