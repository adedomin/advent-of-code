use aoc_shared::{parse_to_flat2d, read_input_to_string, FlatVec2D};
use rustc_hash::FxHashSet;
use std::{fmt::Write, io};

#[derive(Default, Copy, Clone)]
enum X {
    #[default]
    Wall,
    Box,
    VBox,
    Robot,
    Dot,
}

impl std::fmt::Debug for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X::Wall => f.write_char('#'),
            X::Box => f.write_char('['),
            X::VBox => f.write_char(']'),
            X::Robot => f.write_char('@'),
            X::Dot => f.write_char('.'),
        }
    }
}

impl From<u8> for X {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Self::Wall,
            b'@' => Self::Robot,
            b'O' => Self::Box,
            b'[' => Self::Box,
            b']' => Self::VBox,
            _ => Self::Dot,
        }
    }
}

impl X {
    fn to_wide(self) -> [Self; 2] {
        match self {
            X::Wall => [self; 2],
            X::Box => [X::Box, X::VBox],
            X::Robot => [self, X::Dot],
            _ => [X::Dot; 2],
        }
    }
}

#[rustfmt::skip]
const CARD: [(isize, isize); 4] = [
             (0, -1),
    (-1 ,0),          (1, 0),
             (0,  1)
];

fn from_move(byte: &u8) -> Option<usize> {
    match byte {
        b'^' => Some(0),
        b'<' => Some(1),
        b'>' => Some(2),
        b'v' => Some(3),
        _ => None,
    }
}

type Int = usize;
type Output = FlatVec2D<X>;
type Moves = Vec<usize>;

fn solve(mut grid: Output, moves: &Moves, part2: bool) -> Int {
    let (mut x, mut y) = grid
        .xyrange()
        .find(|&xy| matches!(grid[xy], X::Robot))
        .expect("No Robot found.");

    let mut stack = vec![];
    // filters out visited.
    let mut moveset = FxHashSet::default();
    // swap pairs
    let mut swap_pairs = vec![];
    moves.iter().for_each(|&card| {
        #[cfg(debug_assertions)]
        println!("{card:?}\n{grid:?}");
        let (dx, dy) = CARD[card];
        stack.clear();
        stack.push((x, y));
        moveset.clear();
        swap_pairs.clear();
        while let Some((cx, cy)) = stack.pop() {
            let (nx, ny) = ((cx as isize + dx) as usize, (cy as isize + dy) as usize);
            if moveset.insert((cx, cy)) {
                match grid[(nx, ny)] {
                    X::Wall => return, // can't move.
                    // part1 has a box of width 1, vs box of width 2
                    X::Box => stack.extend(&[(nx, ny), (nx + 1, ny)][..if part2 { 2 } else { 1 }]),
                    // will never show in p1.
                    X::VBox => stack.extend(&[(nx - 1, ny), (nx, ny)]),
                    _ => (),
                }
                // cancel out the non-moving vector, invert direction so we swap in right order.
                swap_pairs.push((cx as isize * -dx + cy as isize * -dy, (cx, cy), (nx, ny)));
            }
        }
        swap_pairs.sort_unstable_by_key(|&(pos, _, _)| pos);
        swap_pairs
            .drain(..)
            .for_each(|(_, xy, nxy)| grid.swap(xy, nxy));
        x = (x as isize + dx) as usize;
        y = (y as isize + dy) as usize;
    });

    grid.xyrange()
        .filter(|&xy| matches!(grid[xy], X::Box))
        .map(|(x, y)| 100 * y + x)
        .sum::<Int>()
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let (grid, moves) = input
        .split_once("\n\n")
        .expect("Grid to be followed by moves after two new lines");

    let grid: FlatVec2D<X> = parse_to_flat2d(grid.as_bytes());
    let mut grid_p2 = FlatVec2D::<X>::new(grid.1 * 2, grid.2);
    grid.0
        .iter()
        .enumerate()
        .for_each(|(i, x)| [grid_p2.0[i * 2], grid_p2.0[i * 2 + 1]] = x.to_wide());
    let moves = moves
        .as_bytes()
        .iter()
        .filter_map(from_move)
        .collect::<Moves>();

    let part1 = solve(grid, &moves, false);
    let part2 = solve(grid_p2, &moves, true);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
