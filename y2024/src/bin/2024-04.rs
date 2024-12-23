use aoc_shared::{parse_to_flat2d, read_input, FlatVec2D};
use std::io;

type Output = FlatVec2D<u8>;
type Solved = usize;

#[rustfmt::skip]
const VECTORS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), ( 1, -1),
    (-1,  0),          ( 1,  0),
    (-1,  1), (0,  1), ( 1,  1),
];

fn part1_sol(map: &Output) -> Solved {
    let mut cnt = 0;
    map.xyrange().for_each(|(x, y)| {
        if map[(x, y)] == b'X' {
            VECTORS
                .iter()
                .filter(|(dx, dy)| map.in_bounds(x as isize + dx * 3, y as isize + dy * 3))
                .for_each(|(dx, dy)| {
                    let mut bar = [0; 3];
                    bar.iter_mut().enumerate().for_each(|(i, cell)| {
                        let x = (x as isize + dx * (i + 1) as isize) as usize;
                        let y = (y as isize + dy * (i + 1) as isize) as usize;
                        *cell = map[(x, y)];
                    });
                    if &bar == b"MAS" {
                        cnt += 1;
                    }
                });
        }
    });
    cnt
}

fn is_xmas(bar: &[u8; 3]) -> bool {
    bar == b"MAS" || bar == b"SAM"
}

fn part2_sol(map: &Output) -> Solved {
    let mut cnt = 0;
    // window over all 3x3 boxes
    for y in 0..map.2 - 2 {
        for x in 0..map.1 - 2 {
            let lx = [map[(x, y)], map[(x + 1, y + 1)], map[(x + 2, y + 2)]];
            let rx = [map[(x + 2, y)], map[(x + 1, y + 1)], map[(x, y + 2)]];
            if is_xmas(&lx) && is_xmas(&rx) {
                cnt += 1;
            }
        }
    }
    cnt
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
