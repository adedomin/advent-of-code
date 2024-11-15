use aoc_shared::{parse_to_flat2d, read_input, FlatVec2D};
use std::io;

#[derive(Default, Copy, Clone)]
enum Legend {
    #[default]
    Square,
    Tree,
}

impl From<u8> for Legend {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Self::Tree,
            _ => Self::Square,
        }
    }
}

type Output = FlatVec2D<Legend>;
type Solved = i64;

fn part1_sol(map: &Output, slopes: Vec<(isize, isize)>) -> Solved {
    let mut tot_trees = 1;
    for (xs, ys) in slopes {
        let (mut x, mut y) = (0isize, 0isize);
        let mut curr_trees = 0;
        while (y as usize) < map.2 {
            // FlatVec2D using isize "wraps" around which complies with the problem
            // We just need to early terminate when y > the map.
            if matches!(map[(x, y)], Legend::Tree) {
                curr_trees += 1;
            }
            x += xs;
            y += ys;
        }
        tot_trees *= if curr_trees == 0 { 1 } else { curr_trees };
    }
    tot_trees
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let part1 = part1_sol(&parsed_input, vec![(3, 1)]);
    let part2 = part1_sol(&parsed_input, vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
