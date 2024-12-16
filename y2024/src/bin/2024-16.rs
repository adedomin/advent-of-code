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

macro_rules! push_path {
    ($dij:ident, $pathmap:ident, $key:ident, $cost:ident, $oldkey:ident) => {
        if let Some(t) = $dij.push_equal($key, $cost) {
            let vis = $pathmap.entry($key).or_default();
            if t {
                vis.clear();
                vis.push($oldkey);
            } else {
                vis.push($oldkey);
            }
        }
    };
}

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
    let mut visited = FxHashSet::default();
    let mut part1 = Int::MAX;
    dij.push((sx, sy, 1, 0), 0); // reindeer start easterly
    while let Some(HeapState { key, cost }) = dij.pop() {
        let (x, y, dx, dy) = key;
        // so we don't waste time traversing further... see below comment.
        if cost > part1 {
            break;
        }
        match map[(x, y)] {
            X::Wall => continue,
            X::End => {
                // I don't think it's possible for inputs (and not for example)
                // for there to be unique approaches to end with the *SAME* cost....
                // however, for the sake of my sanity, we keep popping til we see a cost higher than the lowest...
                part1 = cost;

                #[cfg(debug_assertions)]
                let mut debug_map = map.clone();
                let mut stack = vec![key];
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
            }
            _ => (),
        }
        let cwkey = (x, y, -dy, dx);
        let ccwkey = (x, y, dy, -dx);
        let stepkey = (x + dx, y + dy, dx, dy);
        let rotcost = cost + 1000;
        let stepcost = cost + 1;
        push_path!(dij, p2_pathmap, cwkey, rotcost, key);
        push_path!(dij, p2_pathmap, ccwkey, rotcost, key);
        push_path!(dij, p2_pathmap, stepkey, stepcost, key);
    }
    (part1, visited.len() as Int + 1)
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
