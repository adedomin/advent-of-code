use aoc_shared::read_input_to_string;
use std::io;

type Output = Vec<u64>;

fn parse_input(input: &str) -> Output {
    let pat = |b| (b == b'#') as u64;
    input
        .split("\n\n")
        .filter(|pat| !pat.is_empty())
        .map(|pattern| {
            pattern
                .as_bytes()
                .iter()
                .filter(|&&b| b == b'.' || b == b'#')
                .fold(0u64, |acc, &b| acc << 1 | pat(b))
        })
        .collect::<Vec<u64>>()
}

fn part1_sol(keys_and_locks: Output) -> u64 {
    // we exploit the top and bottom rows differentiating the two and they *ALWAYS* self filter
    // we basically just ask if both lock and key occupy the same space, if not, then ANDing them is always 0.
    keys_and_locks
        .iter()
        .enumerate()
        .flat_map(|(idx, kl)| {
            keys_and_locks[..idx]
                .iter()
                .map(move |&kl2| ((kl & kl2) == 0) as u64)
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(parsed_input);
    println!("Part1: {part1}");
    Ok(())
}
