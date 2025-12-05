use std::io;

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
        if cand_s <= curr_e {
            // still have to pick the biggest of the two...
            *curr_e = *cand_e.max(curr_e);
            true
        } else {
            false
        }
    });
    let ids = ids
        .split_ascii_whitespace()
        .map(|num| num.parse::<u64>().expect("Valid number."))
        .collect::<Vec<_>>();
    (ranges, ids)
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let (ranges, ids) = parse_input(input.trim());
    let part1 = ids
        .iter()
        .filter(|id| ranges.iter().any(|(s, e)| (s..=e).contains(id)))
        .count();
    let part2 = ranges.iter().map(|(s, e)| e - s + 1).sum::<u64>();
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
