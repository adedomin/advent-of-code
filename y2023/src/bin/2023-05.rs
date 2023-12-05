#![feature(btree_cursors)]
#![feature(array_chunks)]
use aoc_shared::{atoi, destructure_or_none, read_input, try_atoi, GroupTokenize, Token};
use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{
    collections::BTreeMap,
    io,
    ops::{Bound, Range},
};

type Output = (
    Vec<i64>,
    Vec<BTreeMap<i64, (i64, i64)>>,
    Vec<BTreeMap<i64, (i64, i64)>>,
);

fn parse_input(input: &[u8]) -> Output {
    let mut iter = input.group_tokens(Token::DoubleNewline);
    let seedlist = iter.next().expect("expected to have seeds");
    let seedlist = seedlist
        .iter()
        .skip_while(|t| !matches!(t, Token::Delimiter(b':')))
        .flat_map(|t| destructure_or_none!(Token::Something|word| = t))
        .map(|w| atoi::<i64, 10>(w))
        .collect::<Vec<i64>>();

    let mut forward = vec![];
    let mut reverse = vec![];

    iter.for_each(|toks| {
        let mut ftree = BTreeMap::new();
        let mut rtree = BTreeMap::new();
        toks.iter()
            .flat_map(|t| destructure_or_none!(Token::Something|word| = t))
            .flat_map(|t| try_atoi::<i64, 10>(t))
            .tuples()
            .for_each(|(dst, src, to)| {
                ftree.insert(src, (src + to, dst));
                rtree.insert(dst, (dst + to, src));
            });
        forward.push(ftree);
        reverse.push(rtree);
    });
    (
        seedlist,
        forward,
        reverse.iter().rev().cloned().collect::<Vec<_>>(),
    )
}

fn descend_alma(seed: i64, almanac: &[BTreeMap<i64, (i64, i64)>]) -> i64 {
    let mut last = seed;
    for map in almanac {
        let b = map.upper_bound(Bound::Included(&last));
        if let Some((src, (srcto, dst))) = b.key_value() {
            if *srcto > last {
                let diff = last.abs_diff(*src) as i64;
                last = *dst + diff;
            }
        }
    }
    last
}

fn bottom_up_alma(
    seed: i64,
    almanac: &[BTreeMap<i64, (i64, i64)>],
    seed_interval: &[Range<i64>],
) -> Option<i64> {
    let nseed = descend_alma(seed, almanac);
    if seed_interval
        .iter()
        .find(|iv| iv.contains(&nseed))
        .is_some()
    {
        Some(nseed)
    } else {
        None
    }
}

fn lowest_seed(seeds: &[i64], almanac: &[BTreeMap<i64, (i64, i64)>]) -> i64 {
    seeds
        .iter()
        .copied()
        .map(|seed| descend_alma(seed, almanac))
        .min()
        .unwrap()
}

fn solve_interval(seeds: &[i64], almanac: &[BTreeMap<i64, (i64, i64)>]) -> i64 {
    let seed_interv = seeds
        .array_chunks()
        .map(|&[start, end]| {
            let end = start + end;
            start..end
        })
        .collect::<Vec<_>>();
    let biggest = seed_interv
        .iter()
        .max_by(|ix, iy| ix.end.cmp(&iy.end))
        .unwrap()
        .end;
    (0..biggest)
        .into_par_iter()
        .find_first(|&backward_v| bottom_up_alma(backward_v, almanac, &seed_interv).is_some())
        .unwrap()
}

// fn part2_sol(input: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (init_seed, almanac, rev_alma) = parse_input(&input);
    let part1 = lowest_seed(&init_seed, &almanac);
    let part2 = solve_interval(&init_seed, &rev_alma);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
