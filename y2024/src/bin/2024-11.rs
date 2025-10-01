use aoc_shared::{fold_decimal_from, read_input_to_string};
use rustc_hash::FxHashMap;
use std::{io, mem};

type Int = u64;
type Output = Vec<u64>;

fn parse_input(input: &str) -> Output {
    input
        .split_ascii_whitespace()
        .map(|word| fold_decimal_from(word.as_bytes()))
        .collect::<Output>()
}

fn rules(i: Int) -> Result<Int, [Int; 2]> {
    if i == 0 {
        return Ok(1);
    }

    let id = i.ilog10() + 1;
    if id.is_multiple_of(2) {
        let mult = (10 as Int).pow(id / 2);
        Err([i / mult, i % mult])
    } else {
        Ok(i * 2024)
    }
}

const P1_CYCLE: Int = 25;
// we reuse the existing p1 cycle for p2
const P2_CONT: Int = 75 - P1_CYCLE;

fn solve(input: &Output) -> [Int; 2] {
    let mut memo = FxHashMap::default();
    let mut memo2 = FxHashMap::default();
    input.iter().for_each(|&n| {
        *memo.entry(n).or_default() += 1;
    });

    [P1_CYCLE, P2_CONT].map(|cycles| {
        for _ in 0..cycles {
            memo.drain().for_each(|(plu, counts)| {
                let mut add_new = |plu| *memo2.entry(plu).or_default() += counts;
                match rules(plu) {
                    Ok(new) => add_new(new),
                    Err(new) => new.into_iter().for_each(add_new),
                }
            });
            mem::swap(&mut memo, &mut memo2);
        }
        memo.values().sum()
    })
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let [part1, part2] = solve(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
