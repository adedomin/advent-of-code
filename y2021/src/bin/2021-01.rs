#![feature(array_windows)]
use std::io;

use aoc_shared::read_input_to_string;

/// Where N - 1 is the window size
pub fn solve<const N: usize>(input: &[u64]) -> u64 {
    input
        .array_windows::<N>()
        .filter(|&w| w[0] < w[N - 1])
        .count() as u64
}

pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

pub fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let input = input_generator(&input);
    println!("Part1 {}, Part2 {}", solve::<2>(&input), solve::<4>(&input));
    Ok(())
}
