use aoc_shared::{atoi, destructure_or_none, read_input, try_atoi, GroupTokenize, Token};
use itertools::Itertools;
use std::{io, ops::Range};

type Output = (Vec<i64>, Vec<Vec<(Range<i64>, i64)>>);

fn parse_input(input: &[u8]) -> Output {
    let mut iter = input.group_tokens(Token::DoubleNewline);
    let seedlist = iter.next().expect("expected to have seeds");
    let seedlist = seedlist
        .iter()
        .skip_while(|t| !matches!(t, Token::Delimiter(b':')))
        .flat_map(|t| destructure_or_none!(Token::Something|word| = t))
        .map(|w| atoi::<i64, 10>(w))
        .collect::<Vec<i64>>();

    let intervals = iter
        .map(|toks| {
            toks.iter()
                .flat_map(|t| destructure_or_none!(Token::Something|word| = t))
                .flat_map(|t| try_atoi::<i64, 10>(t))
                .tuples()
                .map(|(dst, src, to)| (src..src + to, dst - src))
                .collect_vec()
        })
        .collect_vec();
    (seedlist, intervals)
}

fn solve_interval(mut seeds: Vec<(bool, Range<i64>)>, almanac: &[Vec<(Range<i64>, i64)>]) -> i64 {
    for alma in almanac {
        let mut new_ranges = vec![];
        for (iv, off) in alma {
            let mut read_unchanged = vec![];
            seeds
                .iter_mut()
                .filter(|(a, siv)| *a && (iv.start < siv.end && siv.start < iv.end))
                .for_each(|(active, siv)| {
                    *active = false;
                    let mut ns = iv.start;
                    if siv.start < iv.start {
                        read_unchanged.push((true, siv.start..iv.start));
                    } else {
                        ns = siv.start;
                    }

                    if siv.end <= iv.end {
                        new_ranges.push((true, ns + off..siv.end + off));
                    } else {
                        new_ranges.push((true, ns + off..iv.end + off));
                        read_unchanged.push((true, iv.end..siv.end));
                    }
                });
            seeds.extend(read_unchanged);
        }
        seeds = seeds
            .into_iter()
            .filter(|(a, _)| *a)
            .chain(new_ranges)
            .sorted_by_key(|(_, i)| i.start)
            .collect_vec();
    }

    seeds
        .into_iter()
        .filter(|(a, _)| *a)
        .map(|(_, s)| s.start)
        .sorted()
        .next()
        .expect("at least one seed.")
}

// fn part2_sol(input: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (init_seed, almanac) = parse_input(&input);

    let interval_1 = init_seed
        .iter()
        .map(|&seed| (true, seed..seed + 1))
        .collect_vec();
    let part1 = solve_interval(interval_1, &almanac);

    let interval_2 = init_seed
        .iter()
        .tuples()
        .map(|(&seed1, &seed2)| (true, seed1..seed1 + seed2))
        .collect_vec();
    let part2 = solve_interval(interval_2, &almanac);

    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
