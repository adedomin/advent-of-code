use aoc_shared::{
    atoi, destructure_or_none, read_input, try_atoi, ts_to_u64, GroupTokenize, GroupTokens, Token,
};
use itertools::Itertools;

use std::{collections::HashMap, io};

type Output = (HashMap<u32, Workflow>, Vec<Part>);

#[derive(Default, Eq, PartialEq, PartialOrd, Ord)]
struct Part(pub [u32; 4]);

impl std::fmt::Debug for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{ ")?;
        for (i, v) in self.0.iter().enumerate() {
            match i {
                0 => f.write_fmt(format_args!("x={} ", v)),
                1 => f.write_fmt(format_args!("m={} ", v)),
                2 => f.write_fmt(format_args!("a={} ", v)),
                3 => f.write_fmt(format_args!("s={} ", v)),
                _ => unreachable!(),
            }?
        }
        f.write_str("}")?;
        Ok(())
    }
}

#[derive(Clone)]
struct PartPerm(pub [(u32, u32); 4]);

impl std::fmt::Debug for PartPerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{ ")?;
        for (i, v) in self.0.iter().enumerate() {
            match i {
                0 => f.write_fmt(format_args!("x={:?} ", v)),
                1 => f.write_fmt(format_args!("m={:?} ", v)),
                2 => f.write_fmt(format_args!("a={:?} ", v)),
                3 => f.write_fmt(format_args!("s={:?} ", v)),
                _ => unreachable!(),
            }?
        }
        f.write_str("}")?;
        Ok(())
    }
}

impl Default for PartPerm {
    fn default() -> Self {
        Self([(CHARACTERISTICS_MIN, CHARACTERISTICS_MAX); 4])
    }
}

const X: usize = 0;
const M: usize = 1;
const A: usize = 2;
const S: usize = 3;
const INVALID: usize = 999;

// const A_LABEL: u32 = ts_to_u64(b"A") as u32;
const R_LABEL: u32 = ts_to_u64(b"R") as u32;
const IN_LABEL: u32 = ts_to_u64(b"in") as u32;

const CHARACTERISTICS_MIN: u32 = 1;
const CHARACTERISTICS_MAX: u32 = 4000;

#[derive(Debug)]
enum Op {
    Greater(usize, u32),
    Less(usize, u32),
    Noop,
}

impl Op {
    fn mcharacteristics(&self, p: &Part) -> bool {
        match self {
            Op::Greater(idx, val) => p.0[*idx] > *val,
            Op::Less(idx, val) => p.0[*idx] < *val,
            Op::Noop => true,
        }
    }

    fn mselects(&self, part: &mut PartPerm) -> PartPerm {
        let mut npart = part.clone();
        match self {
            Op::Greater(idx, val) => {
                let idx = *idx;
                let (rstart, rend) = part.0[idx];
                let vmin = std::cmp::min(*val, rend);
                let vmax = std::cmp::max(*val, rstart);
                npart.0[idx] = (vmax + 1, rend);
                part.0[idx] = (rstart, vmin);
                npart
            }
            Op::Less(idx, val) => {
                let idx = *idx;
                let (rstart, rend) = part.0[idx];
                let vmin = std::cmp::min(*val, rend);
                let vmax = std::cmp::max(*val, rstart);
                npart.0[idx] = (rstart, vmin - 1);
                part.0[idx] = (vmax, rend);
                npart
            }
            // consumes the rest.
            Op::Noop => {
                part.0 = [(0, 0); 4];
                npart
            }
        }
    }
}

#[derive(Default)]
struct Workflow(pub Vec<(Op, u32)>);

fn parse_input(input: &[u8]) -> Output {
    let mut grp = input.group_tokens(Token::DoubleNewline);
    let workflows = grp.next().expect("Expected workflows to be in input");
    let parts = grp.next().expect("Expected parts to be in input");

    let workflows = workflows
        .into_iter()
        .group_tokens(Token::Newline)
        .map(|wf| {
            let key = &wf[0];
            let rest = &wf[1..];
            if let Token::Something(key) = key {
                let key = ts_to_u64(key) as u32;
                let wfs = rest
                    .iter()
                    .fold(
                        (Workflow::default(), INVALID, b' ', 0u32, 0u32),
                        |(mut acc, ty, op, v, label), token| match token {
                            Token::Something(w) if ty == INVALID => {
                                let ty = match *w {
                                    b"x" => X,
                                    b"m" => M,
                                    b"a" => A,
                                    b"s" => S,
                                    _ => {
                                        let label = ts_to_u64(w) as u32;
                                        return (acc, INVALID, b' ', 0, label);
                                    }
                                };
                                (acc, ty, op, v, label)
                            }
                            Token::Something(w) => {
                                if let Some(v) = try_atoi::<u32, 10>(w) {
                                    (acc, ty, op, v, label)
                                } else {
                                    let label = ts_to_u64(w) as u32;
                                    (acc, ty, op, v, label)
                                }
                            }
                            Token::Delimiter(b'>') => (acc, ty, b'>', v, label),
                            Token::Delimiter(b'<') => (acc, ty, b'<', v, label),
                            Token::Delimiter(b',') | Token::Delimiter(b'}') => {
                                let op = match op {
                                    b'>' => Op::Greater(ty, v),
                                    b'<' => Op::Less(ty, v),
                                    _ => Op::Noop,
                                };
                                acc.0.push((op, label));
                                (acc, INVALID, b' ', 0, 0)
                            }
                            _ => (acc, ty, op, v, label),
                        },
                    )
                    .0;

                (key, wfs)
            } else {
                panic!("Invalid token!");
            }
        })
        .collect::<HashMap<u32, Workflow>>();

    let parts = parts
        .into_iter()
        .group_tokens(Token::Newline)
        .map(|part| {
            part.into_iter()
                .group_tokens(Token::Delimiter(b','))
                .flat_map(|w| {
                    w.iter()
                        .flat_map(|w| destructure_or_none!(Token::Something|word| = w))
                        .tuples()
                        .map(|(t, v)| {
                            let v = atoi::<u32, 10>(v);
                            match t {
                                &b"x" => (X, v),
                                &b"m" => (M, v),
                                &b"a" => (A, v),
                                &b"s" => (S, v),
                                _ => panic!("Invalid Type: {t:?}"),
                            }
                        })
                        .collect_vec()
                })
                .fold(Part::default(), |mut acc, (idx, v)| {
                    acc.0[idx] = v;
                    acc
                })
        })
        .collect::<Vec<Part>>();

    (workflows, parts)
}

fn solve1(workflows: &HashMap<u32, Workflow>, parts: &[Part]) -> u32 {
    let mut accepted = 0;
    for part in parts {
        let mut label = IN_LABEL;
        while let Some(wf) = workflows.get(&label) {
            if let Some((_, l)) = wf.0.iter().find(|(op, _)| op.mcharacteristics(part)) {
                label = *l;
            } else {
                break;
            }
        }
        if label != R_LABEL {
            accepted += part.0.iter().sum::<u32>();
        }
    }
    accepted
}

fn solve2(workflows: &HashMap<u32, Workflow>) -> u64 {
    fn rec(workflows: &HashMap<u32, Workflow>, label: u32, mut part: PartPerm) -> u64 {
        if let Some(wf) = workflows.get(&label) {
            let mut ret = 0u64;
            for (op, label) in wf.0.iter() {
                let npart = op.mselects(&mut part);
                ret += rec(&workflows, *label, npart);
            }
            debug_assert_eq!(
                part.0
                    .into_iter()
                    .map(|(s, e)| (e - s) as u64)
                    .product::<u64>(),
                0
            );
            ret
        } else if label != R_LABEL {
            let ret = part
                .0
                .into_iter()
                .map(|(s, e)| (e - s) as u64 + 1)
                .product();
            ret
        } else {
            let ret = 0;
            ret
        }
    }
    rec(&workflows, IN_LABEL, PartPerm::default())
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (workflows, parts) = parse_input(&input);
    let part1 = solve1(&workflows, &parts);
    print!("Part1: {part1}, ");
    let part2 = solve2(&workflows);
    print!("Part2: {part2}");
    println!();
    Ok(())
}
