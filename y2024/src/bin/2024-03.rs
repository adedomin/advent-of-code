use std::io;

use aoc_shared::{fold_decimal_from, read_input};
use regex::bytes::Regex;

type Output = i32;

fn parse_solve1(input: &[u8]) -> (Output, Output) {
    let re = Regex::new(
        r##"(?x)
         (?<do>do\(\))
        |(?<dont>don't\(\))
        |(?:mul\((?<n1>-?[0-9]+),(?<n2>-?[0-9]+)\))
    "##,
    )
    .unwrap();
    let (p1, p2, _) =
        re.captures_iter(input)
            .fold((0, 0, true), |(acc, acc2, enabled), matcher| {
                if matcher.name("do").is_some() {
                    (acc, acc2, true)
                } else if matcher.name("dont").is_some() {
                    (acc, acc2, false)
                } else {
                    let n1 = fold_decimal_from::<Output>(matcher.name("n1").unwrap().as_bytes());
                    let n2 = fold_decimal_from::<Output>(matcher.name("n2").unwrap().as_bytes());
                    let prod = n1 * n2;
                    (acc + prod, acc2 + prod * enabled as Output, enabled)
                }
            });
    (p1, p2)
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (part1, part2) = parse_solve1(&input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
