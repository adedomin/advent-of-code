use aoc_shared::{read_input_to_string, try_atoi};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::io;

type Int = u64;
type Output<'a> = (
    Vec<(&'a str, bool)>,
    Vec<(Op, [Option<bool>; 2], &'a str)>,
    FxHashMap<&'a str, Vec<(usize, usize)>>,
);

#[derive(Clone, Copy, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl TryFrom<&str> for Op {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(()),
        }
    }
}

impl Op {
    fn exec(&self, v: [Option<bool>; 2]) -> Option<bool> {
        match (self, v) {
            (Op::And, [None, _]) => None,
            (Op::And, [_, None]) => None,
            (Op::And, [Some(a), Some(b)]) => Some(a && b),
            (Op::Or, [None, _]) => None,
            (Op::Or, [_, None]) => None,
            (Op::Or, [Some(a), Some(b)]) => Some(a || b),
            (Op::Xor, [None, _]) => None,
            (Op::Xor, [_, None]) => None,
            (Op::Xor, [Some(a), Some(b)]) => Some(a ^ b),
        }
    }
}

fn parse_input<'a>(input: &'a str) -> Output<'a> {
    let (start, instructions) = input
        .split_once("\n\n")
        .expect("input needs to be delimited.");
    let start = start
        .split([':', ' ', '\n'])
        .filter(|v| !v.is_empty())
        .tuples()
        .map(|(ident, val)| (ident, val == "1"))
        .collect::<Vec<(_, bool)>>();
    let mut wiremap: FxHashMap<&'a str, Vec<(usize, usize)>> = FxHashMap::default();
    let wires = instructions
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(i, line)| {
            let mut items = line.split_ascii_whitespace();
            let from = items.next().expect("Need a left hand to operation");
            wiremap.entry(from).or_default().push((i, 0));

            let op: Op = items
                .next()
                .expect("Need op")
                .try_into()
                .expect("Invalid Operation.");

            let to = items.next().expect("need right hand op");
            wiremap.entry(to).or_default().push((i, 1));

            items.next().unwrap();
            let out = items.next().expect("need out");

            (op, [None, None], out)
        })
        .collect::<Vec<_>>();
    (start, wires, wiremap)
}

fn part1_sol<'a>(
    start: &[(&'a str, bool)],
    wiremap: &FxHashMap<&'a str, Vec<(usize, usize)>>,
    mut wires: Vec<(Op, [Option<bool>; 2], &'a str)>,
) -> Int {
    let mut ret: Int = 0;
    start.iter().for_each(|&(ident, v)| {
        let mut propagate = vec![(ident, v)];
        while let Some((ident, v)) = propagate.pop() {
            if let Some(idc) = wiremap.get(ident) {
                propagate.extend(idc.iter().filter_map(|&(i, pos)| {
                    wires[i].1[pos] = Some(v);
                    wires[i].0.exec(wires[i].1).map(|b| (wires[i].2, b))
                }))
            } else if ident.starts_with("z") {
                let shl = try_atoi::<u32, 10>(&ident.as_bytes()[1..]).expect("not a number!");
                ret |= (v as Int)
                    .checked_shl(shl)
                    .expect("shl value too large: {shl}");
            } else {
                panic!("unknown ident! {ident}");
            }
        }
    });
    ret
}

// fn part2_sol(input: &Output) -> Int {}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let (start, wires, wiremap) = parse_input(&input);
    let part1 = part1_sol(&start, &wiremap, wires.clone());
    // let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    // print!("Part2: {part2}");
    println!();
    Ok(())
}
