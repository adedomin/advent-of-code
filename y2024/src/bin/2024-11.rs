use aoc_shared::{fold_decimal_from, read_input_to_string};
use rustc_hash::FxHashMap;
use std::io;

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
    if id % 2 == 0 {
        let mult = (10 as Int).pow(id / 2);
        Err([i / mult, i % mult])
    } else {
        Ok(i * 2024)
    }
}

fn find_cnt(i: Int, find: Int) -> Int {
    let mut memo = FxHashMap::default();
    fn rec(memo: &mut FxHashMap<(Int, Int), Int>, i: Int, depth: Int, find: Int) -> Int {
        if depth == find {
            return 1;
        }

        if let Some(m) = memo.get(&(i, depth)) {
            return *m;
        }

        let counts = match rules(i) {
            Ok(ni) => rec(memo, ni, depth + 1, find),
            Err(ni) => ni.into_iter().map(|i| rec(memo, i, depth + 1, find)).sum(),
        };
        memo.insert((i, depth), counts);
        counts
    }
    rec(&mut memo, i, 0, find)
}

fn solve(input: &Output, cycle: Int) -> Int {
    input.iter().map(|&num| find_cnt(num, cycle)).sum()
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = solve(&parsed_input, 25);
    let part2 = solve(&parsed_input, 75);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
