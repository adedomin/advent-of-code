use aoc_shared::{fold_decimal_from, read_input_to_string};
use itertools::Itertools;
use std::{collections::HashMap, io, iter::once};

type Output = Vec<i64>;
type Solved = i64;

fn parse_input(input: &str) -> Output {
    let mut i = input
        .split_ascii_whitespace()
        .map(|word| fold_decimal_from(word.as_bytes()))
        .collect::<Vec<_>>();
    i.sort_unstable();

    let integrated_adp = i.last().expect("at least one adapter needs to be given...") + 3;
    // both p1 and 2 need the 0 and last adapter in the list.
    once(0)
        .chain(i.into_iter())
        .chain(once(integrated_adp))
        .collect::<Output>()
}

fn part1_sol(input: &Output) -> Solved {
    let (d1, d3) = input
        .iter()
        .tuple_windows()
        .fold((0, 0), |(d1, d3), (x, y)| match y - x {
            1 => (d1 + 1, d3),
            3 => (d1, d3 + 1),
            _ => panic!("Invalid adapter list"),
        });
    d1 * d3
}

fn part2_sol(input: &Output) -> Solved {
    let integrated = input.last().unwrap();
    // 0 can only be attached to one, much like the last can only be attached to one
    let mut paths_from = HashMap::from([(0, 1)]);
    input.iter().skip(1).for_each(|&adp| {
        let cellm1 = *paths_from.entry(adp - 1).or_insert(0);
        let cellm2 = *paths_from.entry(adp - 2).or_insert(0);
        let cellm3 = *paths_from.entry(adp - 3).or_insert(0);
        *paths_from.entry(adp).or_insert(0) = cellm1 + cellm2 + cellm3;
    });
    *paths_from
        .get(integrated)
        .expect("integrated should have been visited (and set)")
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
