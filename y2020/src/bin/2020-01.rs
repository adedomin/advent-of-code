use aoc_shared::{destructure_or_none, fold_decimal_from, read_input, Token, Tokenize};
use itertools::Itertools;
use std::{collections::HashSet, io};

type Output = Vec<i32>;
type Solved = i32;

fn parse_input(input: &[u8]) -> Output {
    input
        .tokenize()
        .flat_map(|t| destructure_or_none!(Token::Something|word| = t))
        .map(fold_decimal_from)
        .collect::<Vec<_>>()
}

const SUM_FIND: i32 = 2020;

fn part1_sol(input: &Output, sums: &HashSet<i32>) -> Solved {
    for &val in input {
        if sums.contains(&(SUM_FIND - val)) && SUM_FIND - val != val {
            return val * (SUM_FIND - val);
        }
    }
    panic!("NO SAT");
}

fn part2_sol(input: Output, sums: &HashSet<i32>) -> Solved {
    for (x, y) in input.into_iter().tuple_combinations() {
        if sums.contains(&(SUM_FIND - (x + y))) {
            return x * y * (SUM_FIND - (x + y));
        }
    }
    panic!("NO SAT (p2)")
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let sums = parsed_input.iter().copied().collect::<HashSet<i32>>();
    let part1 = part1_sol(&parsed_input, &sums);
    let part2 = part2_sol(parsed_input, &sums);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
