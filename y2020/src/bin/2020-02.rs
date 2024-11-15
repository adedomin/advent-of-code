use std::io;

use aoc_shared::{fold_decimal_from, read_input};
use regex::bytes::Regex;

type Output = Vec<PasswordPolicy>;
type Solved = i32;

#[derive(Debug)]
struct PasswordPolicy {
    min: i32,
    max: i32,
    chr: u8,
    pas: Vec<u8>,
}

fn parse_input(input: &[u8]) -> Output {
    let mut ret = vec![];
    let re =
        Regex::new(r##"(?m)^(?<min>[[:digit:]]+)-(?<max>[[:digit:]]+) (?<chr>.): (?<pas>.*)$"##)
            .unwrap();
    re.captures_iter(input).for_each(|matcher| {
        let min = fold_decimal_from(matcher.name("min").unwrap().as_bytes());
        let max = fold_decimal_from(matcher.name("max").unwrap().as_bytes());
        let chr = matcher.name("chr").unwrap().as_bytes()[0];
        let pas = matcher.name("pas").unwrap().as_bytes().to_owned();
        ret.push(PasswordPolicy { min, max, chr, pas });
    });
    ret
}

fn part1_sol(policy: &Output) -> Solved {
    policy.iter().fold(0, |acc, pol| {
        let count = pol.pas.iter().filter(|&&chr| chr == pol.chr).count() as i32;
        if pol.min <= count && pol.max >= count {
            acc + 1
        } else {
            acc
        }
    })
}

fn part2_sol(policy: &Output) -> Solved {
    policy.iter().fold(0, |acc, pol| {
        let pmin = pol.pas.get((pol.min - 1) as usize);
        let pmax = pol.pas.get((pol.max - 1) as usize);

        match (pmin, pmax) {
            (Some(min), Some(max)) => {
                if (*min == pol.chr) ^ (*max == pol.chr) {
                    acc + 1
                } else {
                    acc
                }
            }
            _ => {
                eprintln!("WARN: Should not happen?");
                acc
            }
        }
    })
}

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
