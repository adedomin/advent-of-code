use aoc_shared::{fold_decimal_from, read_input_to_string};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{io, ops::Not};

type Int = i64;
type Output = Vec<Int>;

fn parse_input(input: &str) -> Output {
    input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|n| n.is_empty().not())
        .map(|num| fold_decimal_from(num.as_bytes()))
        .collect()
}

const P1_LOOP: usize = 2000;
const PRUNE: Int = 16777216;

fn mix_and_prune(secret: Int, inter: Int) -> Int {
    (secret ^ inter) % PRUNE
}

fn calc_next(mut secret: Int) -> Int {
    secret = mix_and_prune(secret, secret * 64);
    secret = mix_and_prune(secret, secret / 32);
    secret = mix_and_prune(secret, secret * 2048);
    secret
}

fn get_seq(seqs: &mut FxHashMap<u32, Int>, mut secret: Int) {
    let mut seen = FxHashSet::default();
    for (key, bananas) in std::iter::from_fn(|| {
        let r = Some(secret % 10);
        secret = calc_next(secret);
        r
    })
    .take(P1_LOOP)
    .tuple_windows()
    .map(|(a, b)| ((b as u8).wrapping_sub(a as u8) as u32, b))
    .tuple_windows()
    .map(|(a, b, c, d)| (a.0 << 24 | b.0 << 16 | c.0 << 8 | d.0, d.1))
    .filter(|(key, _)| seen.insert(*key))
    {
        *seqs.entry(key).or_default() += bananas;
    }
}

fn part1_sol(input: &Output) -> Int {
    input
        .iter()
        .map(|&num| {
            let mut num = num;
            for _ in 0..P1_LOOP {
                num = calc_next(num)
            }
            num
        })
        .sum()
}

fn part2_sol(input: Output) -> Int {
    input
        .into_iter()
        .fold(FxHashMap::default(), |mut acc, monkeynum| {
            get_seq(&mut acc, monkeynum);
            acc
        })
        .values()
        .copied()
        .max()
        .unwrap()
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
