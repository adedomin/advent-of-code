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
fn solve(map: &Output) -> (Int, Int) {
    let mut antis = FlatVec2D::<bool>::new(map.1, map.2);
    let mut antis_p2 = FlatVec2D::<bool>::new(map.1, map.2);
    let fmap = |(x, y)| match map[(x, y)] {
        X::Dot => None,
        X::Hash => None,
        X::Antenna(label) => Some((label, (x, y))),
    };

    let mut antennas = vec![vec![]; u8::MAX as usize];
    map.xyrange()
        .filter_map(fmap)
        .for_each(|(label, coord)| antennas[label as usize].push(coord));

    antennas
        .into_iter()
        .filter(|a| !a.is_empty())
        .for_each(|atn_t| {
            atn_t.iter().for_each(|&a| {
                atn_t.iter().for_each(|&b| {
                    if a == b {
                        return;
                    }

                    let (dx, dy) = manhattan_slope(a, b);
                    // going down, remember we visit each line segment twice.
                    for i in 1.. {
                        let (x, y) = (a.0 as isize + (dx * i), a.1 as isize + (dy * i));
                        if map.in_bounds(x, y) {
                            if i as usize == PART1_NUM {
                                antis[(x as usize, y as usize)] = true;
                            }
                            antis_p2[(x as usize, y as usize)] = true;
                        } else {
                            break;
                        }
                    }
                });
            });
        });

    (
        antis.0.into_iter().map(Int::from).sum(),
        antis_p2.0.into_iter().map(Int::from).sum(),
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
