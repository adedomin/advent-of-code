use std::{cmp::Ordering, io};

use aoc_shared::read_input_to_string;

fn parse_input(i: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges, ids) = i.split_once("\n\n").expect("Invalid input.");
    let mut ranges = ranges
        .split_ascii_whitespace()
        .map(|line| {
            let (start, end) = line.split_once('-').expect("Range");
            let start = start.parse::<u64>().expect("valid number");
            let end = end.parse::<u64>().expect("valid number");
            (start, end)
        })
        .collect::<Vec<_>>();
    ranges.sort_unstable();
    // dedup_by starts from the start at the rhs, and the candidate on the lhs
    ranges.dedup_by(|(cand_s, cand_e), (_curr_s, curr_e)| {
        // because of sort order, cand_s being -le curr_e means its in the range.
        match cand_s.cmp(&curr_e) {
            Ordering::Less | Ordering::Equal => {
                *curr_e = *cand_e.max(curr_e);
                true
            }
            Ordering::Greater => false,
        }
    });
    let mut ids = ids
        .split_ascii_whitespace()
        .map(|num| num.parse::<u64>().expect("Valid number."))
        .collect::<Vec<_>>();
    ids.sort_unstable();
    (ranges, ids)
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let (ranges, ids) = parse_input(input.trim());
    let part1 = {
        let mut ri = 0;
        ids.iter().fold(0, |acc, id| {
            while let Some((s, e)) = ranges.get(ri) {
                if id <= e {
                    return acc + u64::from(s <= id);
                } else {
                    ri += 1;
                }
            }
            acc
        })
    };
    let part2 = ranges.iter().map(|(s, e)| e - s + 1).sum::<u64>();
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
