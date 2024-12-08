use aoc_shared::{debug, parse_to_flat2d, read_input, FlatVec2D};
use std::{collections::HashSet, io};

#[derive(Default, Copy, Clone, Eq, PartialEq)]
enum X {
    #[default]
    Dot,
    Hash,
    Antenna(u8),
}

impl From<u8> for X {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Dot,
            b'#' => Self::Hash,
            _ => Self::Antenna(value),
        }
    }
}

type Int = usize;
type Output = FlatVec2D<X>;

fn manhattan_slope(lhs: (usize, usize), rhs: (usize, usize)) -> (isize, isize) {
    (
        rhs.0 as isize - lhs.0 as isize,
        rhs.1 as isize - lhs.1 as isize,
    )
}

fn y_inter(lhs: (usize, usize)) -> isize {
    -(lhs.0 as isize) - -(lhs.1 as isize)
}

fn get_count(antis: &FlatVec2D<bool>) -> Int {
    antis.0.iter().map(|&v| Int::from(v)).sum()
}

fn solve(map: &Output) -> (Int, Int) {
    let mut antis = FlatVec2D::<bool>::new(map.1, map.2);
    let mut antis_p2 = FlatVec2D::<bool>::new(map.1, map.2);
    let fmap = |(x, y)| match map[(x, y)] {
        X::Dot => None,
        X::Hash => None,
        X::Antenna(label) => Some((label, x, y)),
    };
    map.xyrange().filter_map(fmap).for_each(|(label, lx, ly)| {
        map.xyrange()
            .filter_map(|xy| {
                let X::Antenna(label2) = map[xy] else {
                    return None;
                };
                if label == label2 && (lx, ly) != xy {
                    fmap(xy)
                } else {
                    None
                }
            })
            .for_each(|(_, rx, ry)| {
                let (sx, sy) = manhattan_slope((lx, ly), (rx, ry));
                let (mut atx, mut aty) = (lx as isize + -sx, ly as isize + -sy);
                let (mut abx, mut aby) = (lx as isize + (sx * 2), ly as isize + (sy * 2));

                // part 1
                if let Some(m) = antis.get_isize_mut((atx, aty)) {
                    *m = true;
                }
                if let Some(m) = antis.get_isize_mut((abx, aby)) {
                    *m = true;
                }

                // part 2 going up
                while map.in_bounds(atx, aty) {
                    antis_p2[(atx as usize, aty as usize)] = true;
                    (atx, aty) = (atx + -sx, aty + -sy);
                }

                // part 2 going down
                // atennas are also antis in p2, roll it back one
                (abx, aby) = (abx + -sx, aby + -sy);
                while map.in_bounds(abx, aby) {
                    antis_p2[(abx as usize, aby as usize)] = true;
                    (abx, aby) = (abx + sx, aby + sy);
                }
            });
    });
    (get_count(&antis), get_count(&antis_p2))
}

// fn part2_sol(map: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let (part1, part2) = solve(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
