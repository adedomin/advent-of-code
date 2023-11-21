use aoc_shared::{atoi, destructure_or_none, read_input, Token, Tokenize};
use itertools::Itertools;
use std::io;

fn parse_input(input: &[u8]) -> Vec<i64> {
    input
        .tokenize()
        .flat_map(|word| destructure_or_none!(Token::Something|word| = word))
        .map(|word| atoi::<i64, 10>(word))
        .collect::<Vec<i64>>()
}

fn solve_p1(max_amt: i64, containers: &[i64]) -> usize {
    containers
        .iter()
        .powerset()
        .filter(|vals| vals.iter().fold(0, |acc, &&val| acc + val) == max_amt)
        .count()
}

fn solve_p2(max_amt: i64, containers: &[i64]) -> usize {
    containers
        .iter()
        .powerset()
        .filter(|vals| vals.iter().fold(0, |acc, &&val| acc + val) == max_amt)
        .map(|vals| vals.len())
        .min_set()
        .len()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let part1 = solve_p1(150, &parsed_input);
    let part2 = solve_p2(150, &parsed_input);

    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::solve_p1;

    #[test]
    fn example_p1() {
        assert_eq!(solve_p1(25, &[5, 5, 10, 15, 20]), 4);
    }
}
