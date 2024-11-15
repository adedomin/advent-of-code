use std::io;

use aoc_shared::read_input;
use regex::bytes::Regex;

type Output = Vec<!>;
type Solved = i64;

fn parse_input(input: &[u8]) -> Output {
    let mut out = vec![];
    let re = Regex::new(r##"-- (?<R>REGEX) (?<H>HERE) --"##).unwrap();
    re.captures_iter(input).for_each(|matcher| {
        let r = fold_decimal_from(matcher.name("R").unwrap().as_bytes());
        let h = fold_decimal_from(matcher.name("H").unwrap().as_bytes());
        out.push((r, h));
    });
    out
}

fn part1_sol(input: &Output) -> Solved {}

fn part2_sol(input: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
