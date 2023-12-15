use aoc_shared::{fold_decimal, parse_to_flat2d, read_input, FlatVec2D, Neighbor};
use std::{
    collections::{HashMap, HashSet},
    io,
    mem::take,
};

fn mut_ans(
    sum: &mut i32,
    gears: &mut HashMap<(usize, usize), GearProd>,
    num: i32,
    is_part_num: bool,
    gear: HashSet<(usize, usize)>,
) {
    if is_part_num {
        *sum += num;
    }

    for (gx, gy) in gear {
        if let Some(g) = gears.get_mut(&(gx, gy)) {
            let _ = g.add_partnum(num);
        } else {
            let mut g = GearProd::new();
            let _ = g.add_partnum(num);
            gears.insert((gx, gy), g);
        }
    }
}

struct GearProd {
    pos: u8,
    partnum: [i32; 2],
}

impl GearProd {
    fn new() -> Self {
        GearProd {
            pos: 0,
            partnum: [0i32; 2],
        }
    }
    /// A gear is any * symbol that is adjacent to exactly two part numbers.
    fn add_partnum(&mut self, p: i32) -> Result<(), ()> {
        let ret = if self.pos > 1 {
            Err(())
        } else {
            self.partnum[self.pos as usize] = p;
            Ok(())
        };
        self.pos += 1; // always increment, more than 2 should result in no product;
        ret
    }
    /// Its gear ratio is the result of multiplying those two numbers together.
    fn product(&self) -> Option<i32> {
        if self.pos != 2 {
            None
        } else {
            Some(self.partnum[0] * self.partnum[1])
        }
    }
}

fn solve(input: &FlatVec2D<u8>) -> (i32, i32) {
    let mut sum = 0;
    let mut gears: HashMap<(usize, usize), GearProd> = HashMap::new();
    for y in 0..input.2 {
        let mut num = 0;
        let mut is_part_num = false;
        let mut gear = HashSet::new();
        for x in 0..input.1 {
            let digit = input[(x, y)];
            if digit.is_ascii_digit() {
                num = fold_decimal(num, &digit);
                let neigh = input.get_neigh(x, y);
                if !is_part_num {
                    is_part_num = neigh
                        .iter()
                        .any(|Neighbor(v, ..)| !v.is_ascii_digit() && **v != b'.');
                }
                gear.extend(
                    neigh
                        .iter()
                        .filter(|n| *(n.0) == b'*')
                        .map(|Neighbor(_, gx, gy)| (*gx, *gy)),
                );
            } else {
                mut_ans(&mut sum, &mut gears, num, is_part_num, take(&mut gear));
                num = 0;
                is_part_num = false;
            }
        }
        mut_ans(&mut sum, &mut gears, num, is_part_num, gear);
    }

    let prod = gears.into_values().flat_map(|gr| gr.product()).sum();

    (sum, prod)
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let (part1, part2) = solve(&parsed_input);
    // let part2 = part2_sol(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
