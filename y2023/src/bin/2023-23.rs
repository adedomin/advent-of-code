use aoc_shared::{parse_to_flat2d, read_input, FlatVec2D, Neighbor};
use std::io;

type XY = (usize, usize);

#[derive(Debug)]
enum DfsNode {
    Explore { at: XY },
    BackTrack { at: XY, unexplored: Vec<DfsNode> },
}

fn solve(nodes: &FlatVec2D<MazeCell>, start: XY, end: XY, is_part2: bool) -> usize {
    use DfsNode::*;
    let mut ret = 0;
    let mut stack = vec![Explore { at: start }];
    let mut curr_track = FlatVec2D::<bool>::new(nodes.1, nodes.2);
    while let Some(mut dfs) = stack.pop() {
        match &mut dfs {
            Explore { at: (x, y) } => {
                if (*x, *y) == end {
                    if ret < stack.len() {
                        ret = stack.len();
                    }
                    continue;
                } else if curr_track[(*x, *y)] {
                    continue;
                }
                curr_track[(*x, *y)] = true;
                let unexplored = nodes
                    .get_neigh_card_iter((*x, *y))
                    .filter(|Neighbor(n, nx, ny)| n.can_traverse(*nx, *ny, *x, *y, is_part2))
                    .map(|Neighbor(_, nx, ny)| Explore { at: (nx, ny) })
                    .collect::<Vec<DfsNode>>();
                stack.push(BackTrack {
                    unexplored,
                    at: (*x, *y),
                });
            }
            BackTrack { unexplored, at } => {
                if let Some(n) = unexplored.pop() {
                    stack.push(dfs);
                    stack.push(n);
                } else {
                    curr_track[*at] = false;
                }
            }
        }
    }
    ret
}

#[derive(Default, Copy, Clone)]
enum MazeCell {
    #[default]
    Forest,
    Path,
    NSlope,
    ESlope,
    SSlope,
    WSlope,
}

impl std::fmt::Debug for MazeCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MazeCell::Forest => f.write_str("#"),
            MazeCell::Path => f.write_str("."),
            MazeCell::NSlope => f.write_str("^"),
            MazeCell::ESlope => f.write_str(">"),
            MazeCell::SSlope => f.write_str("v"),
            MazeCell::WSlope => f.write_str("<"),
        }
    }
}

impl From<u8> for MazeCell {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Path,
            b'^' => Self::NSlope,
            b'>' => Self::ESlope,
            b'<' => Self::WSlope,
            b'v' => Self::SSlope,
            _ => Self::Forest,
        }
    }
}

impl MazeCell {
    fn can_traverse(&self, sx: usize, sy: usize, lx: usize, ly: usize, is_part2: bool) -> bool {
        match self {
            MazeCell::Forest => false,
            MazeCell::Path => true,
            // can't come from south, thus diff of sy - ly must be negative.
            MazeCell::NSlope => (sy as isize - ly as isize) < 0 || is_part2,
            // can't come from east, thus diff of sx - lx must be greater than 0.
            MazeCell::ESlope => (sx as isize - lx as isize) > 0 || is_part2,
            // can't come from north, thus diff of sy - ly must be greater than 0.
            MazeCell::SSlope => (sy as isize - ly as isize) > 0 || is_part2,
            // can't come from west, thus diff of sx - lx must be less than 0.
            MazeCell::WSlope => (sx as isize - lx as isize) < 0 || is_part2,
        }
    }
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let grid = parse_to_flat2d(&input);
    let part1 = solve(&grid, (1, 0), (grid.1 - 2, grid.2 - 1), false);
    print!("Part1: {part1}, ");
    let part2 = solve(&grid, (1, 0), (grid.1 - 2, grid.2 - 1), true);
    print!("Part2: {part2}");
    println!();
    Ok(())
}
