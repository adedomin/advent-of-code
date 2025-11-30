use aoc_shared::{fold_decimal_from, inner_or_none, read_input_to_string};
use itertools::{EitherOrBoth, Itertools};
use std::io;

type Solved = usize;
type Output = [Vec<Solved>; 2];

fn parse_input(input: &str) -> Output {
    let (mut left, mut right) = input
        .split_ascii_whitespace()
        .map(|num| fold_decimal_from::<Solved>(num.as_bytes()))
        .tuples()
        .collect::<(Vec<_>, Vec<_>)>();
    // both sides benefit from this
    // naive p2, not so much.
    left.sort_unstable();
    right.sort_unstable();
    [left, right]
}

fn part1_sol([left, right]: &Output) -> Solved {
    left.iter()
        .zip(right.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum::<Solved>()
}

fn part2_sol([left, right]: Output) -> Solved {
    left.into_iter()
        .dedup_with_count()
        .merge_join_by(right.into_iter().dedup_with_count(), |a, b| a.1.cmp(&b.1))
        .flat_map(|lr| inner_or_none!(EitherOrBoth::Both|(l, r)| = lr))
        .fold(0, |sum, ((lcount, _), (rcount, rnum))| {
            sum + (rnum * lcount * rcount)
        })
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
