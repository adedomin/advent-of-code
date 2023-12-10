use aoc_shared::{array_windows, pad_to_flat2d, read_input, FlatVec2D};

use std::{fmt::Write, io};

#[derive(Clone, Copy)]
enum Pipe {
    Ground,
    Vert,
    Hori,
    NE90,
    NW90,
    SW90,
    SE90,
    Start,
}

enum Cardinal {
    North,
    East,
    South,
    West,
}

impl Pipe {
    fn enter_exit(&self, dir: Cardinal, x: usize, y: usize) -> Option<(Cardinal, usize, usize)> {
        match (dir, self) {
            (Cardinal::North, Self::Vert) => Some((Cardinal::North, x, y - 1)),
            (Cardinal::North, Self::SE90) => Some((Cardinal::East, x + 1, y)),
            (Cardinal::North, Self::SW90) => Some((Cardinal::West, x - 1, y)),

            (Cardinal::South, Self::Vert) => Some((Cardinal::South, x, y + 1)),
            (Cardinal::South, Self::NE90) => Some((Cardinal::East, x + 1, y)),
            (Cardinal::South, Self::NW90) => Some((Cardinal::West, x - 1, y)),

            (Cardinal::East, Self::Hori) => Some((Cardinal::East, x + 1, y)),
            (Cardinal::East, Self::NW90) => Some((Cardinal::North, x, y - 1)),
            (Cardinal::East, Self::SW90) => Some((Cardinal::South, x, y + 1)),

            (Cardinal::West, Self::Hori) => Some((Cardinal::West, x - 1, y)),
            (Cardinal::West, Self::NE90) => Some((Cardinal::North, x, y - 1)),
            (Cardinal::West, Self::SE90) => Some((Cardinal::South, x, y + 1)),
            _ => None,
        }
    }

    fn is_bent(&self) -> bool {
        match self {
            Pipe::NE90 | Pipe::NW90 | Pipe::SW90 | Pipe::SE90 => true,
            _ => false,
        }
    }
}

impl Default for Pipe {
    fn default() -> Self {
        Self::Ground
    }
}

impl std::fmt::Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => f.write_char('.'),
            Self::Vert => f.write_char('║'),
            Self::Hori => f.write_char('═'),
            Self::NE90 => f.write_char('╚'),
            Self::NW90 => f.write_char('╝'),
            Self::SW90 => f.write_char('╗'),
            Self::SE90 => f.write_char('╔'),
            Self::Start => f.write_char('S'),
        }
    }
}

impl From<u8> for Pipe {
    fn from(value: u8) -> Self {
        match value {
            b'|' => Self::Vert,
            b'-' => Self::Hori,
            b'L' => Self::NE90,
            b'J' => Self::NW90,
            b'7' => Self::SW90,
            b'F' => Self::SE90,
            b'S' => Self::Start,
            _ => Self::Ground,
        }
    }
}

fn solve(grid: &FlatVec2D<Pipe>) -> (i64, i64) {
    use Cardinal::*;
    let mut start = (usize::MAX, usize::MAX);
    'out: for y in 0..grid.2 {
        for x in 0..grid.1 {
            if matches!(grid[(x, y)], Pipe::Start) {
                start = (x, y);
                break 'out;
            }
        }
    }

    if start == (usize::MAX, usize::MAX) {
        panic!("No start found in input grid");
    }

    let (sx, sy) = start;

    #[rustfmt::skip]
    let mut stack = vec![
                      (North, sx, sy - 1, 1, vec![]),
        (West, sx - 1, sy, 1, vec![]), (East, sx + 1, sy, 1, vec![]),
                      (South, sx, sy + 1, 1, vec![]),
    ];

    let start_is_bent = match [
        grid[(sx, sy - 1)],
        grid[(sx - 1, sy)],
        grid[(sx + 1, sy)],
        grid[(sx, sy + 1)],
    ] {
        [Pipe::Vert, _, _, Pipe::Vert] => false,
        [_, Pipe::Hori, Pipe::Hori, _] => false,
        _ => true,
    };

    let mut acost = i64::MAX;
    let mut area = vec![];
    while let Some((card, x, y, cost, mut points)) = stack.pop() {
        let pipe = grid[(x, y)];
        if let Some((ncard, nx, ny)) = pipe.enter_exit(card, x, y) {
            if pipe.is_bent() {
                points.push((x as i64, y as i64));
            }
            stack.push((ncard, nx, ny, cost + 1, points));
        } else if matches!(pipe, Pipe::Start) {
            acost = cost / 2 + (cost & 1);
            if start_is_bent {
                area.push((sx as i64, sy as i64));
                area.extend_from_slice(&points[..]);
                area.push((sx as i64, sy as i64));
            }
            break;
        }
    }

    // shoelace algorithm - (perimiter / 2) since this is a "closed" polygon the perimeter does not count towards area.
    let area = array_windows(&area)
        .map(|&[(x1, y1), (x2, y2)]| x1 * y2 - x2 * y1)
        .sum::<i64>()
        .abs()
        / 2
        - acost
        + 1;

    (acost, area)
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let grid: FlatVec2D<Pipe> = pad_to_flat2d(&input, Pipe::Ground);
    let (part1, part2) = solve(&grid);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
