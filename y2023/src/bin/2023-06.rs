use aoc_shared::{destructure_or_none, fold_decimal, read_input, try_atoi, GroupTokenize, Token};
use std::io;

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

/// problem: HELD * REM = DISTANCE
///
/// constraint1: HELD + REM = CONSTRAINT
/// constraint2: HELD * REM > RECORD
/// substitute:  HELD = CONSTRAINT - REM
/// replace to Quadratic EQN -> REM * (CONSTRAINT - REM) = record -> -REM^2 + CON*REM - record = 0
/// if fractional part is exact, (0.0), it's not > RECORD, so subtract - possible number combo
/// if fractional part of min rounds up (<=0.5), add +1 for edge-case in floats.
fn solve(constraint: u64, record: u64) -> u64 {
    // a is always -1
    let a = -1f64;
    let b = constraint as f64;
    let c = -(record as f64);
    let xmax = (-b - (b.powf(2f64) - 4f64 * a * c).sqrt()) / -2f64;
    let xmin = (-b + (b.powf(2f64) - 4f64 * a * c).sqrt()) / -2f64;
    let mut ans = (xmax - xmin).trunc() as u64;
    if xmax.fract() == 0.0 || xmin.fract() == 0.0 as f64 {
        ans -= 1;
    } else if xmin.fract() > 0.5 {
        ans += 1;
    }
    ans as u64
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
    // let part2 = 0;
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
