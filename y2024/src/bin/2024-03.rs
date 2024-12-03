use std::io;

use aoc_shared::read_input_to_string;
use regex::{Captures, Regex};

type Output = i32;

fn extract_num(m: &Captures, name: &str) -> Output {
    m.name(name).unwrap().as_str().parse().unwrap()
}

fn parse_solve1(input: &str) -> (Output, Output) {
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
                    let n = extract_num(&matcher, "n1") * extract_num(&matcher, "n2");
                    (acc + n, acc2 + n * enabled as Output, enabled)
                }
            });
    (p1, p2)
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let (part1, part2) = parse_solve1(&input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
