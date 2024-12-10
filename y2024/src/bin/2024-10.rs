use aoc_shared::{parse_to_flat2d, read_input, FlatVec2D, Neighbor};
use rustc_hash::FxHashSet;
use std::io;

#[derive(Default, Copy, Clone)]
struct X(u8);

impl From<u8> for X {
    fn from(value: u8) -> Self {
        X(value)
    }
}

type Int = usize;
type Output = FlatVec2D<X>;

fn find_reachable(trails: &mut FlatVec2D<Int>, map: &Output, (x, y): (usize, usize)) -> (Int, Int) {
    let mut uniq = FxHashSet::default();
    fn rec(
        trails: &mut FlatVec2D<Int>,
        uniq: &mut FxHashSet<(usize, usize)>,
        map: &Output,
        (x, y): (usize, usize),
    ) -> Option<(usize, usize)> {
        let cs = map[(x, y)];
        if cs.0 == b'9' {
            uniq.insert((x, y));
            trails[(x, y)] = 1;
            return Some((x, y));
        }

        trails[(x, y)] = map
            .get_neigh_cardinal(x, y)
            .into_iter()
            .flat_map(|Neighbor(slope, x, y)| {
                if cs.0 + 1 == slope.0 {
                    rec(trails, uniq, map, (x, y))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>() // have to collect due to mutable borrow.
            .into_iter()
            .map(|fxy| trails[fxy])
            .sum();

        if trails[(x, y)] > 0 {
            Some((x, y))
        } else {
            None
        }
    }
    let _ = rec(trails, &mut uniq, map, (x, y));
    (uniq.len(), trails[(x, y)])
}

fn solve(map: &Output) -> (Int, Int) {
    let mut trailmap = FlatVec2D::<Int>::new(map.1, map.2);
    map.xyrange().fold((0, 0), |(p1, p2), xy| {
        if map[xy].0 == b'0' {
            let (one, two) = find_reachable(&mut trailmap, map, xy);
            (p1 + one, p2 + two)
        } else {
            (p1, p2)
        }
    })
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let (part1, part2) = solve(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
