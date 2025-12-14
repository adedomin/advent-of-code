use std::{collections::HashMap, io};

use aoc_shared::read_input_to_string;

const YOU: usize = 0;
const SVR: &str = "svr";
const DAC: &str = "dac";
const FFT: &str = "fft";

fn parse_input(i: &str) -> (Vec<Vec<usize>>, Option<[usize; 3]>) {
    let mut last_id = 0usize;
    let mut idmap = HashMap::from([("you", last_id)]);
    let mut pathmap = vec![vec![]];
    i.split('\n').for_each(|line| {
        let (key, paths) = line.split_once(": ").expect("key: value[, values]+ pair");
        let slot_idx = if let Some(kid) = idmap.get(key) {
            *kid
        } else {
            idmap.insert(key, last_id + 1);
            last_id += 1;
            last_id
        };
        let paths = paths
            .split_ascii_whitespace()
            .map(|key| {
                if let Some(kid) = idmap.get(key) {
                    *kid
                } else {
                    idmap.insert(key, last_id + 1);
                    last_id += 1;
                    last_id
                }
            })
            .collect::<Vec<_>>();
        if pathmap.len() <= slot_idx {
            pathmap.resize_with(slot_idx + 1, std::vec::Vec::new);
        }
        pathmap[slot_idx].extend(paths);
    });
    let svr = idmap.get(SVR);
    let dac = idmap.get(DAC);
    let fft = idmap.get(FFT);
    // if out is the last label
    if pathmap.len() < idmap.len() {
        pathmap.resize_with(idmap.len(), std::vec::Vec::new);
    }
    let p2_indicies = match (svr, dac, fft) {
        (Some(s), Some(d), Some(f)) => Some([*s, *d, *f]),
        _ => None,
    };
    (pathmap, p2_indicies)
}

fn recurse_paths(memo: &mut [usize], pathmap: &[Vec<usize>], curr: usize) -> usize {
    if pathmap[curr].is_empty() {
        return 1;
    } else if memo[curr] != 0 {
        return memo[curr];
    }

    memo[curr] = pathmap[curr]
        .iter()
        .map(|p| recurse_paths(memo, pathmap, *p))
        .sum();
    memo[curr]
}

fn solve(pathmap: &[Vec<usize>]) -> usize {
    let mut memo = vec![0; pathmap.len()];
    recurse_paths(&mut memo, pathmap, YOU)
}

fn recurse_paths2(
    memo: &mut [Option<(usize, [bool; 2])>],
    pathmap: &[Vec<usize>],
    curr: usize,
    dac: usize,
    fft: usize,
    mut seen: [bool; 2],
) -> usize {
    println!("{curr}:{seen:?}");
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

fn solve2(pathmap: &[Vec<usize>], svr: usize, dac: usize, fft: usize) -> usize {
    let mut memo = vec![None; pathmap.len()];
    recurse_paths2(&mut memo, pathmap, svr, dac, fft, [false; 2])
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let (pathmap, p2_req) = parse_input(input.trim());
    let part1 = solve(&pathmap);
    let part2 = if let Some([svr, dac, fft]) = p2_req {
        solve2(&pathmap, svr, dac, fft)
    } else {
        0
    };
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
