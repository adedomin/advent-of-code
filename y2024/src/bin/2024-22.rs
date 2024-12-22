use aoc_shared::{array_windows, fold_decimal_from, read_input_to_string};
use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashMap, FxHasher};
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

type P2HashMap = FxHashMap<(usize, [Int; 4]), Int>;

fn mix_and_prune(secret: Int, inter: Int) -> Int {
    (secret ^ inter) % PRUNE
}

fn calc_next(mut secret: Int) -> Int {
    for _ in 0..P1_LOOP {
        secret = mix_and_prune(secret, secret * 64);
        secret = mix_and_prune(secret, secret / 32);
        secret = mix_and_prune(secret, secret * 2048);
    }
    secret
}

fn get_seq(seqs: &mut P2HashMap, monkey: usize, mut secret: Int) {
    let mut secrets = Vec::with_capacity(P1_LOOP + 1);
    secrets.push(secret % 10);
    for _ in 0..P1_LOOP {
        secret = mix_and_prune(secret, secret * 64);
        secret = mix_and_prune(secret, secret / 32);
        secret = mix_and_prune(secret, secret * 2048);
        secrets.push(secret % 10);
    }
    for (key, val) in array_windows(&secrets)
        .map(|[a, b]| (b - a, *b))
        .tuple_windows()
        .map(|(a, b, c, d)| ((monkey, [a.0, b.0, c.0, d.0]), d.1))
    {
        // we can only take the first entry?
        seqs.entry(key)
            // .and_modify(|e| *e = std::cmp::max(*e, val))
            .or_insert(val);
    }
}

fn part1_sol(input: &Output) -> Int {
    input.iter().map(|&num| calc_next(num)).sum()
}

fn part2_sol(input: Output) -> Int {
    input
        .into_iter()
        .enumerate()
        .fold(P2HashMap::default(), |mut acc, (monkey, num)| {
            get_seq(&mut acc, monkey, num);
            acc
        })
        .into_iter()
        .fold(
            FxHashMap::<[Int; 4], Int>::default(),
            |mut acc, ((_, key), banana)| {
                *acc.entry(key).or_default() += banana;
                acc
            },
        )
        .into_values()
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
