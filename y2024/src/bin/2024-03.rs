use std::io;

use aoc_shared::{fold_decimal_from, read_input};
use regex::bytes::{Captures, Regex};

type Int = u64;

fn solve(input: &[u8]) -> (Int, Int) {
    let extract_num =
        |m: &Captures, name| fold_decimal_from::<Int>(m.name(name).unwrap().as_bytes());
    let re = Regex::new(
        r##"(?x)
         (?<do>do\(\))
        |(?<dont>don't\(\))
        |(?:mul\((?<n1>[[:digit:]]{1,3}),(?<n2>[[:digit:]]{1,3})\))
    "##,
    )
    .unwrap();
    let (p1, p2, _) = re
        .captures_iter(input)
        .fold((0, 0, true), |(p1, p2, enabled), m| {
            if m.name("do").is_some() {
                (p1, p2, true)
            } else if m.name("dont").is_some() {
                (p1, p2, false)
            } else {
                let n = extract_num(&m, "n1") * extract_num(&m, "n2");
                (p1 + n, p2 + n * enabled as Int, enabled)
            }
        });
    (p1, p2)
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (part1, part2) = solve(&input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
