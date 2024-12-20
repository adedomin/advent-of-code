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

const P1_SAVING_MIN: usize = 100;
const P2_CHEAT_LIM: usize = 21;

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

    let mut distmap = FlatVec2D::<usize>::new(map.1, map.2);
    // there can be only one path, so just crawl through and find it.
    let mut path = vec![(sx, sy, dx, dy, 0)];
    loop {
        let &(x, y, dx, dy, time) = path.last().unwrap();
        distmap[(x as usize, y as usize)] = time;
        if (x, y) == (ex, ey) {
            break;
        }

        [(dy, -dx), (dx, dy), (-dy, dx)]
            .into_iter()
            .filter(|(dx, dy)| map.in_bounds(x + dx, y + dy))
            .find(|&(dx, dy)| {
                if !matches!(map[(x + dx, y + dy)], X::Hash) {
                    path.push((x + dx, y + dy, dx, dy, time + 1));
                    true
                } else {
                    false
                }
            });
    }
    path.into_iter().tuple_combinations().fold(
        (0, 0),
        |(p1, p2), ((x1, y1, _, _, _), (x2, y2, _, _, _))| {
            // manhattan distance.
            let cheat_dur = x1.abs_diff(x2) + y1.abs_diff(y2);
            // can't have a cheat of "one" it is meaningless for p1 or 2.
            if !(2..P2_CHEAT_LIM).contains(&cheat_dur) {
                return (p1, p2);
            }
            // Since we can approach these cheats from either side, we'll take the absolute value.
            // The absolute value of the distance should be the "optimal" pick since the negative case
            // implies we went backwards and ADDED time taking this cheat.
            //
            // tuple_combinations should be unique pairs, so no worries of dupes.
            let s = distmap[(x1, y1)];
            let e = distmap[(x2, y2)];
            if e.abs_diff(s).abs_diff(cheat_dur) >= P1_SAVING_MIN {
                (p1 + if cheat_dur == 2 { 1 } else { 0 }, p2 + 1)
            } else {
                (p1, p2)
            }
        },
    )
}
// fn part2_sol(map: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let (part1, part2) = solve(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
