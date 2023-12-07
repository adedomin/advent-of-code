use aoc_shared::{destructure_or_none, fold_decimal, read_input, try_atoi, GroupTokenize, Token};
use std::{collections::HashSet, io};

type Output = Vec<(u64, u64)>;

fn parse_input(input: &[u8]) -> Output {
    let mut iter = input.group_tokens(Token::Newline);
    let times = iter
        .next()
        .unwrap()
        .iter()
        .flat_map(|word| destructure_or_none!(Token::Something|word| = word))
        .flat_map(|word| try_atoi::<u64, 10>(word))
        .collect::<Vec<u64>>();
    let records = iter
        .next()
        .unwrap()
        .iter()
        .flat_map(|word| destructure_or_none!(Token::Something|word| = word))
        .flat_map(|word| try_atoi::<u64, 10>(word))
        .collect::<Vec<u64>>();
    times
        .iter()
        .copied()
        .zip(records.iter().copied())
        .collect::<Vec<_>>()
}

/// problem: HELD * REM = MAX(DISTANCE)
///
/// constraint1: HELD + REM = CONSTRAINT
/// thus:         HELD = CONSTRAINT - REM
/// thus optimal: REM * (CONSTRAINT - REM) -> dx/dy -> CONSTRAINT - 2REM -> CONSTRAINT/2 = REM
fn solve(constraint: u64, record: u64) -> u64 {
    let mut rem = constraint / 2 + (constraint & 1);
    let mut held = constraint - rem;
    let mut set = HashSet::new();
    while rem * held > record {
        set.insert(rem);
        set.insert(held);
        rem -= 1;
        held += 1;
    }
    set.len() as u64
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let races = parse_input(&input);
    let part1 = races
        .iter()
        .map(|&(cons, rec)| solve(cons, rec))
        .product::<u64>();
    let (constraint, record) = races.iter().fold((0, 0), |(cons, rec), &(c, r)| {
        let mut b = itoa::Buffer::new();
        let ci = b.format(c);
        let mut b2 = itoa::Buffer::new();
        let ri = b2.format(r);
        (
            ci.as_bytes()
                .iter()
                .fold(cons, |acc, v| fold_decimal(acc, v)),
            ri.as_bytes()
                .iter()
                .fold(rec, |acc, v| fold_decimal(acc, v)),
        )
    });
    let part2 = solve(constraint, record);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
