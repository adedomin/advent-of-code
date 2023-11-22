use aoc_shared::{advanced_cli, parse_to_flat2d, FlatVec2D, Neighbor};
use std::io::{self, Write};

fn part1_sol(
    iter: u16,
    stuck_corners: bool,
    writable: &mut Option<impl Write>,
    mut input: FlatVec2D<u8>,
) -> usize {
    let xmax = input.1;
    let ymax = input.2;
    if stuck_corners {
        input[(0, 0)] = b'#';
        input[(xmax - 1, 0)] = b'#';
        input[(0, ymax - 1)] = b'#';
        input[(xmax - 1, ymax - 1)] = b'#';
    }

    for _ in 0..iter {
        if let Some(w) = writable {
            input.write_pgm(w).expect("Expected to write Netpbm image.");
        }
        let mut changed = Vec::with_capacity(input.0.len());
        for y in 0..ymax {
            for x in 0..xmax {
                let neigh_on_cnt = input
                    .get_neigh(x, y)
                    .iter()
                    .filter(|Neighbor(chr, _, _)| **chr == b'#')
                    .count();
                if input[(x, y)] != b'#' && neigh_on_cnt == 3 {
                    changed.push((b'#', x, y));
                } else if !(2..4).contains(&neigh_on_cnt) {
                    changed.push((b'.', x, y));
                }
            }
        }
        for (chr, x, y) in changed {
            if stuck_corners
                && ((x, y) == (0, 0)
                    || (x, y) == (xmax - 1, ymax - 1)
                    || (x, y) == (xmax - 1, 0)
                    || (x, y) == (0, ymax - 1))
            {
            } else {
                input[(x, y)] = chr;
            }
        }
    }
    if let Some(w) = writable {
        input.write_pgm(w).expect("Expected to write Netpbm image.");
    }
    input.0.iter().filter(|&&chr| chr == b'#').count()
}

fn main() -> io::Result<()> {
    let (input, mut output, options) = advanced_cli();
    let parsed_input = parse_to_flat2d(&input);
    let p1_input = parsed_input.clone();

    let iter = options
        .get("iter")
        .cloned()
        .unwrap_or_else(|| "100".to_owned());

    let iter = iter
        .parse::<u16>()
        .expect("iter= option should be a valid u16.");

    let part1 = part1_sol(iter, false, &mut output, p1_input);
    let part2 = part1_sol(iter, true, &mut output, parsed_input);

    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
