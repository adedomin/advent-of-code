use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    io,
};

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

fn conn(
    mut cid: usize,
    circuits: &mut HashMap<usize, Vec<usize>>,
    cmap: &mut [Option<usize>],
    (l, r): (usize, usize),
) -> usize {
    match (&cmap[l], &cmap[r]) {
        (None, None) => {
            circuits.insert(cid, vec![l, r]);
            cmap[l] = Some(cid);
            cmap[r] = Some(cid);
            cid += 1;
        }
        (None, Some(rcid)) => {
            let key = circuits.get_mut(rcid).unwrap();
            key.push(l);
            cmap[l] = Some(*rcid);
        }
        (Some(lcid), None) => {
            let key = circuits.get_mut(lcid).unwrap();
            key.push(r);
            cmap[r] = Some(*lcid);
        }
        (Some(lcid), Some(rcid)) if lcid != rcid => {
            let mut lc = circuits.remove(lcid).unwrap();
            lc.extend(circuits.remove(rcid).unwrap());
            lc.iter().for_each(|i| cmap[*i] = Some(cid));
            circuits.insert(cid, lc);
            cid += 1;
        }
        _ => (),
    }
    cid
}

fn solve(input: &[[Int; 3]]) -> (usize, Int) {
    let mut dists = (0..input.len() - 1)
        .flat_map(|l| (l + 1..input.len()).map(move |r| (l, r)))
        .map(|(l, r)| Reverse((dist_pow2(&input[l], &input[r]), (l, r))))
        .collect::<BinaryHeap<Reverse<(Int, (usize, usize))>>>();

    let mut cid = 0usize;
    let mut circuits: HashMap<usize, Vec<usize>, _> = HashMap::new();
    let mut circuit_map = vec![None; input.len()];
    for _ in 0..P1_LIM {
        if let Some(Reverse((_d, lr))) = dists.pop() {
            cid = conn(cid, &mut circuits, &mut circuit_map, lr);
        } else {
            panic!("Not enough junctions to connect!");
        }
    }
    // println!("{circuits:?}\n{circuit_map:?}");
    let mut x = circuits
        .values()
        .map(|c| c.len())
        .collect::<BinaryHeap<_>>();
    let mut i = 0;
    let mut last = usize::MIN;
    let mut part1 = 1;
    while i < 3 {
        if let Some(n) = x.pop()
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
    while circuits.len() != 1 {
        if let Some(Reverse((_d, lr))) = dists.pop() {
            last = lr;
            cid = conn(cid, &mut circuits, &mut circuit_map, lr);
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
