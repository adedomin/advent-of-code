use aoc_shared::{pad_to_flat2d, read_input, FlatVec2D};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashSet, io};

#[derive(Default, Copy, Clone)]
enum X {
    #[default]
    Out,
    Dot,
    Junk,
    Guard,
}

impl From<u8> for X {
    fn from(value: u8) -> Self {
        match value {
            b'!' => Self::Out,
            b'.' => Self::Dot,
            b'#' => Self::Junk,
            _ => Self::Guard,
        }
    }
}

type Output = FlatVec2D<X>;
type Solved = usize;

// right turn (clockwise 90deg)
const NINTY_SIN: isize = 1;
const NINTY_COS: isize = 0;
fn rot_right_90(x: isize, y: isize) -> (isize, isize) {
    (x * NINTY_COS - y * NINTY_SIN, x * NINTY_SIN + y * NINTY_COS)
}

fn move_next(x: isize, dx: isize, y: isize, dy: isize) -> (usize, usize) {
    ((x + dx) as usize, (y + dy) as usize)
}

fn patrol(
    map: &Output,
    (sx, sy): (isize, isize),
    (njx, njy): (isize, isize),
) -> Option<HashSet<(isize, isize)>> {
    let (mut x, mut dx, mut y, mut dy) = (sx, 0, sy, -1);
    let mut visited = HashSet::new();
    let mut cycle_det = HashSet::new();
    while !matches!(map[(x, y)], X::Out) {
        if !visited.insert((x, y)) && !cycle_det.insert((x, dx, y, dy)) {
            return None; // contains a cycle.
        }
        /* Handle a case where we may turn 180deg
           e.g.:

          .#.
          .^#
          ...
        */
        for _ in 0..2 {
            let (mx, my) = move_next(x, dx, y, dy);
            if (mx as isize, my as isize) == (njx, njy) || matches!(map[(mx, my)], X::Junk) {
                (dx, dy) = rot_right_90(dx, dy);
            } else {
                break;
            }
        }
        x += dx;
        y += dy;
    }
    Some(visited)
}

fn solve(map: &Output) -> (Solved, Solved) {
    let mut pos = None;
    'out: for y in map.pad_yrange() {
        for x in map.pad_yrange() {
            if matches!(map[(x, y)], X::Guard) {
                pos = Some((x as isize, y as isize));
                break 'out;
            }
        }
    }

    if let Some((x, y)) = pos {
        let part1_visit = patrol(map, (x, y), (-1, -1)).expect("input should be cycle free");
        let p1 = part1_visit.len();
        // sum of all cycling inputs possible from a given visit map.
        let p2 = part1_visit
            .into_par_iter()
            .map(|(njx, njy)| usize::from(patrol(map, (x, y), (njx, njy)).is_none()))
            .sum();
        (p1, p2)
    } else {
        panic!("no guard on map");
    }
}

// fn part2_sol(map: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = pad_to_flat2d(&input, X::Out);
    let (part1, part2) = solve(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
