use aoc_shared::read_input_to_string;
use itertools::Itertools;
use std::{collections::VecDeque, io};

type Int = u64;

type Reg = [Int; 3];
type Prog = Vec<Int>;

fn parse_input(input: &str) -> (Reg, Prog) {
    let mut iter = input
        .split(|n: char| !n.is_ascii_digit())
        .filter_map(|num| {
            if num.is_empty() {
                None
            } else {
                num.parse::<Int>().ok()
            }
        });
    let reg = [
        iter.next().expect("Expected a A register value."),
        iter.next().expect("Expected a B register value."),
        iter.next().expect("Expected a C register value."),
    ];
    (reg, iter.collect::<Vec<_>>())
}

#[derive(Debug)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug)]
enum Combo {
    Literal(Int, Int),
    Register(usize, Int),
    Invalid(Int),
}

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

impl From<Int> for Combo {
    fn from(value: Int) -> Self {
        match value {
            0..=3 => Self::Literal(value, value),
            4..=6 => Self::Register(value as usize - 4, value),
            _ => Self::Invalid(value),
        }
    }
}

impl Combo {
    fn get_value(&self, reg: &Reg) -> Int {
        match self {
            Combo::Literal(v, _) => *v,
            Combo::Register(i, _) => reg[*i],
            Combo::Invalid(_) => panic!("Invalid Operand!"),
        }
    }

    fn get_literal(&self) -> Int {
        match self {
            Combo::Literal(_, lit) => *lit,
            Combo::Register(_, lit) => *lit,
            Combo::Invalid(lit) => *lit,
        }
    }
}

impl TryFrom<Int> for Op {
    type Error = String;

    fn try_from(value: Int) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Op::Adv),
            1 => Ok(Op::Bxl),
            2 => Ok(Op::Bst),
            3 => Ok(Op::Jnz),
            4 => Ok(Op::Bxc),
            5 => Ok(Op::Out),
            6 => Ok(Op::Bdv),
            7 => Ok(Op::Cdv),
            value => Err(format!("Invalid Op {value}").to_owned()),
        }
    }
}

fn part1_sol(mut reg: Reg, prog: &Prog, into: &mut Prog) {
    into.clear();
    let mut pc = 0usize;
    while let Some((op, combo)) = prog
        .get(pc..pc + 2)
        .and_then(|ops| Op::try_from(ops[0]).ok().map(|v| (v, Combo::from(ops[1]))))
    {
        pc += 2;
        match op {
            Op::Adv => reg[A] >>= combo.get_value(&reg),
            Op::Bxl => reg[B] ^= combo.get_literal(),
            Op::Bst => reg[B] = combo.get_value(&reg) & 0b111,
            Op::Jnz => {
                if reg[A] != 0 {
                    pc = combo.get_literal() as usize;
                }
            }
            Op::Bxc => reg[B] ^= reg[C],
            Op::Out => into.push(combo.get_value(&reg) & 0b111),
            Op::Bdv => reg[B] = reg[A] >> combo.get_value(&reg),
            Op::Cdv => reg[C] = reg[A] >> combo.get_value(&reg),
        }
    }
}

// fn parse_input_hardcode(input: &str) -> Int {
//     let mut iter = input
//         .split(|n: char| !n.is_ascii_digit())
//         .filter_map(|num| {
//             if num.is_empty() {
//                 None
//             } else {
//                 num.parse::<Int>().ok()
//             }
//         });
//     iter.next().expect("Expected a A register value.")
// }

// fn part1_hardcode(mut a: Int) -> Prog {
//     // let mut b = 0;
//     // let mut c = 0;
//     let mut ret = vec![];
//     while a != 0 {
//         let mut b = a & 0b111;
//         b ^= 1;
//         let c = a >> b;
//         b ^= 5;
//         b ^= c;
//         b &= 0b111;
//         ret.push(b);
//         a >>= 3;
//     }
//     ret
// }

fn part2_sol(reg: Reg, prog: &Prog) -> Int {
    let mut curr = vec![];
    // first a we find should be the lowest with BFS.
    let mut queue = VecDeque::from([(0, prog.len() - 1)]);
    while let Some((a, expected)) = queue.pop_front() {
        for a in (0..8).map(|na| (a << 3) + na).filter(|&a| {
            part1_sol([a, reg[1], reg[2]], prog, &mut curr);
            curr[..] == prog[expected..]
        }) {
            if expected == 0 {
                return a;
            } else {
                queue.push_back((a, expected - 1));
            }
        }
    }
    panic!("no solution!");
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    // let reg_a = parse_input_hardcode(&input);
    // let nprog = part1_hardcode(reg_a);
    let (reg, prog) = parse_input(&input);
    let mut part1 = vec![];
    part1_sol(reg, &prog, &mut part1);
    println!("Part1: {}", part1.iter().map(|n| n.to_string()).join(","));
    let part2 = part2_sol(reg, &prog);
    println!("Part2: {part2}");
    Ok(())
}
