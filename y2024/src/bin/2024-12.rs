use aoc_shared::{pad_to_flat2d, read_input, FlatVec2D};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::BinaryHeap, io};

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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Card {
    N,
    E,
    S,
    W,
}

type Output = FlatVec2D<X>;
#[rustfmt::skip]
const CARD: [(Card, isize, isize); 4] = [
                (Card::N, 0, -1),
    (Card::W, -1 ,0),             (Card::E, 1, 0),
                (Card::S, 0,  1)
];

const OUT_OF_BOUNDS: X = X(b'Z' + 1);

type ShapeBoundsSides = (
    FxHashSet<(usize, usize)>,
    FxHashMap<(usize, usize), usize>,
    FxHashMap<(Card, usize), BinaryHeap<usize>>,
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
        (Card, usize),
        BinaryHeap<usize>,
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
                    Card::N | Card::S => segments.entry((card, y)).or_default().push(x),
                    Card::W | Card::E => segments.entry((card, x)).or_default().push(y),
                }
            } else {
                stack.push((nx, ny));
            }
        }
    }
    Some((shape, frontier, segments))
}

fn count_contiguous_lines(points: BinaryHeap<usize>) -> usize {
    points
        .into_sorted_vec()
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
        let area = shape.len();
        let perimeter = bounds.values().sum::<usize>();
        let sides = segments
            .into_values()
            .map(count_contiguous_lines)
            .sum::<usize>();
        p1 += area * perimeter;
        p2 += area * sides;
        stack.extend(bounds.into_keys());
    }
    (p1, p2)
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = pad_to_flat2d(&input, OUT_OF_BOUNDS);
    let (part1, part2) = solve(&parsed_input);
    print!("Part1: {part1}, Part2: {part2}");
    println!();
    Ok(())
}
