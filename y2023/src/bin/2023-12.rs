use aoc_shared::{read_input_to_string, FlatVec2D};
use std::io;

type Output = Vec<(Vec<u8>, Vec<Vec<u8>>)>;

fn parse_input(input: &str) -> Output {
    input
        .split('\n')
        .flat_map(|line| {
            if line.is_empty() {
                return None;
            }

            let (springs, counts) = line
                .split_once(' ')
                .expect("expected to split springs from counts");
            let mut springs = springs.as_bytes().to_owned();
            springs.push(b'.'); // pad with broken so patterns can match end after the pattern is padded.

            let counts = counts
                .split(',')
                .map(|count| {
                    let l = count.parse::<usize>().unwrap();
                    let mut count = vec![b'#'; l];
                    // pad pattern because patterns cannot be touching, e.g. 2,1 cannot be ###, but can be ##.#
                    count.push(b'.');
                    count
                })
                .collect::<Vec<Vec<u8>>>();
            Some((springs, counts))
        })
        .collect::<Output>()
}

fn unfold_and_solve(springs: &[u8], counts: &[Vec<u8>]) -> u64 {
    // remove padding...
    let mut unfold_springs = [&springs[..springs.len() - 1]; 5].join(&b'?');
    unfold_springs.push(b'.'); // add it back at the end.

    let unfold_counts = counts
        .iter()
        .cloned()
        .cycle()
        .take(counts.len() * 5)
        .collect::<Vec<Vec<u8>>>();
    solve_grp(&unfold_springs, &unfold_counts)
}

fn solve_grp(springs: &[u8], counts: &[Vec<u8>]) -> u64 {
    let mut memo = FlatVec2D::<u64>::new(springs.len(), counts.len());
    fn tree_rec(
        memo: &mut FlatVec2D<u64>,
        springs: &[u8],
        counts: &[Vec<u8>],
        si: usize,
        ci: usize,
    ) -> u64 {
        if ci >= counts.len() {
            return (!springs.get(si..).unwrap_or_default().contains(&b'#')) as u64;
        } else if si >= springs.len() {
            return 0;
        }

        let m = memo[(si, ci)];
        if m > 0 {
            return m - 1;
        }

        let mut ret = 0;

        if matches!(springs[si], b'.' | b'?') {
            ret += tree_rec(memo, springs, counts, si + 1, ci);
        }

        if matches!(springs[si], b'#' | b'?') {
            let count = &counts[ci];
            let subs = &springs[si..];
            if count.len() <= subs.len() {
                if count
                    .iter()
                    .zip(subs.iter())
                    .try_fold(
                        (),
                        |_, (&c, &s)| {
                            if c == s || s == b'?' {
                                Some(())
                            } else {
                                None
                            }
                        },
                    )
                    .is_some()
                {
                    ret += tree_rec(memo, springs, counts, si + count.len(), ci + 1);
                }
            }
        }

        memo[(si, ci)] = ret + 1;
        ret
    }
    tree_rec(&mut memo, springs, counts, 0, 0)
}

fn solve(input: &[(Vec<u8>, Vec<Vec<u8>>)]) -> (u64, u64) {
    let p1 = input
        .iter()
        .map(|(springs, counts)| solve_grp(springs, counts))
        .sum();
    let p2 = input
        .iter()
        .map(|(springs, counts)| unfold_and_solve(springs, counts))
        .sum();
    (p1, p2)
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let input = parse_input(&input);
    let (part1, part2) = solve(&input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
