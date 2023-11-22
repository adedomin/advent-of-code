use aoc_shared::{parse_to_flat2d, read_input, FlatVec2D, Neighbor};
use std::io;

#[cfg(debug_assertions)]
fn animate(input: &FlatVec2D<u8>, clear: bool) {
    if clear {
        println!("{}", "\x1b[2J\x1b[H");
    } else {
        println!("---------------------");
    }
    for y in 0..input.2 {
        for x in 0..input.1 {
            if input[(x, y)] == b'#' {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn part1_sol(iter: u8, stuck_corners: bool, mut input: FlatVec2D<u8>) -> usize {
    let xmax = input.1;
    let ymax = input.2;
    if stuck_corners {
        input[(0, 0)] = b'#';
        input[(xmax - 1, 0)] = b'#';
        input[(0, ymax - 1)] = b'#';
        input[(xmax - 1, ymax - 1)] = b'#';
    }

    for _ in 0..iter {
        #[cfg(debug_assertions)]
        {
            animate(&input, true);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let mut changed = Vec::with_capacity(input.0.len());
        for y in 0..ymax {
            for x in 0..xmax {
                let neigh_on_cnt = input
                    .get_neigh(x, y)
                    .iter()
                    .filter(|Neighbor(chr, _, _)| **chr == b'#')
                    .count();
                if input[(x, y)] != b'#' {
                    if neigh_on_cnt == 3 {
                        changed.push((b'#', x, y));
                    }
                } else {
                    if neigh_on_cnt < 2 || neigh_on_cnt > 3 {
                        changed.push((b'.', x, y));
                    }
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
                ()
            } else {
                input[(x, y)] = chr;
            }
        }
    }
    #[cfg(debug_assertions)]
    {
        animate(&input, true);
    }
    input.0.iter().filter(|&&chr| chr == b'#').count()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let p1_input = parsed_input.clone();
    let part1 = part1_sol(100, false, p1_input);
    let part2 = part1_sol(100, true, parsed_input);

    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
