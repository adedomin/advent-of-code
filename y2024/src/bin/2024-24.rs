use aoc_shared::{debug, read_input_to_string, try_atoi};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::{io, mem::swap};

type Output<'a> = (
    [u64; 2],
    FxHashMap<&'a str, bool>,
    Vec<(&'a str, (Op, [&'a str; 2]))>,
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
    fn resolve<'a>(
        &self,
        outs: &FxHashMap<&'a str, bool>,
        [left, right]: &[&'a str; 2],
    ) -> Result<bool, ()> {
        match (outs.get(left), outs.get(right)) {
            (Some(&l), Some(&r)) => Ok(match self {
                Op::And => l && r,
                Op::Or => l || r,
                Op::Xor => l ^ r,
            }),
            _ => Err(()),
        }
    }
}

fn convert_to(ident: &str, b: bool) -> u64 {
    let shl = try_atoi::<u32, 10>(&ident.as_bytes()[1..]).expect("not a number!");
    (b as u64) << shl
}

fn parse_input<'a>(input: &'a str) -> Output<'a> {
    let (start, instructions) = input
        .split_once("\n\n")
        .expect("input needs to be delimited.");
    let mut outputs: FxHashMap<&'a str, bool> = FxHashMap::default();
    let start = start
        .split([':', ' ', '\n'])
        .filter(|v| !v.is_empty())
        .tuples()
        .fold([0u64; 2], |mut xy, (ident, val)| {
            let val = val == "1";
            outputs.entry(ident).or_insert(val);
            if ident.starts_with("x") {
                xy[0] |= convert_to(ident, val);
            } else if ident.starts_with("y") {
                xy[1] |= convert_to(ident, val);
            } else {
                panic!("Invalid input ident: {ident}");
            }
            xy
        });
    let wires = instructions
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut items = line.split_ascii_whitespace();
            let from = items.next().expect("Need a left hand to operation");
            let op: Op = items
                .next()
                .expect("Need op")
                .try_into()
                .expect("Invalid Operation.");
            let to = items.next().expect("need right hand op");
            items.next().unwrap();
            let out = items.next().expect("need out");
            (out, (op, [from, to]))
        })
        .collect::<Vec<_>>();
    (start, outputs, wires)
}

fn part1_sol<'a>(
    mut outs: FxHashMap<&'a str, bool>,
    mut wiremap: Vec<(&'a str, (Op, [&'a str; 2]))>,
) -> u64 {
    let mut z = 0;
    let mut wiremap_unres = vec![];
    loop {
        let len = wiremap.len();
        wiremap.drain(..).for_each(|(out, opera @ (op, operands))| {
            match op.resolve(&outs, &operands) {
                Ok(v) => {
                    _ = outs.insert(out, v);
                    if out.starts_with('z') {
                        z |= convert_to(out, v);
                    }
                }
                Err(_) => wiremap_unres.push((out, opera)),
            }
        });
        // means we are not making forward progress.
        assert_ne!(len, wiremap_unres.len());
        if wiremap_unres.is_empty() {
            break;
        }
        swap(&mut wiremap_unres, &mut wiremap);
    }
    z
}

fn recurse_check<'a>(
    key: &'a str,
    (oper, lhs, rhs): &(Op, &'a str, &'a str),
    rev_map: &FxHashMap<&'a str, (Op, &'a str, &'a str)>,
    depth: usize,
    iter: i32,
    max: i32,
) -> Result<(), &'a str> {
    let xy_gate = lhs.starts_with(['x', 'y']) && rhs.starts_with(['x', 'y']);
    // only from z00 can the next set of operands be x ^ y
    if xy_gate && depth == 0 && iter > 0 {
        return Err(key);
    }

    let last = max == iter;
    // immediately after zXX
    if depth == 0 {
        // z45 (or whatever would be max theoretically) is special.
        // (i1 | i2) /*CARRY*/ -> z45
        // else, it must be a i1 ^ i2 -> zXX
        if (last && !matches!(oper, Op::Or)) || (!last && !matches!(oper, Op::Xor)) {
            return Err(key);
        }
    } else if depth == 1 && last {
        if matches!(oper, Op::And) {
            return Ok(());
        } else {
            return Err(key); // after OR, next op must be AND when `last`
        }
    } else if depth == 1 {
        // if our operands are not xy, it must be an AND or XOR
        // else, it must be a XOR, unless our operands are x00 and y00, because no initial carry.
        if (!xy_gate && !matches!(oper, Op::Or))
            || (xy_gate && !matches!(oper, Op::Xor) && iter != 1)
        {
            return Err(key);
        } else if xy_gate {
            // this is the base case anyway, it's assumed this is a proper adder and x and y's are not mutable
            return Ok(());
        }
    } else if depth == 2 {
        // at depth 2, we still have to be in the carry code, so it should be ANDing.
        if !matches!(oper, Op::And) {
            return Err(key);
        }
    } else
    /* we visited the adder part */
    {
        return Ok(());
    }

    if let Some(l) = rev_map.get(lhs) {
        recurse_check(lhs, l, rev_map, depth + 1, iter, max)?;
    }
    if let Some(r) = rev_map.get(rhs) {
        recurse_check(rhs, r, rev_map, depth + 1, iter, max)?;
    }
    Ok(())
}

/*
    X{i}-------\
        |    <x1> ------------------------<x2> ---> Z{i}
    Y{i}-------/   |                       |
      | |          \---\     /--\          |
      | \-------\      <a2> ---<o4>--------|------> <CARRY>
      |          ======<a1>----- | -----/          |
      \---------/            |             |
    <CARRY> -------------------------------/
*/
fn part2_sol<'a>(
    part1: u64,
    [x, y]: [u64; 2],
    wires: Vec<(&'a str, (Op, [&'a str; 2]))>,
) -> String {
    let correct = x + y;
    debug!("solved? {}", correct == part1);
    debug!("{x} + {y} =");
    debug!("{:0b}", x + y);
    debug!("incorrect (?) = ");
    debug!("{:0b}", part1);

    let rev_map = wires
        .iter()
        .map(|(out, (op, [i1, i2]))| (*out, (*op, *i1, *i2)))
        .collect::<FxHashMap<&'a str, (Op, &'a str, &'a str)>>();
    let mut res = vec![];
    let mut zmax = 0;
    let zs = wires
        .iter()
        .filter_map(|(out, _)| {
            if let Some(num) = out.strip_prefix('z') {
                let zn = num.parse::<i32>().unwrap();
                zmax = std::cmp::max(zmax, zn);
                Some((*out, zn))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    println!("{zs:?}");
    zs.into_iter().for_each(|(key, iter)| {
        let oper = rev_map.get(key).expect("huh?");
        if let Err(e) = recurse_check(key, oper, &rev_map, 0, iter, zmax) {
            res.push(e);
        }
    });
    res.sort();
    res.join(",")
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let (start, outs, wires) = parse_input(&input);
    let part1 = part1_sol(outs, wires.clone());
    let part2 = part2_sol(part1, start, wires);
    println!("Part1: {part1}");
    println!("Part2: {part2}");
    // println!("Part2: Read comments above part2_sol");
    // println!("       dwp,ffj,gjh,jdr,kfm,z08,z22,z31");
    Ok(())
}
