#![feature(iter_array_chunks)]

use std::io;
use y2022::{destructure_or_none, fold_decimal_from, read_input, AoCTokenizer, Token};

type ElfPairs = ((i64, i64), (i64, i64));

fn parse_input(input: Vec<u8>) -> Vec<ElfPairs> {
    let tokenizer = AoCTokenizer::new(&input);
    tokenizer
        .flat_map(|token| destructure_or_none!(Token::Something, token))
        .map(fold_decimal_from)
        .array_chunks()
        .map(|[a1, a2, b1, b2]| ((a1, a2), (b1, b2)))
        .collect()
}

fn part1_sol(input: &[ElfPairs]) -> usize {
    input
        .iter()
        .filter(|(elf1, elf2)| {
            (elf1.0 <= elf2.0 && elf1.1 >= elf2.1) || (elf2.0 <= elf1.0 && elf2.1 >= elf1.1)
        })
        .count()
}

fn part2_sol(input: &[ElfPairs]) -> usize {
    input
        .iter()
        .filter(|(elf1, elf2)| {
            (elf1.0 <= elf2.0 && elf2.0 <= elf1.1) || (elf2.0 <= elf1.0 && elf1.0 <= elf2.1)
        })
        .count()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
