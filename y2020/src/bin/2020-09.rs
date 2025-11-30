use aoc_shared::{array_windows, fold_decimal_from, read_input_to_string};
use std::io;

type Output = Vec<i64>;
type Solved = i64;

const PREAMBLE: usize = 26;

fn parse_input(input: &str) -> Output {
    input
        .split_ascii_whitespace()
        .map(|word| fold_decimal_from(word.as_bytes()))
        .collect::<Vec<_>>()
}

fn part1_sol(input: &Output) -> Solved {
    *array_windows::<_, PREAMBLE>(input)
        .find(|nums| {
            let (preamble, last) = nums.split_at(PREAMBLE - 1);
            !preamble
                .iter()
                .any(|&num| preamble.contains(&(last[0] - num)))
        })
        .expect("There has to be at least one invalid number in the stream.")
        .last()
        .expect("???")
}

fn part2_sol(input: &Output, inval: i64) -> Solved {
    for i in 0..(input.len() - 1) {
        for j in (i + 1)..input.len() {
            if input[i..j].iter().sum::<i64>() == inval {
                let (min, max) =
                    input[i..j]
                        .iter()
                        .fold((i64::MAX, i64::MIN), |(min, max), &num| {
                            let min = num.min(min);
                            let max = num.max(max);
                            (min, max)
                        });
                assert!(min != i64::MAX && max != i64::MIN && min != max);
                return min + max;
            }
        }
    }
    panic!("NO SAT");
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input, part1);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
