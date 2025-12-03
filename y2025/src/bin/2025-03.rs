use std::{cmp::Ordering, io};

use aoc_shared::read_input_to_string;

fn parse_input(i: &str) -> Vec<Vec<u8>> {
    i.split_ascii_whitespace()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| {
                    let res = b - b'0';
                    assert!(
                        (1..=9).contains(&res),
                        "battery value must be between 1 and 9"
                    );
                    res
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>()
}

// fn solve(arr: &[Vec<u8>]) -> u32 {
//     arr.iter()
//         .map(|bank| {
//             assert!(
//                 bank.len() > 1,
//                 "battery bank too small! must be greater than 1."
//             );
//             bank[2..]
//                 .iter()
//                 .fold(u32::from(bank[0] * 10 + bank[1]), |acc, &batt| {
//                     let rep_hi = acc % 10 * 10 + batt as u32;
//                     let rep_lo = acc / 10 * 10 + batt as u32;
//                     acc.max(rep_hi).max(rep_lo)
//                 })
//         })
//         .sum()
// }

fn solve2<const N: usize>(arr: &[Vec<u8>]) -> u64 {
    arr.iter()
        .map(|bank| {
            assert!(bank.len() >= N, "battery bank too small!");
            let mut off = 0;
            let mut ans = 0;
            for i in 0..N {
                let (noff, digit) = bank
                    .iter()
                    .enumerate()
                    .skip(off + 1)
                    .take(bank.len() - off - (N - i))
                    .fold((off + 1, bank[off]), |(off, b), (idx, batt)| {
                        match b.cmp(batt) {
                            Ordering::Less => (
                                idx + 1, // must offset idx by 1 for next loop itr
                                *batt,
                            ),
                            Ordering::Equal | Ordering::Greater => (off, b),
                        }
                    });
                off = noff;
                ans = ans * 10 + digit as u64;
            }
            ans
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let input = parse_input(input.trim());
    let part1 = solve2::<2>(&input);
    let part2 = solve2::<12>(&input);
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
