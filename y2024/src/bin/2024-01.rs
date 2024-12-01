use aoc_shared::{fold_decimal_from, read_input_to_string};
use itertools::Itertools;
use std::io;

type Solved = u64;
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

fn get_counts(v: Vec<Solved>) -> impl Iterator<Item = (Solved, Solved)> {
    v.into_iter().map(|x| (x, 1)).coalesce(|x, y| {
        if x.0 == y.0 {
            Ok((x.0, x.1 + 1))
        } else {
            Err((x, y))
        }
    })
}

fn part2_sol([left, right]: Output) -> Solved {
    let right_counts = get_counts(right).collect::<Vec<(Solved, Solved)>>();
    get_counts(left)
        .fold((0, 0), |(mut idx, sum), (lnum, lcount)| {
            while let Some((rnum, rcount)) = right_counts.get(idx) {
                match lnum.cmp(rnum) {
                    // next right number cannot match current left.
                    std::cmp::Ordering::Less => return (idx, sum),
                    std::cmp::Ordering::Equal => return (idx, sum + (rnum * rcount * lcount)),
                    // find next matching right number.
                    std::cmp::Ordering::Greater => idx += 1,
                }
            }
            (idx, sum) // no more numbers on right match.
        })
        .1
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
