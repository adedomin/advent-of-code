use aoc_shared::{parse_to_flat2d, read_input, FlatVec2D};
use itertools::Itertools;
use std::io;

fn expand_universe(
    p1: (i64, i64),
    horizon: &[i64],
    vertical: &[i64],
    expansion_factor: i64,
) -> (i64, i64) {
    let x = p1.0 as usize;
    let y = p1.1 as usize;
    let tv = (expansion_factor - 1) * vertical[x];
    let th = (expansion_factor - 1) * horizon[y];

    (p1.0 + tv, p1.1 + th)
}

fn adjust_pairs(p1: (i64, i64), p2: (i64, i64)) -> ((i64, i64), (i64, i64)) {
    if p1 < p2 {
        (p1, p2)
    } else {
        (p2, p1)
    }
}

fn parse(map: &FlatVec2D<u8>, expansion_factor: i64) -> Vec<((i64, i64), (i64, i64))> {
    let mut galaxies = vec![];
    let mut hori = vec![0; map.1];
    let mut last = 0;
    for y in 0..map.2 {
        let mut found = false;
        for x in 0..map.1 {
            let chr = map[(x, y)];
            if chr == b'#' {
                found = true;
                galaxies.push((x as i64, y as i64));
            }
        }
        if !found {
            last += 1;
        }
        hori[y] = last;
    }

    let mut vert = vec![0; map.2];
    last = 0;
    for x in 0..map.1 {
        let mut found = false;
        for y in 0..map.2 {
            let chr = map[(x, y)];
            if chr == b'#' {
                found = true;
            }
        }
        if !found {
            last += 1;
        }
        vert[x] = last;
    }

    let galpair = galaxies
        .into_iter()
        .map(|gal| expand_universe(gal, &hori, &vert, expansion_factor))
        .permutations(2)
        .map(|perm| adjust_pairs(perm[0], perm[1]))
        .unique()
        .collect::<Vec<((i64, i64), (i64, i64))>>();
    galpair
}

fn solve(galaxy_pairs: Vec<((i64, i64), (i64, i64))>) -> u64 {
    galaxy_pairs
        .into_iter()
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(x2) + y1.abs_diff(y2))
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let map = parse_to_flat2d(&input);

    let pair1 = parse(&map, 2);
    let part1 = solve(pair1);
    print!("Part1: {part1}, ");

    let pair2 = parse(&map, 1_000_000);
    let part2 = solve(pair2);
    print!("Part2: {part2}");
    println!();
    Ok(())
}
