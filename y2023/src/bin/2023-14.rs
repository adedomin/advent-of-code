use aoc_shared::{advanced_cli, parse_to_flat2d, FlatVec2D, Rot2D};
use std::{collections::HashMap, io};

type Output = FlatVec2D<Rocks>;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Rocks {
    Rounded,
    Square,
    Empty,
}

impl Default for Rocks {
    fn default() -> Self {
        Self::Empty
    }
}

impl From<u8> for Rocks {
    fn from(value: u8) -> Self {
        match value {
            b'O' => Self::Rounded,
            b'#' => Self::Square,
            _ => Self::Empty,
        }
    }
}

impl std::fmt::Debug for Rocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rocks::Rounded => f.write_str("O"),
            Rocks::Square => f.write_str("#"),
            Rocks::Empty => f.write_str("."),
        }
    }
}

fn calc_load(grid: &Output) -> usize {
    let mut ret = 0;
    for y in 0..grid.2 {
        for x in 0..grid.1 {
            match grid[(x, y)] {
                Rocks::Rounded => ret += grid.2 - y,
                _ => (),
            }
        }
    }
    ret
}

fn solve2(grid: &Output, cycle_cnt: usize, is_p1: bool) -> usize {
    let mut cycle_detector = HashMap::new();
    let mut g2 = grid.clone();
    let mut i = 0;
    'out: while i < cycle_cnt {
        for (cardinal, xend, yend) in [
            (Rot2D::None, grid.1, grid.2),
            (Rot2D::Clock270, grid.2, grid.1),
            (Rot2D::Clock180, grid.1, grid.2),
            (Rot2D::Clock90, grid.2, grid.1),
        ] {
            // from left to right
            for x in 0..xend {
                let mut last = 0;
                // go north to south
                for y in 0..yend {
                    let t = g2[(x, y, cardinal)];
                    use Rocks::*;
                    match t {
                        Square => last = y + 1,
                        Rounded => {
                            if last != y {
                                g2[(x, last, cardinal)] = Rounded;
                                g2[(x, y, cardinal)] = Empty;
                            }
                            last = last + 1;
                        }
                        _ => (),
                    }
                }
            }
            if is_p1 {
                break 'out;
            }
        }
        i += 1;
        // we skip over common periods...
        // insert returns the old result if it existed.
        if let Some(cycle_amt) = cycle_detector.insert(g2.0.clone(), i) {
            let period = i - cycle_amt;
            i += (cycle_cnt - i) / period * period;
        }
    }
    #[cfg(debug_assertions)]
    {
        println!("{g2:?}");
    }
    calc_load(&g2)
}

const P2_CYCLE: usize = 1_000_000_000;

fn main() -> io::Result<()> {
    let (input, _, opts) = advanced_cli();
    let parsed_input = parse_to_flat2d(&input);
    let part1 = solve2(&parsed_input, 1, true);
    let cycles = if let Some(c) = opts.get("cycle") {
        c.parse::<usize>().unwrap()
    } else {
        P2_CYCLE
    };
    let part2 = solve2(&parsed_input, cycles, false);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
