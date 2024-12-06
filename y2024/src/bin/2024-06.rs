use aoc_shared::{flat_coord, inverse_flat_coord, pad_to_flat2d, read_input, FlatVec2D};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::io;

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
// note that cos(90deg) is always ~0
// note that sin(90deg) is always ~1
const fn rot_right_90((x, y): (isize, isize)) -> (isize, isize) {
    (-y, x)
}
const ROT_TAB: [(isize, isize); 4] = {
    let mut ret = [(0, -1); 4];
    let mut i = 1;
    while i < ret.len() {
        ret[i] = rot_right_90(ret[i - 1]);
        i += 1;
    }
    ret
};

fn move_next(x: isize, dx: isize, y: isize, dy: isize) -> (usize, usize) {
    ((x + dx) as usize, (y + dy) as usize)
}

fn patrol(map: &Output, (sx, sy): (isize, isize), njxy: (isize, isize)) -> Option<Vec<u8>> {
    let mut rot_ind = 0u8;
    let (mut dx, mut dy) = ROT_TAB[rot_ind as usize];
    let (mut x, mut y) = (sx, sy);
    let mut visited = vec![0; map.1 * map.2];
    if (sx, sy) == njxy {
        return Some(visited); // can't start here so not a valid loop
    }
    loop {
        /* Handle a case where we may turn 180deg
           e.g.:

          .#.
          .^#
          ...
        */
        for _ in 0..2 {
            let (mx, my) = move_next(x, dx, y, dy);
            if (mx as isize, my as isize) == njxy || matches!(map[(mx, my)], X::Junk) {
                rot_ind = (rot_ind + 1) % 4;
                (dx, dy) = ROT_TAB[rot_ind as usize];
            } else {
                break;
            }
        }
        x += dx;
        y += dy;
        if matches!(map[(x, y)], X::Out) {
            break;
        }
        let vmap = flat_coord(x as usize, y as usize, map.1);
        (visited[vmap] & (1 << rot_ind) == 0).then(|| visited[vmap] |= 1 << rot_ind)?;
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
        let p1 = part1_visit.iter().filter(|&&v| v != 0).count();
        // sum of all cycling inputs possible from a given visit map.
        let p2 = part1_visit
            .into_par_iter()
            .enumerate()
            .filter(|(_, v)| *v != 0)
            .map(|(idx, _)| {
                let (njx, njy) = inverse_flat_coord(idx, map.1);
                usize::from(patrol(map, (x, y), (njx as isize, njy as isize)).is_none())
            })
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
