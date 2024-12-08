use aoc_shared::{parse_to_flat2d, read_input, FlatVec2D};
use std::io;

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

// in the line segment, going downslope, antipats only occur 2 distances from starting point.
// this is not the case in part2, however.
const PART1_NUM: Int = 2;
fn get_count(antis: &FlatVec2D<(bool, bool)>) -> (Int, Int) {
    antis.0.iter().fold((0, 0), |(p1, p2), &(p1v, p2v)| {
        (p1 + Int::from(p1v), p2 + Int::from(p2v))
    })
}

fn solve(map: &Output) -> (Int, Int) {
    let mut antis = FlatVec2D::<(bool, bool)>::new(map.1, map.2);
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
                let (dx, dy) = manhattan_slope((lx, ly), (rx, ry));
                // going down, remember we visit each line segment twice.
                for i in 1.. {
                    let (x, y) = (lx as isize + (dx * i), ly as isize + (dy * i));
                    if map.in_bounds(x, y) {
                        (!antis[(x as usize, y as usize)].0 && i as usize == PART1_NUM)
                            .then(|| antis[(x as usize, y as usize)].0 = true);
                        antis[(x as usize, y as usize)].1 = true;
                    } else {
                        break;
                    }
                }
            });
    });
    get_count(&antis)
}

// fn part2_sol(map: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let (part1, part2) = solve(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
