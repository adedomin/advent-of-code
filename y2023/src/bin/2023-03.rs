use aoc_shared::{atoi, parse_to_flat2d, read_input, FlatVec2D, Neighbor};
use std::{collections::HashMap, io};

fn mut_ans(
    sum: &mut i32,
    gears: &mut HashMap<(usize, usize), Vec<i32>>,
    num: i32,
    is_part_num: bool,
    gear: Option<(usize, usize)>,
) {
    if is_part_num {
        *sum += num;
    }
    if let Some((gx, gy)) = gear {
        if let Some(g) = gears.get_mut(&(gx, gy)) {
            g.push(num)
        } else {
            let v = vec![num];
            gears.insert((gx, gy), v);
        }
    }
}

fn solve(input: &FlatVec2D<u8>) -> (i32, i32) {
    let mut sum = 0;
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    for y in 0..input.2 {
        let mut cnum = vec![];
        let mut is_part_num = false;
        let mut gear = None;
        for x in 0..input.1 {
            let digit = input[(x, y)];
            if digit.is_ascii_digit() {
                cnum.push(digit);
                let neigh = input.get_neigh(x, y);
                if !is_part_num {
                    is_part_num = neigh
                        .iter()
                        .find(|Neighbor(v, ..)| !v.is_ascii_digit() && **v != b'.')
                        .is_some();
                }
                if gear.is_none() {
                    if let Some(Neighbor(_, gx, gy)) = neigh.iter().find(|n| *(n.0) == b'*') {
                        gear = Some((*gx, *gy));
                    }
                }
            } else {
                let num = atoi::<i32, 10>(&cnum);
                mut_ans(&mut sum, &mut gears, num, is_part_num, gear);
                cnum.clear();
                is_part_num = false;
                gear = None;
            }
        }
        let num = atoi::<i32, 10>(&cnum);
        mut_ans(&mut sum, &mut gears, num, is_part_num, gear);
    }

    let prod = gears
        .into_values()
        .flat_map(|gr| {
            // if it's not 2, it's ambiguous
            if gr.len() != 2 {
                None
            } else {
                Some(gr.iter().product::<i32>())
            }
        })
        .sum();

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
