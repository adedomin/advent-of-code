use aoc_shared::{parse_to_flat2d, read_input, FlatVec2D};
use std::io;

#[derive(Default, Copy, Clone)]
enum Seat {
    #[default]
    Floor,
    Occupied,
    Free,
}

impl From<u8> for Seat {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Self::Occupied,
            b'L' => Self::Free,
            _ => Self::Floor,
        }
    }
}

impl Seat {
    fn toggle(&mut self) {
        match self {
            Self::Floor => (),
            Self::Occupied => *self = Self::Free,
            Self::Free => *self = Self::Occupied,
        }
    }
}

type Output = FlatVec2D<Seat>;
type Solved = usize;

fn part1_sol(map: &Output) -> Solved {
    let mut map = map.clone();
    loop {
        let mut changed = vec![];
        for y in map.yrange() {
            for x in map.xrange() {
                if matches!(map[(x, y)], Seat::Floor) {
                    continue;
                }
                let occupied = map
                    .get_neigh_iter((x, y))
                    .filter(|neigh| matches!(neigh.0, Seat::Occupied))
                    .count();
                match map[(x, y)] {
                    Seat::Occupied if occupied > 3 => {
                        changed.push((x, y));
                    }
                    Seat::Free if occupied == 0 => {
                        changed.push((x, y));
                    }
                    _ => (),
                }
            }
        }
        if changed.is_empty() {
            break map
                .0
                .iter()
                .filter(|seat| matches!(seat, Seat::Occupied))
                .count();
        } else {
            changed.into_iter().for_each(|coord| map[coord].toggle())
        }
    }
}

fn find_seat(
    map: &Output,
    (x, y): (isize, isize),
    (xdir, ydir): (isize, isize),
) -> Option<(usize, usize)> {
    let mut x = x;
    let mut y = y;
    loop {
        x += xdir;
        y += ydir;
        if map.in_bounds(x, y) {
            if !matches!(map[(x as usize, y as usize)], Seat::Floor) {
                break Some((x as usize, y as usize));
            }
        } else {
            break None;
        }
    }
}

fn part2_sol(map: &Output) -> Solved {
    let mut vislist: FlatVec2D<Vec<(usize, usize)>> = FlatVec2D::new(map.1, map.2);
    // populate visible list
    for y in map.yrange() {
        for x in map.xrange() {
            if matches!(map[(x, y)], Seat::Floor) {
                continue;
            }
            #[rustfmt::skip]
            let neighdir: [(isize, isize); 8] = [
                (-1, -1), (0, -1), (1, -1),
                (-1,  0),          (1,  0),
                (-1,  1), (0,  1), (1,  1),
            ];
            vislist[(x, y)] = neighdir
                .into_iter()
                .filter_map(|pos| find_seat(map, (x as isize, y as isize), pos))
                .collect::<Vec<(usize, usize)>>();
        }
    }

    let mut map = map.clone();
    loop {
        let mut changed = vec![];
        for y in map.yrange() {
            for x in map.xrange() {
                if matches!(map[(x, y)], Seat::Floor) {
                    continue;
                }
                let occupied = vislist[(x, y)]
                    .iter()
                    .filter(|&&neigh| matches!(map[neigh], Seat::Occupied))
                    .count();
                match map[(x, y)] {
                    Seat::Occupied if occupied > 4 => {
                        changed.push((x, y));
                    }
                    Seat::Free if occupied == 0 => {
                        changed.push((x, y));
                    }
                    _ => (),
                }
            }
        }
        if changed.is_empty() {
            break map
                .0
                .iter()
                .filter(|seat| matches!(seat, Seat::Occupied))
                .count();
        } else {
            changed.into_iter().for_each(|coord| map[coord].toggle())
        }
    }
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
