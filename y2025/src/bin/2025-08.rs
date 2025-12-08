use std::{cmp::Reverse, collections::BinaryHeap, io};

use aoc_shared::read_input_to_string;

type Int = i64;

fn parse_input(i: &str) -> Vec<[Int; 3]> {
    i.split_ascii_whitespace()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<Int>().expect("Valid num."))
                .collect::<Vec<_>>()
                .try_into()
                .expect("3 Numbers per line.")
        })
        .collect()
}

// we don't need to sqrt because we aren't using the distance for anything other than sorting.
fn dist_pow2(lj: &[Int; 3], rj: &[Int; 3]) -> Int {
    lj.iter()
        .zip(rj)
        .map(|(&l, &r)| l.abs_diff(r).pow(2) as Int)
        .sum::<Int>()
}

const P1_LIM: usize = 1000;

fn conn(cmap: &mut [usize], (l, r): (usize, usize)) -> bool {
    let (lc, rc) = (cmap[l], cmap[r]);
    if lc == rc {
        return false;
    }
    cmap.iter_mut().for_each(|c| {
        if *c == rc {
            *c = lc;
        }
    });
    true
}

fn solve(input: &[[Int; 3]]) -> (usize, Int) {
    let mut dists = (0..input.len() - 1)
        .flat_map(|l| (l + 1..input.len()).map(move |r| (l, r)))
        .map(|(l, r)| Reverse((dist_pow2(&input[l], &input[r]), (l, r))))
        .collect::<BinaryHeap<Reverse<(Int, (usize, usize))>>>();

    let mut circuits = (0..input.len()).collect::<Vec<usize>>();
    let mut circuits_len = circuits.len();
    for _ in 0..P1_LIM {
        if let Some(Reverse((_d, lr))) = dists.pop() {
            if conn(&mut circuits, lr) {
                circuits_len -= 1;
            }
        } else {
            panic!("Not enough junctions to connect!");
        }
    }

    // part1
    let mut hist = vec![0; circuits.len()];
    circuits.iter().for_each(|&c| hist[c] += 1);
    hist.sort_unstable();
    let mut i = 0;
    let mut last = usize::MIN;
    let mut part1 = 1;
    while i < 3 {
        if let Some(n) = hist.pop()
            && n != last
        {
            part1 *= n;
            i += 1;
            last = n;
        } else {
            panic!("Not enough unique circuit sizes!");
        }
    }

    // part2
    let mut last = (0, 0);
    while circuits_len != 1 {
        if let Some(Reverse((_d, lr))) = dists.pop() {
            last = lr;
            if conn(&mut circuits, lr) {
                circuits_len -= 1;
            }
        } else {
            panic!("Not enough junctions to connect!");
        }
    }

    (part1, input[last.0][0] * input[last.1][0])
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let input = parse_input(input.trim());
    let (part1, part2) = solve(&input);
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
