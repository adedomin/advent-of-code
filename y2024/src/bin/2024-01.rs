use aoc_shared::{fold_decimal_from, read_input_to_string};
use itertools::Itertools;
use std::io;

type Solved = u64;
type Output = [Vec<Solved>; 2];

fn parse_input(input: &str) -> Output {
    let mut ret = [vec![], vec![]];
    input
        .split_ascii_whitespace()
        .enumerate()
        .for_each(|(i, num)| {
            let idx = i & 1;
            ret[idx].push(fold_decimal_from(num.as_bytes()));
        });
    // both sides benefit from this
    // naive p2, not so much.
    ret[0].sort_unstable();
    ret[1].sort_unstable();
    ret
}

fn part1_sol([left, right]: &Output) -> Solved {
    left.iter()
        .zip(right.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum::<Solved>()
}

fn part2_sol([left, right]: Output) -> Solved {
    let num_counts = right
        .into_iter()
        .map(|x| (x, 1))
        .coalesce(|x, y| {
            if x.0 == y.0 {
                Ok((x.0, x.1 + 1))
            } else {
                Err((x, y))
            }
        })
        .collect::<Vec<(Solved, Solved)>>();
    left.into_iter()
        .fold((0, 0), |(mut idx, sum), num| {
            while let Some((lastn, lastc)) = num_counts.get(idx) {
                match num.cmp(lastn) {
                    // next right number cannot match current left.
                    std::cmp::Ordering::Less => return (idx, sum),
                    std::cmp::Ordering::Equal => return (idx, sum + (num * lastc)),
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
