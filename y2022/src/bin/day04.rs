#![feature(iter_array_chunks)]

use std::io;
use y2022::{fold_decimal_from, read_input, AoCTokenizer, Token};

type ElfPairs = ((i64, i64, i64), (i64, i64, i64));

fn parse_input(input: Vec<u8>) -> Vec<ElfPairs> {
    let tokenizer = AoCTokenizer::new(&input);
    tokenizer
        .fold(
            (Vec::new(), [0i64; 4], 0),
            |(mut acc, mut nums, idx), token| match token {
                Token::Something(number) => {
                    nums[idx] = fold_decimal_from(number);
                    (acc, nums, idx + 1)
                }
                Token::Newline | Token::End if idx == 4 => {
                    let [a, b, c, d] = nums;
                    acc.push(((a, b, a.abs_diff(b) as i64), (c, d, c.abs_diff(d) as i64)));
                    (acc, [0; 4], 0)
                }
                _ => (acc, nums, idx),
            },
        )
        .0
}

fn part1_sol(input: &[ElfPairs]) -> usize {
    input
        .iter()
        .filter(|(elf1, elf2)| {
            if elf1 == elf2 {
                true
            } else if elf1.2 == elf2.2 {
                false
            } else if elf1.2 < elf2.2 && (elf2.0 <= elf1.0 && elf2.1 >= elf1.1) {
                true
            } else {
                elf1.0 <= elf2.0 && elf1.1 >= elf2.1
            }
        })
        .count()
}

fn part2_sol(input: &[ElfPairs]) -> usize {
    input
        .iter()
        .filter(|(elf1, elf2)| (elf1.0 <= elf2.0 && elf2.0 <= elf1.1) || (elf2.0 <= elf1.0 && elf1.0 <= elf2.1))
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
