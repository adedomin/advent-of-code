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

// fn find_cycle(i: Int, cycle: Int) -> Int {
//     let mut nums = vec![i];
//     println!("{nums:?}");
//     for _ in 0..cycle {
//         nums = nums
//             .drain(..)
//             .flat_map(|n| {
//             })
//             .collect::<Vec<_>>();
//         // println!("{nums:?}");
//     }
//     nums.len() as Int
// }

fn rules(i: Int) -> Vec<Int> {
    if i == 0 {
        return vec![1];
    }

    let id = i.ilog10() + 1;
    if id % 2 == 0 {
        let mult = (10 as Int).pow(id / 2);
        vec![i / mult, i % mult]
    } else {
        vec![i * 2024]
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

        let counts = rules(i)
            .into_iter()
            .map(|i| rec(memo, i, depth + 1, find))
            .sum();
        memo.insert((i, depth), counts);
        counts
    }
    rec(&mut memo, i, 0, find)
}

fn part1_sol(input: &Output) -> Int {
    input.iter().map(|&num| find_cnt(num, 75)).sum()
}

// fn part2_sol(input: &Output) -> Int {}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    // let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    // print!("Part2: {part2}");
    println!();
    Ok(())
}
