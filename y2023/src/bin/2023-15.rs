use aoc_shared::{atoi, read_input_to_string, Tokenize};
use std::io;

/// Holiday ASCII String Helper algorithm (HASH)
fn HASH_hasher(v: &[u8]) -> u8 {
    v.iter()
        .fold(0, |acc, &chr| acc.wrapping_add(chr).wrapping_mul(17))
}

fn parse_solve(input: &str) -> u32 {
    input
        .split(',')
        .map(|s| HASH_hasher(s.as_bytes()) as u32)
        .sum()
}

fn solve2(input: &[u8]) -> usize {
    input
        .tokenize()
        .fold(
            (vec![vec![]; 256], b"".as_slice()),
            |(mut acc, clabel), tok| match tok {
                aoc_shared::Token::Something(label) if clabel.is_empty() => (acc, label),
                aoc_shared::Token::Something(focal) => {
                    let idx = HASH_hasher(clabel) as usize;
                    let focal = atoi::<u8, 10>(focal);
                    if let Some(found) = acc[idx].iter_mut().find(|(l, _)| *l == clabel) {
                        *found = (found.0, focal);
                    } else {
                        acc[idx].push((clabel, focal));
                    }
                    (acc, b"")
                }
                aoc_shared::Token::Delimiter(b'-') => {
                    let idx = HASH_hasher(clabel) as usize;
                    acc[idx] = acc[idx]
                        .drain(..)
                        .filter(|(label, _)| *label != clabel)
                        .collect::<Vec<_>>();
                    (acc, b"")
                }
                _ => (acc, clabel),
            },
        )
        .0
        .into_iter()
        .enumerate()
        .filter(|(_, b)| !b.is_empty())
        .map(|(box_num, b)| {
            b.iter()
                .enumerate()
                .map(|(pos, (_, focal))| (box_num + 1) * (pos + 1) * (*focal as usize))
                .sum::<usize>()
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let part1 = parse_solve(input.trim_end());
    let part2 = solve2(input.as_bytes());
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
