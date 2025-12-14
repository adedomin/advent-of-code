use std::{collections::HashMap, io, vec::Vec};

use aoc_shared::read_input_to_string;

const YOU: &str = "you";
const SVR: &str = "svr";
const DAC: &str = "dac";
const FFT: &str = "fft";

fn parse_input(i: &str) -> (HashMap<&str, usize>, Vec<Vec<usize>>) {
    let mut idmap = HashMap::default();
    let mut pathmap = HashMap::new();
    i.split('\n').for_each(|line| {
        let (key, paths) = line.split_once(": ").expect("key: value[, values]+ pair");
        let slen = idmap.len();
        let slot = *idmap.entry(key).or_insert(slen);
        let paths = paths.split_ascii_whitespace().map(|key| {
            let slen = idmap.len();
            *idmap.entry(key).or_insert(slen)
        });
        pathmap.entry(slot).or_insert(vec![]).extend(paths);
    });
    let mut pathmap2 = vec![vec![]; idmap.len()];
    pathmap
        .drain()
        .for_each(|(slot, paths)| pathmap2[slot] = paths);
    (idmap, pathmap2)
}

fn recurse_paths2(
    memo: &mut [Option<(usize, [bool; 2])>],
    pathmap: &[Vec<usize>],
    curr: usize,
    dac: usize,
    fft: usize,
    mut seen: [bool; 2],
) -> usize {
    if pathmap[curr].is_empty() {
        return usize::from(seen == [true; 2]);
    } else if let Some((v, s)) = memo[curr]
        && s == seen
    {
        return v;
    }

    if curr == dac {
        seen[0] = true;
    } else if curr == fft {
        seen[1] = true;
    }

    let ret = pathmap[curr]
        .iter()
        .map(|p| recurse_paths2(memo, pathmap, *p, dac, fft, seen))
        .sum();
    memo[curr] = Some((ret, seen));
    ret
}

fn solve(pathmap: &[Vec<usize>], start: usize) -> usize {
    let mut memo = vec![None; pathmap.len()];
    recurse_paths2(&mut memo, pathmap, start, usize::MAX, usize::MAX, [true; 2])
}

fn solve2(pathmap: &[Vec<usize>], svr: usize, dac: usize, fft: usize) -> usize {
    let mut memo = vec![None; pathmap.len()];
    recurse_paths2(&mut memo, pathmap, svr, dac, fft, [false; 2])
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let (idmap, pathmap) = parse_input(input.trim());
    let part1 = if let Some(you) = idmap.get(YOU) {
        solve(&pathmap, *you)
    } else {
        0
    };
    let p2_ids: Option<[usize; 3]> = [SVR, DAC, FFT]
        .map(|id| idmap.get(id).copied())
        .into_iter()
        .collect::<Option<Vec<usize>>>()
        .and_then(|v| v.try_into().ok());
    let part2 = if let Some([svr, dac, fft]) = p2_ids {
        solve2(&pathmap, svr, dac, fft)
    } else {
        0
    };
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
