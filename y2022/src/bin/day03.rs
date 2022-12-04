#![feature(iter_array_chunks)]

use std::io;
use y2022::{destructure_or_none, read_input, AoCTokenizer, Token};

const PRI_LOW: u8 = b'a' - 1;
const PRI_HIGH: u8 = b'A' - 1;

fn convert_to_pri_num(chr: u8) -> u64 {
    if PRI_LOW < chr {
        (chr - PRI_LOW) as u64
    } else {
        (chr - PRI_HIGH + 26) as u64
    }
}

fn part1_algo(rucksack: &[u8]) -> u64 {
    let (ruck1, ruck2) = rucksack.split_at(rucksack.len() / 2);
    let found_dup = *ruck1
        .iter()
        .find(|chr| ruck2.iter().any(|chr2| chr == &chr2))
        .expect("At least one item has to be in each rucksack");
    convert_to_pri_num(found_dup)
}

fn part2_algo(rucksacks: &[&[u8]; 3]) -> u64 {
    let [ruck1, ruck2, ruck3] = rucksacks;
    let found_dup = *ruck1
        .iter()
        .find(|chr| ruck2.iter().any(|chr2| chr == &chr2) && ruck3.iter().any(|chr3| chr == &chr3))
        .expect("At least one item has to be in each elf group");
    convert_to_pri_num(found_dup)
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let tokenizer = AoCTokenizer::new(&input);
    let (part1, part2) = tokenizer
        .flat_map(|token| destructure_or_none!(Token::Something, token))
        .array_chunks()
        .fold((0u64, 0u64), |(p1_acc, p2_acc), elf_group: [&[u8]; 3]| {
            (
                p1_acc + elf_group.iter().map(|ruck| part1_algo(ruck)).sum::<u64>(),
                p2_acc + part2_algo(&elf_group),
            )
        });

    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
