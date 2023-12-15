use aoc_shared::{atoi, debug, read_input, try_atoi, AoCTokenizer, Token};
use std::{collections::HashMap, io};

type Output = HashMap<u16, Op>;
type Solved = u16;

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug)]
enum OpTreeTok {
    OpOr,
    OpAnd,
    OpLshift,
    OpRshift,
    OpNot,
    OpAssign,
    Ident(u16),
    Literal(u16),
    NOOP,
}

enum Dir {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Op(OpTreeTok, Option<OpTreeTok>, Option<OpTreeTok>);

impl Op {
    fn eval(&self) -> Result<u16, (Dir, u16)> {
        use OpTreeTok::*;
        fn get_rhs_and(op: OpTreeTok, lhs: u16, rhs: OpTreeTok) -> Result<u16, (Dir, u16)> {
            match rhs {
                Ident(i) => Err((Dir::Right, i)),
                Literal(val) => match op {
                    OpOr => Ok(lhs | val),
                    OpAnd => Ok(lhs & val),
                    OpLshift => Ok(lhs << val),
                    OpRshift => Ok(lhs >> val),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        let Op(op, lhs, rhs) = self;
        if let Some(lhs) = lhs {
            match lhs {
                Ident(i) => Err((Dir::Left, *i)),
                Literal(lhs) => match op {
                    NOOP => Ok(*lhs),
                    OpNot => Ok(!*lhs),
                    OpOr | OpAnd | OpLshift | OpRshift => {
                        if let Some(rhs) = rhs {
                            get_rhs_and(*op, *lhs, *rhs)
                        } else {
                            panic!("Binary operation, but only got one value.");
                        }
                    }
                    _ => panic!("value in place of operator."),
                },
                _ => panic!("Operator in place of value."),
            }
        } else {
            panic!("Operator missing lhs!");
        }
    }
}

fn parse_input(input: Vec<u8>) -> Output {
    use OpTreeTok::*;
    let (tokens, _, _) = AoCTokenizer::new(&input)
        .flat_map(|token| {
            match token {
                Token::Something(op) if op.to_ascii_lowercase() != op => Some(match op {
                    b"OR" => OpOr,
                    b"AND" => OpAnd,
                    b"NOT" => OpNot,
                    b"LSHIFT" => OpLshift,
                    b"RSHIFT" => OpRshift,
                    _ => panic!("Unknown OP {op:?}"),
                }),
                Token::Something(num_or_ident) => {
                    if let Some(num) = try_atoi::<u16, 10>(num_or_ident) {
                        // numbers are decimal
                        Some(Literal(num))
                    } else if let Some(ident) = try_atoi::<u16, 36>(num_or_ident) {
                        // Identifiers are like, a, aa, kj, etc. so they should
                        // parse as radix 36 numbers.
                        Some(Ident(ident))
                    } else {
                        panic!("Invalid number or ident {num_or_ident:?}");
                    }
                }
                Token::Delimiter(b'>') => Some(OpAssign),
                _ => None,
            }
        })
        .fold(
            (HashMap::new(), Op(NOOP, None, None), false),
            |(mut acc, Op(op, lhs, rhs), found_asgn), tok| {
                if found_asgn {
                    match tok {
                        Ident(i) => {
                            acc.insert(i, Op(op, lhs, rhs));
                            return (acc, Op(NOOP, None, None), false);
                        }
                        _ => panic!("invalid line"),
                    }
                };
                let new_op = match tok {
                    OpOr => Op(tok, lhs, rhs),
                    OpAnd => Op(tok, lhs, rhs),
                    OpNot => Op(tok, lhs, rhs),
                    OpLshift => Op(tok, lhs, rhs),
                    OpRshift => Op(tok, lhs, rhs),
                    OpAssign => return (acc, Op(op, lhs, rhs), true),
                    Literal(_) | Ident(_) if lhs.is_none() => Op(op, Some(tok), rhs),
                    Literal(_) | Ident(_) if rhs.is_none() => Op(op, lhs, Some(tok)),
                    _ => unreachable!(),
                };
                (acc, new_op, found_asgn)
            },
        );
    tokens
}

fn part1_sol(input: &Output, solve_for: u16) -> Solved {
    let mut resolved = vec![None; u16::MAX as usize];
    let start_op = input
        .get(&solve_for)
        .expect("no such value to solve for.")
        .clone();
    let mut stack = vec![(solve_for, start_op)];
    while let Some((var, op)) = stack.pop() {
        debug!("{:-width$} <- {op:?}", var, width = stack.len());
        if stack.len() == input.len() {
            panic!("We're probably infinite looping.");
        }
        match op.eval() {
            Ok(solved) => {
                resolved[var as usize] = Some(solved);
            }
            Err((direction, unident)) => {
                if let Some(val) = resolved[unident as usize] {
                    match direction {
                        Dir::Left => {
                            stack.push((var, Op(op.0, Some(OpTreeTok::Literal(val)), op.2)))
                        }
                        Dir::Right => {
                            stack.push((var, Op(op.0, op.1, Some(OpTreeTok::Literal(val)))))
                        }
                    }
                } else {
                    stack.push((var, op));
                    let solve_op = input
                        .get(&unident)
                        .expect("no such value to solve for.")
                        .clone();
                    stack.push((unident, solve_op))
                }
            }
        };
    }
    resolved[solve_for as usize].expect("Could not resolve given variable.")
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let mut parsed_input = parse_input(input);
    let part1 = part1_sol(&parsed_input, atoi::<_, 36>(b"a"));
    // part2 requires changing a rule for wire b: part1 -> b, and rerunning, solving for wire a.
    parsed_input.insert(
        atoi::<_, 36>(b"b"),
        Op(OpTreeTok::NOOP, Some(OpTreeTok::Literal(part1)), None),
    );
    let part2 = part1_sol(&parsed_input, atoi::<u16, 36>(b"a"));

    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
