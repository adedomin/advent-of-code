use aoc_shared::{
    fold_decimal_from, read_input_to_string, Dijkstra, FlatVec2D, HeapState, Neighbor,
};
use itertools::Itertools;
use std::{fmt::Write, io, ops::Not};

type Int = usize;
type Key = (Int, Int);
type Output = Vec<Key>;

#[derive(Default, Clone, Copy)]
enum X {
    #[default]
    Dot,
    Hash,
}

impl std::fmt::Debug for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X::Dot => f.write_char('.'),
            X::Hash => f.write_char('#'),
        }
    }
}

fn parse_input(input: &str) -> Output {
    input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|&num| num.is_empty().not())
        .map(|num| fold_decimal_from(num.as_bytes()))
        .tuples()
        .collect::<Output>()
}

fn part1_sol(corrupt: &Output, (bx, by): (Int, Int), corr_amt: Int) -> Option<Int> {
    let mut map = FlatVec2D::<X>::new(bx, by);
    corrupt.iter().take(corr_amt).for_each(|&xy| {
        if let Some(cell) = map.get_mut(xy) {
            *cell = X::Hash
        }
    });
    #[cfg(debug_assertions)]
    println!("{map:?}");
    let target = (bx - 1, by - 1);
    let mut dij = Dijkstra::<Key, Int>::new();
    dij.push((0, 0), 0);
    while let Some(HeapState { key: (x, y), cost }) = dij.pop() {
        // corrupt, can't move here.
        if matches!(map[(x, y)], X::Hash) {
            continue;
        } else if (x, y) == target {
            return Some(cost);
        }

        map.get_neigh_cardinal(x, y)
            .into_iter()
            .for_each(|Neighbor(_, nx, ny)| {
                dij.push((nx, ny), cost + 1);
            });
    }
    None
}

// const EXBOUND: Int = 7;
const P1BOUND: Int = 71;

// const EXCORR: Int = 12;
const P1CORR: Int = 1024;

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 =
        part1_sol(&parsed_input, (P1BOUND, P1BOUND), P1CORR).expect("Solution for 1024 bytes.");
    print!("Part1: {part1}, ");
    let part2 = (P1CORR..parsed_input.len())
        .collect::<Vec<_>>()
        .binary_search_by(|&v| {
            if part1_sol(&parsed_input, (P1BOUND, P1BOUND), v).is_some() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
        .unwrap_err()
        + P1CORR;
    let (p2x, p2y) = if part1_sol(&parsed_input, (P1BOUND, P1BOUND), part2).is_some() {
        parsed_input[part2]
    } else {
        parsed_input[part2 - 1]
    };

    print!("Part2: {p2x},{p2y}");
    println!();
    Ok(())
}
