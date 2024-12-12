use aoc_shared::{fold_decimal_from, read_input_to_string};
use std::io;

type IntCode = Vec<i64>;

pub fn read_intcode() -> io::Result<IntCode> {
    let values = read_input_to_string()?;
    Ok(values
        .split(|chr: char| !chr.is_ascii_digit())
        .map(|num| fold_decimal_from(num.as_bytes()))
        .collect::<IntCode>())
}

pub enum Op {
    Add = 1,
    Mul = 2,
    End = 99,
}

#[derive(thiserror::Error, Debug)]
pub enum IntCodeErr {
    #[error("Received an invalid operation: {0}")]
    InvalidOp(i64),
    #[error("End of program.")]
    End,
}

impl TryFrom<i64> for Op {
    type Error = IntCodeErr;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Op::Add),
            2 => Ok(Op::Mul),
            99 => Ok(Op::End),
            value => Err(IntCodeErr::InvalidOp(value)),
        }
    }
}

impl Op {
    fn execute(&self, pc: usize, program: &mut [i64]) -> Result<usize, IntCodeErr> {
        let fetch2 = |pc| program[pc..pc + 2].iter().map(|&idx| program[idx as usize]);
        match self {
            Op::Add => {
                program[program[pc + 3] as usize] = fetch2(pc + 1).sum::<i64>();
                Ok(pc + 4)
            }
            Op::Mul => {
                program[program[pc + 3] as usize] = fetch2(pc + 1).product::<i64>();
                Ok(pc + 4)
            }
            Op::End => Err(IntCodeErr::End),
        }
    }
}

pub fn execute(pc: usize, program: &mut [i64]) -> Result<usize, IntCodeErr> {
    let opcode: Op = program[pc].try_into()?;
    opcode.execute(pc, program)
}
