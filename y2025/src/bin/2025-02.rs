use std::{collections::HashSet, io};

use aoc_shared::read_input_to_string;

type Int = u64;

fn parse_input(i: &str) -> Vec<(Int, Int)> {
    i.split([',', '\n'])
        .flat_map(|line| {
            let (start, end) = line.split_once('-')?;
            let start = start.parse::<Int>().expect("valid number");
            let end = end.parse::<Int>().expect("valid number");
            Some((start, end))
        })
        .collect::<Vec<_>>()
}

const TEN: Int = 10;
const PATS_1: [(u32, Int); 5] = [
    (1, 11),     // 2
    (2, 101),    // 4
    (3, 1001),   // 6
    (4, 10001),  // 8
    (5, 100001), // 10
];
const PATS_2: [(u32, Int); 7] = [
    (1, 111),       // 3
    (1, 11111),     // 5
    (2, 10101),     // 6
    (1, 1111111),   // 7
    (2, 1010101),   // 8
    (3, 1001001),   // 9
    (2, 101010101), // 10
];

fn repeating(pat: &[(u32, Int)]) -> impl Iterator<Item = Int> {
    let mut i = 0;
    let mut range = TEN.pow(pat[i].0 - 1)..TEN.pow(pat[i].0);
    std::iter::successors(Some(pat[i].1 * range.next().unwrap()), move |_| {
        if let Some(n) = range.next() {
            Some(pat[i].1 * n)
        } else if i + 1 < pat.len() {
            i += 1;
            range = TEN.pow(pat[i].0 - 1)..TEN.pow(pat[i].0);
            Some(pat[i].1 * range.next().unwrap())
        } else {
            None
        }
    })
}

fn solve(input: &[(Int, Int)], range: &[(u32, Int)], dupes: &mut HashSet<Int>) -> Int {
    let mut i = 0;
    let mut acc = 0;
    'out: for r in repeating(range) {
        loop {
            if let Some(&(s, e)) = input.get(i) {
                if r <= e {
                    if s <= r && dupes.insert(r) {
                        acc += r
                    }
                    continue 'out;
                } else if r > e {
                    i += 1;
                }
            } else {
                // no more ranges.
                break 'out;
            }
        }
    }
    acc
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let mut input = parse_input(&input);
    input.sort_unstable();
    // the part2 ranges overlap slightly.
    let mut dupes = HashSet::new();
    let part1 = solve(&input, &PATS_1, &mut dupes);
    let part2 = solve(&input, &PATS_2, &mut dupes) + part1;
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
