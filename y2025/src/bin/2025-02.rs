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

const TEN: Int = 10;

fn solve2(i: &[(Int, Int)]) -> (Int, Int) {
    i.iter()
        .flat_map(|&(s, e)| s..(e + 1))
        .fold((0, 0), |(p1, p2), n| {
            let digits = n.ilog10() + 1;
            // must have at least 2 digits to "repeat" any pattern.
            if digits < 2 {
                return (p1, p2);
            }

            let mid = digits / 2;
            let is_even = digits.is_multiple_of(2);
            if let Some(pos) = (1..mid + 1)
                .rev()
                .filter(|&sub| digits.is_multiple_of(sub))
                .find(|&sublen| {
                    let pow = TEN.pow(sublen);
                    let sub = n % pow;
                    let reconstructed = (0..digits / sublen).fold(0, |acc, _| acc * pow + sub);
                    n == reconstructed
                })
            {
                (p1 + n * Int::from(pos == mid && is_even), p2 + n)
            } else {
                (p1, p2)
            }
        })
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let input = parse_input(&input);
    let (part1, part2) = solve2(&input);
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
