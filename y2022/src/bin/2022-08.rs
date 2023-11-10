use std::io;
use aoc_shared::{flat_coord_rot, parse_to_flat2d, read_input, FlatVec2D, Rot2D};

#[derive(Clone, Copy, Debug, Default)]
struct Tree(pub u8);

const U8_START_VAL: u8 = b'0' - 1;

impl From<u8> for Tree {
    fn from(value: u8) -> Self {
        Tree(
            value
                .checked_sub(U8_START_VAL)
                .expect("input is not numeric"),
        )
    }
}

// fn neighbors(x: usize, y: usize) -> [(usize, usize); 4] {
//     [(x, y + 1), (x + 1, y), (x, y - 1), (x - 1, y)]
// }

fn solve_p1(trees: &FlatVec2D<Tree>) -> usize {
    let xdim = trees.1;
    let ydim = trees.2;
    let mut vis_map = vec![false; xdim * ydim];

    for rot in [
        Rot2D::None,
        Rot2D::Clock90,
        Rot2D::Clock180,
        Rot2D::Clock270,
    ] {
        // border is always visible.
        for y in 1..trees.2 - 1 {
            let Tree(mut line_max) = trees.0[flat_coord_rot(0, y, xdim, ydim, rot)];
            for x in 1..trees.1 - 1 {
                let Tree(h) = trees.0[flat_coord_rot(x, y, xdim, ydim, rot)];
                if line_max < h {
                    line_max = h;
                    vis_map[flat_coord_rot(x, y, xdim, ydim, rot)] = true;
                }
            }
        }
    }

    // 2 * (l+h)
    let perimeter = 2 * ((xdim - 1) + (ydim - 1));
    vis_map.iter().filter(|&&v| v).count() + perimeter
}

fn solve_p2(trees: &FlatVec2D<Tree>) -> usize {
    let xdim = trees.1;
    let ydim = trees.2;
    let mut scenic_score = vec![1usize; xdim * ydim];

    for rot in [
        Rot2D::None,
        Rot2D::Clock90,
        Rot2D::Clock180,
        Rot2D::Clock270,
    ] {
        for y in 1..trees.2 - 1 {
            for x in 1..trees.1 - 1 {
                let Tree(h) = trees.0[flat_coord_rot(x, y, xdim, ydim, rot)];
                let mut i = x - 1;
                while i != 0 {
                    let Tree(lhs) = trees.0[flat_coord_rot(i, y, xdim, ydim, rot)];
                    if h <= lhs {
                        break;
                    }
                    i -= 1;
                }
                scenic_score[flat_coord_rot(x, y, xdim, ydim, rot)] *= x.abs_diff(i);
            }
        }
    }

    *scenic_score.iter().max().unwrap()
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let trees = parse_to_flat2d::<Tree>(&input);
    let part1 = solve_p1(&trees);
    let part2 = solve_p2(&trees);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
