use aoc_shared::{parse_to_flat2d, read_input, FlatVec2D, Neighbor};
use itertools::Itertools;
use std::io;

#[derive(Default, Copy, Clone)]
enum X {
    #[default]
    Dot,
    Hash,
    Start,
    End,
}

impl From<u8> for X {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Self::Hash,
            b'S' => Self::Start,
            b'E' => Self::End,
            _ => Self::Dot,
        }
    }
}

type Output = FlatVec2D<X>;

const P1_SAVING_LT: isize = 99;
const P2_CHEAT_GT: usize = 20;

fn solve(map: &Output) -> (usize, usize) {
    // find starting params.
    let mut start = None;
    let mut end = None;
    map.xyrange().for_each(|(x, y)| match map[(x, y)] {
        X::Start => start = Some((x as isize, y as isize)),
        X::End => end = Some((x as isize, y as isize)),
        _ => (),
    });
    let (sx, sy) = start.expect("expected a start.");
    let (ex, ey) = end.expect("expected an end.");
    // find start dir
    let (dx, dy) = map
        .get_neigh_cardinal(sx as usize, sy as usize)
        .into_iter()
        .find_map(|Neighbor(t, x, y)| {
            if matches!(t, X::Hash) {
                None
            } else {
                Some(((x as isize) - sx, (y as isize) - sy))
            }
        })
        .expect("A starting vector.");

    let mut distmap = FlatVec2D::<Option<isize>>::new(map.1, map.2);
    // there can be only one path, so just crawl through and find it.
    let mut path = vec![(sx, sy, dx, dy, 0)];
    let mut p1_cheats = vec![];
    // edge case: check if immediately behind me is a cheat.
    let (cdsx, cdsy) = (sx - dx * 2, sy - dy * 2);
    if map.in_bounds(cdsx, cdsy) && !matches!(map[(cdsx, cdsy)], X::Hash) {
        p1_cheats.push(((sx, sy), (cdsx, cdsy)));
    }
    loop {
        let &(x, y, dx, dy, time) = path.last().unwrap();
        distmap[(x as usize, y as usize)].get_or_insert(time);
        if (x, y) == (ex, ey) {
            break;
        }

        [(dy, -dx), (dx, dy), (-dy, dx)]
            .into_iter()
            .filter(|(dx, dy)| map.in_bounds(x + dx, y + dy))
            .for_each(|(dx, dy)| {
                let (cdx, cdy) = (x + dx * 2, y + dy * 2);
                if matches!(map[(x + dx, y + dy)], X::Hash) {
                    if map.in_bounds(cdx, cdy)
                        && !matches!(map[(cdx, cdy)], X::Hash)
                        && distmap[(cdx, cdy)].is_none()
                    // backtracking makes zero sense here.
                    {
                        p1_cheats.push(((x, y), (cdx, cdy)));
                    }
                } else {
                    path.push((x + dx, y + dy, dx, dy, time + 1));
                }
            });
    }
    let p1 = p1_cheats
        .into_iter()
        .filter(|&(start, end)| {
            let s = distmap[start].expect("starting point of cheat HAD to be visted");
            let e = distmap[end]
                .expect("all empty paths should be a part of the unitary path in input");
            ((e - s) - 2) > P1_SAVING_LT
        })
        .count();
    let p2 = path
        .into_iter()
        .tuple_combinations()
        .filter(|((x, y, _, _, _), (x2, y2, _, _, _))| {
            let cheat_dur = x.abs_diff(*x2) + y.abs_diff(*y2);
            if cheat_dur > P2_CHEAT_GT {
                return false;
            }
            // since we can approach these cheats from either side, we'll take the abs diff of the distance
            // tuple_combinations should be unique pairs.
            let s = distmap[(*x, *y)].expect("starting point of cheat HAD to be visted");
            let e = distmap[(*x2, *y2)]
                .expect("all empty paths should be a part of the unitary path in input");
            e.abs_diff(s).abs_diff(cheat_dur) > P1_SAVING_LT as usize
        })
        .count();
    (p1, p2)
}
// fn part2_sol(map: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let (part1, part2) = solve(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}