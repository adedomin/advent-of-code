use aoc_shared::read_input_to_string;
use itertools::Itertools;
use std::io;

type Output = Vec<u64>;

fn parse_input(input: &str) -> Output {
    input
        .split("\n\n")
        .filter(|pat| !pat.is_empty())
        .map(|pattern| {
            pattern
                .as_bytes()
                .iter()
                .filter(|&&b| b == b'.' || b == b'#')
                .fold(0u64, |acc, &b| acc << 1 | (b == b'#') as u64)
        })
        .collect::<Vec<u64>>()
}

fn part1_sol(keys_and_locks: Output) -> usize {
    // we exploit the top and bottom rows differentiating the two and they *ALWAYS* self filter
    // we basically just ask if both lock and key occupy the same space, if not, then ANDing them is always 0.
    keys_and_locks
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| *a & *b == 0)
        .count()
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(parsed_input);
    println!("Part1: {part1}");
    Ok(())
}
