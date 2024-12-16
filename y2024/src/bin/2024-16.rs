use aoc_shared::{parse_to_flat2d, read_input, Dijkstra, FlatVec2D, HeapState};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{fmt::Write, io};

#[derive(Default, Copy, Clone)]
enum X {
    #[default]
    Dot,
    Wall,
    Start,
    End,
    #[cfg(debug_assertions)]
    DebugPath(char),
}

impl From<u8> for X {
    fn from(value: u8) -> Self {
        match value {
            b'S' => Self::Start,
            b'E' => Self::End,
            b'#' => Self::Wall,
            _ => Self::Dot,
        }
    }
}

impl std::fmt::Debug for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X::Dot => f.write_char('.'),
            X::Wall => f.write_char('#'),
            X::Start => f.write_char('S'),
            X::End => f.write_char('E'),
            #[cfg(debug_assertions)]
            X::DebugPath(c) => f.write_char(*c),
        }
    }
}

#[cfg(debug_assertions)]
fn from_rot((dx, dy): (isize, isize)) -> char {
    match (dx.signum(), dy.signum()) {
        (0, -1) => '^',
        (1, 0) => '>',
        (0, 1) => 'v',
        (-1, 0) => '<',
        _ => unreachable!(),
    }
}

type Int = isize;
type Output = FlatVec2D<X>;
type Key = (Int, Int, Int, Int);

fn part1_sol(map: &Output) -> (Int, Int) {
    let (mut sx, mut sy) = (-1, -1);
    let (mut ex, mut ey) = (-1, -1);
    map.xyrange().for_each(|(x, y)| match map[(x, y)] {
        X::Start => (sx, sy) = (x as isize, y as isize),
        X::End => (ex, ey) = (x as isize, y as isize),
        _ => (),
    });
    if (sx, sy) == (-1, -1) || (ex, ey) == (-1, -1) {
        panic!("No start or end found.");
    }

    let mut dij = Dijkstra::<Key, Int>::new();
    let mut p2_pathmap: std::collections::HashMap<Key, Vec<Key>, rustc_hash::FxBuildHasher> =
        FxHashMap::default();
    dij.push((sx, sy, 1, 0), 0); // reindeer start easterly
    while let Some(HeapState { key, cost }) = dij.pop() {
        let (x, y, dx, dy) = key;
        match map[(x, y)] {
            X::Wall => continue,
            X::End => {
                #[cfg(debug_assertions)]
                let mut debug_map = map.clone();
                let mut stack = vec![key];
                let mut visited = FxHashSet::default();
                while let Some(key) = stack.pop() {
                    if let Some(keys) = p2_pathmap.get(&key) {
                        keys.iter().for_each(|&(x, y, _dx, _dy)| {
                            #[cfg(debug_assertions)]
                            {
                                debug_map[(x as usize, y as usize)] =
                                    X::DebugPath(from_rot((_dx, _dy)));
                            }
                            visited.insert((x, y));
                        });
                        stack.extend(keys);
                    }
                }
                #[cfg(debug_assertions)]
                println!("{debug_map:?}");
                return (cost, visited.len() as Int + 1);
            }
            _ => (),
        }
        let cwkey = (x, y, -dy, dx);
        let ccwkey = (x, y, dy, -dx);
        let stepkey = (x + dx, y + dy, dx, dy);
        if dij.push_equal(cwkey, cost + 1000).is_some() {
            p2_pathmap.entry(cwkey).or_default().push(key);
        }
        if dij.push_equal(ccwkey, cost + 1000).is_some() {
            p2_pathmap.entry(ccwkey).or_default().push(key);
        }
        if dij.push_equal(stepkey, cost + 1).is_some() {
            p2_pathmap.entry(stepkey).or_default().push(key);
        }
    }
    panic!("no solution");
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let (part1, part2) = part1_sol(&parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
