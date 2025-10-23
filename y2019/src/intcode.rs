use aoc_shared::read_input_to_string;
use std::io;

pub fn read_intcode() -> io::Result<Vec<i64>> {
    let values = read_input_to_string()?;
    Ok(values
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|s| !s.is_empty())
        .map(|num| num.parse::<i64>().expect("Invalid number."))
        .collect::<Vec<i64>>())
}

#[derive(Copy, Clone, Debug)]
enum PMode {
    Pos,
    Imm,
}

impl TryFrom<i64> for PMode {
    type Error = IntCodeErr;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Pos,
            1 => Self::Imm,
            _ => return Err(IntCodeErr::InvalidParamMode(value)),
        })
    }
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Inp,
    Out,
    Jit,
    Jif,
    Lt,
    Eq,
    End,
}

struct Oper {
    p1: PMode,
    p2: PMode,
    p3: PMode,
    opcode: Op,
}

#[derive(thiserror::Error, Debug)]
pub enum IntCodeErr {
    #[error("Received an invalid operation: {0}")]
    InvalidOp(i64),
    #[error("Out of bounds fetching operands.")]
    OutOfBounds,
    #[error("Negative index.")]
    NegativeIndex,
    #[error("Invalid Parameter Mode: {0}")]
    InvalidParamMode(i64),
}

impl TryFrom<i64> for Oper {
    type Error = IntCodeErr;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if !(0..=99999).contains(&value) {
            return Err(IntCodeErr::InvalidOp(value));
        }
        let p3 = PMode::try_from(value / 10i64.pow(4) % 10)?;
        let p2 = PMode::try_from(value / 10i64.pow(3) % 10)?;
        let p1 = PMode::try_from(value / 10i64.pow(2) % 10)?;
        let op = value % 100;
        let opcode = match op {
            1 => Op::Add,
            2 => Op::Mul,
            3 => Op::Inp,
            4 => Op::Out,
            5 => Op::Jit,
            6 => Op::Jif,
            7 => Op::Lt,
            8 => Op::Eq,
            99 => Op::End,
            value => return Err(IntCodeErr::InvalidOp(value)),
        };
        Ok(Self { p1, p2, p3, opcode })
    }
}

fn get_op(pc: usize, program: &[i64]) -> Result<Oper, IntCodeErr> {
    let opcode: Oper = program[pc].try_into()?;
    Ok(opcode)
}

fn get_mode(pc: usize, mode: PMode, program: &[i64]) -> Result<i64, IntCodeErr> {
    match mode {
        PMode::Pos => get_pos(pc, program),
        PMode::Imm => get_imm(pc, program),
    }
}

fn get_pos(pc: usize, program: &[i64]) -> Result<i64, IntCodeErr> {
    let value = program
        .get(pc)
        .ok_or(IntCodeErr::OutOfBounds)
        .and_then(|&idx| usize::try_from(idx).map_err(|_| IntCodeErr::NegativeIndex))
        .and_then(|idx| program.get(idx).ok_or(IntCodeErr::OutOfBounds))?;
    Ok(*value)
}

fn get_imm(pc: usize, program: &[i64]) -> Result<i64, IntCodeErr> {
    let value = program.get(pc).ok_or(IntCodeErr::OutOfBounds)?;
    Ok(*value)
}

macro_rules! fetch_mut {
    ($pc:expr, $program:ident) => {{
        let idx = $program
            .get($pc)
            .ok_or(IntCodeErr::OutOfBounds)
            .and_then(|addr| usize::try_from(*addr).map_err(|_| IntCodeErr::NegativeIndex))?;
        $program.get_mut(idx).ok_or(IntCodeErr::OutOfBounds)
    }};
}

pub enum Exec {
    Ok(usize),
    Output(usize, i64),
    NeedInput,
    End,
}

impl Oper {
    fn execute(
        &self,
        mut pc: usize,
        program: &mut [i64],
        input: &mut Option<i64>,
    ) -> Result<Exec, IntCodeErr> {
        pc += match self.opcode {
            Op::Add => {
                let p1 = get_mode(pc + 1, self.p1, program)?;
                let p2 = get_mode(pc + 2, self.p2, program)?;
                *fetch_mut!(pc + 3, program)? = p1 + p2;
                4
            }
            Op::Mul => {
                let p1 = get_mode(pc + 1, self.p1, program)?;
                let p2 = get_mode(pc + 2, self.p2, program)?;
                *fetch_mut!(pc + 3, program)? = p1 * p2;
                4
            }
            Op::Inp => {
                if let Some(input) = input.take() {
                    *fetch_mut!(pc + 1, program)? = input;
                    2
                } else {
                    return Ok(Exec::NeedInput);
                }
            }
            Op::Out => {
                let out = get_mode(pc + 1, self.p1, program)?;
                return Ok(Exec::Output(pc + 2, out));
            }
            Op::Jit => {
                let p1 = get_mode(pc + 1, self.p1, program)?;
                if p1 != 0 {
                    let p2 = get_mode(pc + 2, self.p2, program).and_then(|idx| {
                        usize::try_from(idx).map_err(|_| IntCodeErr::NegativeIndex)
                    })?;
                    return Ok(Exec::Ok(p2));
                }
                3
            }
            Op::Jif => {
                let p1 = get_mode(pc + 1, self.p1, program)?;
                if p1 == 0 {
                    let p2 = get_mode(pc + 2, self.p2, program).and_then(|idx| {
                        usize::try_from(idx).map_err(|_| IntCodeErr::NegativeIndex)
                    })?;
                    return Ok(Exec::Ok(p2));
                }
                3
            }
            Op::Lt => {
                let p1 = get_mode(pc + 1, self.p1, program)?;
                let p2 = get_mode(pc + 2, self.p2, program)?;
                *fetch_mut!(pc + 3, program)? = i64::from(p1 < p2);
                4
            }
            Op::Eq => {
                let p1 = get_mode(pc + 1, self.p1, program)?;
                let p2 = get_mode(pc + 2, self.p2, program)?;
                *fetch_mut!(pc + 3, program)? = i64::from(p1 == p2);
                4
            }
            Op::End => return Ok(Exec::End),
        };
        Ok(Exec::Ok(pc))
    }
}

pub fn execute(
    pc: usize,
    program: &mut [i64],
    input: &mut Option<i64>,
) -> Result<Exec, IntCodeErr> {
    let opcode = get_op(pc, program)?;
    opcode.execute(pc, program, input)
}

#[cfg(test)]
mod tests {
    use crate::intcode::{execute, Exec};

    #[test]
    fn param_modes() {
        let mut program = vec![1001i64, 4, 11111, 0, 99];
        let Ok(Exec::Ok(new_pc)) = execute(0, &mut program, &mut None) else {
            panic!("unexpected error");
        };
        assert_eq!(new_pc, 4);
        assert_eq!(program[0], 99 + 11111);
    }

    #[test]
    fn input() {
        let mut program = vec![3, 2, 99];
        let mut input = Some(111);
        assert!(execute(0, &mut program, &mut input).is_ok());
        assert!(input.is_none());
        assert_eq!(program[2], 111);
    }

    #[test]
    fn output() {
        let mut program = vec![4, 2, 99];
        let Ok(Exec::Output(_, out)) = execute(0, &mut program, &mut None) else {
            panic!("unexpected no input.");
        };
        assert_eq!(out, 99);
    }
}
