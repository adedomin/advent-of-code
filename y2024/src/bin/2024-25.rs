use aoc_shared::read_input_to_string;
use itertools::Itertools;
use std::io;

type Int = i32;
type Output = [Vec<[Int; PIN_CNT]>; 2];
const KEY: usize = 0;
const LOCK: usize = 1;
const PIN_CNT: usize = 5;
const LOCK_DEPTH: Int = 7;

fn parse_input(input: &str) -> Output {
    let mut ret = [vec![], vec![]];
    let pat = |c| (c == '#') as Int;
    input.split("\n\n").for_each(|pattern| {
        let (t, pat) = pattern
            .chars()
            .filter(|&c| c == '.' || c == '#')
            .tuples()
            .enumerate()
            .fold((KEY, [0; PIN_CNT]), |(t, cnt), (i, (a, b, c, d, e))| {
                // println!("- {a} {b} {c} {d} {e} -");
                let t = if i == 0 && (a, b, c, d, e) == ('#', '#', '#', '#', '#') {
                    LOCK
                } else {
                    t
                };
                (
                    t,
                    [
                        cnt[0] + pat(a),
                        cnt[1] + pat(b),
                        cnt[2] + pat(c),
                        cnt[3] + pat(d),
                        cnt[4] + pat(e),
                    ],
                )
            });
        ret[t].push(pat);
    });
    ret
}

fn part1_sol([keys, locks]: Output) -> Int {
    locks
        .iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| {
                    lock.iter()
                        .zip(key.iter())
                        .all(|(lp, kp)| lp + kp <= LOCK_DEPTH)
                })
                .count() as Int
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
