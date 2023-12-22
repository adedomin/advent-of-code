use aoc_shared::{parse_to_flat2d, read_input, FlatVec2D};
use std::{
    collections::{HashMap, VecDeque},
    io,
};

#[derive(Default, Clone, Copy)]
enum Garden {
    Start,
    Rock,
    #[default]
    Plot,
}

impl From<u8> for Garden {
    fn from(value: u8) -> Self {
        match value {
            b'S' => Self::Start,
            b'#' => Self::Rock,
            _ => Self::Plot,
        }
    }
}

// cardinals will go out of bounds.
fn get_card(x: isize, y: isize) -> [(isize, isize); 4] {
    [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
}

fn solve1(grid: &FlatVec2D<Garden>, step_cnt: u32) -> usize {
    let mut start = (0, 0, 1);
    'out: for y in grid.yrange() {
        for x in grid.xrange() {
            if matches!(grid[(x, y)], Garden::Start) {
                start = (x as isize, y as isize, 0);
                break 'out;
            }
        }
    }

    let mut queue = VecDeque::from([start]);
    let mut visited = HashMap::new();
    while let Some((x, y, step)) = queue.pop_back() {
        if *visited.get(&(x, y)).unwrap_or(&0) > 0 || step == step_cnt + 1 {
            continue;
        }
        visited.insert((x, y), step);
        get_card(x, y)
            .into_iter()
            .filter(|(x, y)| matches!(grid[(*x, *y)], Garden::Plot | Garden::Start))
            .for_each(|(x, y)| {
                queue.push_front((x, y, step + 1));
            });
    }

    let is_odd = step_cnt & 1;
    visited.into_iter().fold(0, |acc, ((x, y), step)| {
        let g = grid[(x, y)];

        if step > 0 && step % 2 == is_odd && matches!(g, Garden::Start | Garden::Plot) {
            acc + 1
        } else {
            acc
        }
    })
}

const P2_STEP: u32 = 26_501_365;

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let part1 = solve1(&parsed_input, 64);
    print!("Part1: {part1}, ");
    // x and y bounds are the same.
    let xy_b = parsed_input.1 as u32;
    // we basically need to find the: quadratic internal repetiion
    //                                frontier (boundary, straight and edge) repetion
    // this is only because of how Eric makes the inputs....
    let p2_pat = P2_STEP % xy_b;
    let [r1, r2, r3] = [p2_pat, p2_pat + xy_b, p2_pat + xy_b * 2]
        .into_iter()
        .map(|i| solve1(&parsed_input, i))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let x = (P2_STEP / xy_b) as usize;
    // note the quad part looks suspiciously like triangle number stuff...
    let ax2 = (r1 + r3 - r2 * 2) * (x * (x - 1) / 2);
    let bx = x * (r2 - r1);
    let c = r1;
    let part2 = ax2 + bx + c;
    print!("Part2: {part2}");
    println!();
    Ok(())
}
