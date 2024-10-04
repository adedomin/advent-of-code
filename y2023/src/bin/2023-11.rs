use aoc_shared::{array_windows, parse_to_flat2d, read_input, FlatVec2D};
use std::io;

// find geometric median, input is sorted by order of traversal.
fn geomed(points: &[i64]) -> i64 {
    let max = points.len() as i64;
    array_windows(points)
        .enumerate()
        .map(|(p, [xy1, xy2])| {
            let pos = p as i64;
            xy1.abs_diff(*xy2) as i64 * (pos + 1) * (max - pos - 1)
        })
        .sum::<i64>()
}

// fulfills ask of sum of all minimum distances between points via geo median.
// properties of manhattan distance allows for an O(n) sum of of all the X's and Y's distance
// in the map.
fn parse_and_solve(map: &FlatVec2D<u8>, expansion_factor: i64) -> i64 {
    let mut ypoints = vec![];
    let mut last = 0;
    for y in 0..map.2 {
        let mut found = false;
        for x in 0..map.1 {
            let chr = map[(x, y)];
            if chr == b'#' {
                found = true;
                ypoints.push(y as i64 + last);
            }
        }
        if !found {
            last += expansion_factor - 1;
        }
    }

    let mut xpoints = vec![];
    last = 0;
    for x in 0..map.1 {
        let mut found = false;
        for y in 0..map.2 {
            let chr = map[(x, y)];
            if chr == b'#' {
                found = true;
                xpoints.push(x as i64 + last);
            }
        }
        if !found {
            last += expansion_factor - 1;
        }
    }

    geomed(&ypoints) + geomed(&xpoints)
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let map = parse_to_flat2d(&input);

    let part1 = parse_and_solve(&map, 2);
    print!("Part1: {part1}, ");

    let part2 = parse_and_solve(&map, 1_000_000);
    print!("Part2: {part2}");
    println!();
    Ok(())
}
