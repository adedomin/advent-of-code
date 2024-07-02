use std::io;

use aoc_shared::read_input;

fn fold_decimal(acc: u64, chr: u8) -> u64 {
    acc * 10 + (chr - b'0') as u64
}

fn parse(input: Vec<u8>) -> Vec<u64> {
    let mut crabs = vec![];
    let mut num = 0u64;
    let last = *input.last().unwrap();
    for digit in input {
        match digit {
            b'0'..=b'9' => {
                num = fold_decimal(num, digit);
            }
            b',' | b'\n' => {
                crabs.push(num);
                num = 0u64;
            }
            _ => (),
        }
    }
    if last.is_ascii_digit() {
        crabs.push(num);
    }
    crabs
}

fn p2_cost(n: u64) -> u64 {
    //(1..=n).sum()
    (n * (n + 1)) / 2
}

fn solve(crabs: &mut [u64]) -> (u64, u64) {
    let is_odd = crabs.len() % 2 == 1;
    let med_pos = crabs.len() / 2;
    let median = if is_odd {
        let (_, select, _) = crabs.select_nth_unstable(med_pos + 1);
        *select
    } else {
        let (smaller, rhs_m, _) = crabs.select_nth_unstable(med_pos);
        let (_, lhs_m, _) = smaller.select_nth_unstable(smaller.len() - 1);
        (*lhs_m + *rhs_m) / 2
    };
    let part1 = crabs.iter().map(|&crab| crab.abs_diff(median)).sum::<u64>();

    let mean = crabs.iter().sum::<u64>() / crabs.len() as u64;
    let part2_guess1 = crabs
        .iter()
        .map(|&crab| p2_cost(crab.abs_diff(mean)))
        .sum::<u64>();
    let part2_guess2 = crabs
        .iter()
        .map(|&crab| p2_cost(crab.abs_diff(mean + 1)))
        .sum::<u64>();
    (part1, part2_guess1.min(part2_guess2))
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let mut crabs = parse(input);
    let (p1, p2) = solve(&mut crabs);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
