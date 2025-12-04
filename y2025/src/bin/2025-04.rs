use std::io;

use aoc_shared::{FlatVec2D, Neighbor, parse_to_flat2d, read_input};

#[derive(Copy, Clone, Default)]
enum Fork {
    #[default]
    Floor,
    Paper,
}

impl From<u8> for Fork {
    fn from(value: u8) -> Self {
        match value {
            b'@' => Self::Paper,
            _ => Self::Floor,
        }
    }
}

fn solve(input: &FlatVec2D<Fork>, to_remove: &mut Vec<(usize, usize)>) -> usize {
    input
        .xyrange()
        .filter(|&xy| matches!(input[xy], Fork::Paper))
        .filter(|&xy| {
            input
                .get_neigh_iter(xy)
                .filter(|Neighbor(t, _, _)| matches!(t, Fork::Paper))
                .count()
                < 4
        })
        .inspect(|&xy| to_remove.push(xy))
        .count()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let mut input = parse_to_flat2d(&input);
    let mut to_remove = Vec::with_capacity(input.0.len());
    let part1 = solve(&input, &mut to_remove);
    let mut part2 = part1;
    while !to_remove.is_empty() {
        to_remove.drain(..).for_each(|xy| input[xy] = Fork::Floor);
        part2 += solve(&input, &mut to_remove)
    }
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
