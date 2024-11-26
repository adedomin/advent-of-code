use aoc_shared::{read_input_to_string, try_atoi};
use itertools::Itertools;
use std::io;

type Output = Vec<i64>;
type Solved = i64;

fn parse_input(input: &str) -> Output {
    input
        .split_ascii_whitespace()
        .filter_map(|num| try_atoi::<_, 10>(num.as_bytes()))
        .collect::<Vec<_>>()
}

fn part1_sol<const GROUP_CNT: i64>(input: &Output) -> Solved {
    let group_target = input.iter().sum::<i64>();
    // the sum must be divisible by 3
    assert_eq!(group_target % GROUP_CNT, 0);
    let group_target = group_target / GROUP_CNT;
    input
        .iter()
        .copied()
        .powerset()
        .filter_map(|perm| {
            let sum = perm.iter().sum::<i64>();
            if sum == group_target {
                let quantum = perm.iter().product::<i64>();
                Some((perm.len(), quantum))
            } else {
                None
            }
        })
        .min()
        .expect("At least one group can be made.")
        .1
}

// fn part2_sol(input: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol::<3>(&parsed_input);
    let part2 = part1_sol::<4>(&parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
