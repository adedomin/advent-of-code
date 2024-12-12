use aoc_shared::{fold_decimal_from, read_input_to_string};
use std::{io, iter::successors};

type Int = i32;
type Output = Vec<Int>;

fn parse_input(input: &str) -> Output {
    input
        .split_ascii_whitespace()
        .map(|num| fold_decimal_from(num.as_bytes()))
        .collect::<Vec<_>>()
}

fn part1_sol(input: &Output) -> Int {
    input.iter().map(|num| num / 3 - 2).sum()
}

fn calc_fuel(num: Int) -> Option<Int> {
    let ret = num / 3 - 2;
    if ret < 1 {
        None
    } else {
        Some(ret)
    }
}

fn part2_sol(input: Output) -> Int {
    input
        .into_iter()
        .map(|module| successors(calc_fuel(module), |&value| calc_fuel(value)).sum::<Int>())
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
