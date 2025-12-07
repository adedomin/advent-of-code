use std::{collections::HashSet, io};

use aoc_shared::{FlatVec2D, parse_to_flat2d, read_input};

#[derive(Default, Clone, Copy)]
enum Lab {
    #[default]
    Space,
    Splitter,
    Start,
}

impl From<u8> for Lab {
    fn from(value: u8) -> Self {
        match value {
            b'S' => Self::Start,
            b'^' => Self::Splitter,
            _ => Self::Space,
        }
    }
}

fn solve(input: &FlatVec2D<Lab>, (x, mut y): (usize, usize)) -> usize {
    let mut splits = 0;
    let mut xs = HashSet::from([x]);
    let mut xs_swap = HashSet::default();
    while y < input.2 {
        xs.drain().for_each(|x| {
            if let Lab::Splitter = input[(x, y)] {
                xs_swap.extend([x.saturating_sub(1), (x + 1).min(input.1 - 1)]);
                splits += 1;
            } else {
                xs_swap.insert(x);
            }
        });
        std::mem::swap(&mut xs, &mut xs_swap);
        y += 1;
    }
    splits
}

fn split_timeline(
    memo: &mut FlatVec2D<usize>,
    map: &FlatVec2D<Lab>,
    tl: usize,
    (x, y): (usize, usize),
) -> usize {
    if y >= map.2 {
        return tl;
    } else if memo[(x, y)] != 0 {
        return memo[(x, y)];
    }

    let mut ntl = 0;
    if let Lab::Splitter = map[(x, y)] {
        if let Some(lx) = x.checked_sub(1) {
            ntl += split_timeline(memo, map, tl, (lx, y));
        }
        if (x + 1) < map.1 {
            ntl += split_timeline(memo, map, tl, (x + 1, y));
        }
    } else {
        ntl = split_timeline(memo, map, tl, (x, y + 1));
    }

    memo[(x, y)] = ntl;
    ntl
}

fn solve2(input: &FlatVec2D<Lab>, xy: (usize, usize)) -> usize {
    let mut memo = FlatVec2D::new(input.1, input.2);
    split_timeline(&mut memo, input, 1, xy)
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let input = parse_to_flat2d(&input);
    let start = input
        .xyrange()
        .find(|&xy| matches!(input[xy], Lab::Start))
        .expect("No Start on the map.");
    let part1 = solve(&input, start);
    let part2 = solve2(&input, start);
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
