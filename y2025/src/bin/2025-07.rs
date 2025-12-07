use std::io;

use aoc_shared::{FlatVec2D, parse_to_flat2d, read_input};

#[derive(Default, Clone, Copy)]
enum Lab {
    #[default]
    Space,
    Splitter,
    Start,
}

impl From<u8> for Lab {
    fn from(value: u8) -> Self {
        match value {
            b'S' => Self::Start,
            b'^' => Self::Splitter,
            _ => Self::Space,
        }
    }
}

fn solve(input: &FlatVec2D<Lab>, (sx, sy): (usize, usize)) -> (usize, usize) {
    let mut splits = 0;
    let mut timelines = vec![0; input.1];
    timelines[sx] = 1;

    for y in sy..input.2 {
        for x in input.xrange() {
            if timelines[x] != 0 && matches!(input[(x, y)], Lab::Splitter) {
                splits += 1;
                let t = std::mem::take(&mut timelines[x]);
                _ = [x.checked_sub(1), (x + 1 < input.1).then_some(x + 1)]
                    .map(|x| x.map(|x| timelines[x] += t));
            }
        }
    }

    (splits, timelines.into_iter().sum())
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let input = parse_to_flat2d(&input);
    let start = input
        .xyrange()
        .find(|&xy| matches!(input[xy], Lab::Start))
        .expect("No Start on the map.");
    let (part1, part2) = solve(&input, start);
    // let part2 = solve2(&input, start);
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
