use aoc_shared::{pad_to_flat2d, read_input, FlatVec2D};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::io;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
struct X(u8);

impl From<u8> for X {
    fn from(value: u8) -> Self {
        match value {
            b'A'..=b'Z' => Self(value - b'A'),
            _ => OUT_OF_BOUNDS,
        }
    }
}

const N: u8 = 0;
const E: u8 = 1;
const S: u8 = 2;
const W: u8 = 3;

type Output = FlatVec2D<X>;
#[rustfmt::skip]
const CARD: [(u8, isize, isize); 4] = [
                (N, 0, -1),
    (W, -1 ,0),             (E, 1, 0),
                (S, 0,  1)
];

const OUT_OF_BOUNDS: X = X(b'Z' + 1);

type ShapeBoundsSides = (
    FxHashSet<(usize, usize)>,
    FxHashMap<(usize, usize), usize>,
    FxHashMap<(u8, usize), Vec<usize>>,
);

fn get_shape(
    map: &Output,
    visit: &mut FlatVec2D<bool>,
    (x, y): (usize, usize),
) -> Option<ShapeBoundsSides> {
    if visit[(x, y)] || map[(x, y)] == OUT_OF_BOUNDS {
        return None;
    }

    let mut shape = FxHashSet::default();
    let mut frontier = FxHashMap::default();
    let mut segments: std::collections::HashMap<
        (u8, usize),
        Vec<usize>,
        rustc_hash::FxBuildHasher,
    > = FxHashMap::default();
    let mut stack = vec![(x, y)];
    while let Some((x, y)) = stack.pop() {
        visit[(x, y)] = true;
        if !shape.insert((x, y)) {
            continue;
        }

        let c = map[(x, y)];
        for (card, dx, dy) in CARD {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            let n = map[(nx, ny)];

            if n != c {
                *frontier.entry((nx, ny)).or_default() += 1;
                match card {
                    N | S => segments.entry((card, y)).or_default().push(x),
                    W | E => segments.entry((card, x)).or_default().push(y),
                    _ => unreachable!(),
                }
            } else {
                stack.push((nx, ny));
            }
        }
    }
    Some((shape, frontier, segments))
}

fn count_contiguous_lines(mut points: Vec<usize>) -> usize {
    points.sort_unstable();
    points
        .into_iter()
        .coalesce(|x, y| if x + 1 == y { Ok(y) } else { Err((x, y)) })
        .count()
}

fn solve(map: &Output) -> (usize, usize) {
    let mut visit = FlatVec2D::<bool>::new(map.1, map.2);
    let mut p1 = 0;
    let mut p2 = 0;
    let mut stack = vec![(1, 1)];
    while let Some((x, y)) = stack.pop() {
        let Some((shape, bounds, segments)) = get_shape(map, &mut visit, (x, y)) else {
            continue;
        };
        p1 += shape.len() * bounds.values().sum::<usize>();
        let sides = segments
            .into_values()
            .map(count_contiguous_lines)
            .sum::<usize>();
        p2 += shape.len() * sides;
        stack.extend(bounds.into_keys());
    }
    (p1, p2)
}

// fn part2_sol(map: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = pad_to_flat2d(&input, OUT_OF_BOUNDS);
    let (part1, part2) = solve(&parsed_input);
    print!("Part1: {part1}, Part2: {part2}");
    println!();
    Ok(())
}
