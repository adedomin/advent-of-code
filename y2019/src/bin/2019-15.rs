use std::{collections::HashSet, io};

use aoc_shared::{Dijkstra, HeapState};
use y2019::intcode::{read_intcode, IntCode};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Dir {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

const DIRS: [(Dir, i32, i32); 4] = [
    (Dir::North, 0, -1),
    (Dir::South, 0, 1),
    (Dir::West, -1, 0),
    (Dir::East, 1, 0),
];

#[derive(Clone, Copy, Debug)]
enum Status {
    Wall = 0,
    Moved = 1,
    AtLoc = 2,
}

impl TryFrom<i64> for Status {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Wall,
            1 => Self::Moved,
            2 => Self::AtLoc,
            _ => return Err(()),
        })
    }
}

fn discover_map(program: Vec<i64>) -> (HashSet<(i32, i32)>, (i32, i32)) {
    let mut target = (i32::MIN, i32::MIN);
    let mut visit = HashSet::from([(0, 0)]);

    // Yes, I am effectively cloning the machine (incl memory) for each branching action.
    // This reduces the complexity of managing backtracking in a singular intcode machine.
    // This comes with the cost of extra memory.
    let mut stack = vec![(0, 0, IntCode::default(), program)];
    while let Some((x, y, intcode, prog)) = stack.pop() {
        for (d, dx, dy) in DIRS {
            let (newx, newy) = (x + dx, y + dy);
            let mut ic = intcode;
            let mut nprog = prog.clone();
            let out: Status = ic
                .execute_til(&mut nprog, &mut Some(d as i64))
                .expect("No error")
                .try_into()
                .expect("valid status");
            match out {
                Status::Wall => (),
                Status::AtLoc => target = (newx, newy),
                Status::Moved => {
                    if visit.insert((newx, newy)) {
                        stack.push((newx, newy, ic, nprog));
                    }
                }
            }
        }
    }
    (visit, target)
}

fn min_path(map: &HashSet<(i32, i32)>, target: (i32, i32)) -> u32 {
    type Key = (i32, i32);
    type Cost = u32;
    let mut dij = Dijkstra::<Key, Cost>::new();
    dij.push((0, 0), 0);
    while let Some(HeapState { key, cost }) = dij.pop() {
        if key == target {
            return cost;
        } else if !map.contains(&key) {
            continue;
        }
        DIRS.iter()
            .map(|&(_, dx, dy)| (key.0 + dx, key.1 + dy))
            .for_each(|key| {
                dij.push(key, cost + 1);
            });
    }
    panic!("No Answer!");
}

fn oxygenate_map(map: HashSet<(i32, i32)>, start: (i32, i32)) -> u32 {
    let mut flood_time = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::default();

    let mut frontier = vec![];
    let mut oxygenating = vec![start];

    // each slice of time oxygenates new areas of the map.
    // so we store the frontier nodes in the `next` vec.
    // after collecting all frontier, we swap, increment answer and loop.
    loop {
        for (x, y) in oxygenating.drain(..) {
            for (_, dx, dy) in DIRS {
                let new = (x + dx, y + dy);
                if !map.contains(&new) {
                    continue;
                }
                if visited.insert(new) {
                    frontier.push(new);
                }
            }
        }
        if frontier.is_empty() {
            break flood_time;
        } else {
            flood_time += 1;
            std::mem::swap(&mut frontier, &mut oxygenating);
        }
    }
}

fn main() -> io::Result<()> {
    let program = read_intcode()?;
    let (map, oxy_sys) = discover_map(program);
    let part1 = min_path(&map, oxy_sys);
    let part2 = oxygenate_map(map, oxy_sys);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
