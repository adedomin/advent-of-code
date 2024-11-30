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

            let patterns = counts
                .split(',')
                .map(|count| {
                    let l = count.parse::<usize>().unwrap();
                    let mut pat = vec![b'#'; l];
                    // pad pattern because patterns cannot be touching, e.g. 2,1 cannot be ###, but can be ##.#
                    pat.push(b'.');
                    pat
                })
                .collect::<Vec<Vec<u8>>>();
            Some((springs, patterns))
        })
        .collect::<Output>()
}

fn unfold_and_solve(springs: &[u8], patterns: &[Vec<u8>]) -> u64 {
    // remove padding...
    let mut unfold_springs = [&springs[..springs.len() - 1]; 5].join(&b'?');
    unfold_springs.push(b'.'); // add it back at the end.

    let unfold_patterns = patterns
        .iter()
        .cloned()
        .cycle()
        .take(patterns.len() * 5)
        .collect::<Vec<Vec<u8>>>();
    solve_grp(&unfold_springs, &unfold_patterns)
}

fn solve_grp(springs: &[u8], patterns: &[Vec<u8>]) -> u64 {
    let mut memo = FlatVec2D::<Option<u64>>::new(springs.len(), patterns.len());
    fn brec(
        memo: &mut FlatVec2D<Option<u64>>,
        springs: &[u8],
        patterns: &[Vec<u8>],
        si: usize,
        pi: usize,
    ) -> u64 {
        if pi >= patterns.len() {
            return (!springs.get(si..).unwrap_or_default().contains(&b'#')) as u64;
        } else if si >= springs.len() {
            return 0;
        }

        if let Some(m) = memo[(si, pi)] {
            return m;
        }

        let mut ret = 0;

        if matches!(springs[si], b'.' | b'?') {
            ret += brec(memo, springs, patterns, si + 1, pi);
        }

        if matches!(springs[si], b'#' | b'?') {
            let pattern = &patterns[pi];
            let subs = &springs[si..];
            if pattern.len() <= subs.len()
                && pattern
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
                ret += brec(memo, springs, patterns, si + pattern.len(), pi + 1);
            }
        }

        memo[(si, pi)] = Some(ret);
        ret
    }
    brec(&mut memo, springs, patterns, 0, 0)
}

fn solve(input: &[(Vec<u8>, Vec<Vec<u8>>)]) -> (u64, u64) {
    let p1 = input
        .iter()
        .map(|(springs, patterns)| solve_grp(springs, patterns))
        .sum();
    let p2 = input
        .iter()
        .map(|(springs, patterns)| unfold_and_solve(springs, patterns))
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
