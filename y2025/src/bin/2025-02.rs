use std::io;

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

fn solve(i: &[(Int, Int)]) -> Int {
    i.iter().flat_map(|&(s, e)| s..(e + 1)).fold(0, |acc, n| {
        let digits = n.ilog10() + 1;
        let pow = (10 as Int).pow(digits / 2);
        let upper = n / pow;
        let lower = n % pow;
        if upper == lower { acc + n } else { acc }
    })
}

fn solve2(i: &[(Int, Int)]) -> Int {
    i.iter().flat_map(|&(s, e)| s..(e + 1)).fold(0, |acc, n| {
        let nstr = n.to_string();
        let len = nstr.len();
        if len < 2 {
            return acc;
        }

        let nb = nstr.as_bytes();
        for i in 1..(len / 2 + 1) {
            let mut chunks = nb.chunks_exact(i);
            if !chunks.remainder().is_empty() {
                continue;
            } else if chunks.all(|chunk| &nb[..i] == chunk) {
                return acc + n;
            }
        }
        acc
    })
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let input = parse_input(&input);
    let part1 = solve(&input);
    let part2 = solve2(&input);
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
