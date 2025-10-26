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
    Rel,
}

impl TryFrom<i64> for PMode {
    type Error = IntCodeErr;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Pos,
            1 => Self::Imm,
            2 => Self::Rel,
            _ => return Err(IntCodeErr::InvalidParamMode(value)),
        })
    }
}

#[derive(Debug)]
enum Op {
    Add = 1,
    Mul = 2,
    Inp = 3,
    Out = 4,
    Jit = 5,
    Jif = 6,
    Lt = 7,
    Eq = 8,
    Rba = 9,
    End = 99,
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
    #[error("Out of bounds fetching operands: {0}")]
    OutOfBounds(usize),
    #[error("Need input.")]
    NeedInput,
    #[error("Negative index.")]
    NegativeIndex,
    #[error("Invalid Parameter Mode: {0}")]
    InvalidParamMode(i64),
    #[error("Output mode cannot be immediate.")]
    ImmediateOutputMode,
    #[error("New Program Break too large, more than 4GiB!")]
    BrkTooLarge,
    #[error("Program completed.")]
    End,
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
        let op = value % 10i64.pow(2);
        let opcode = match op {
            1 => Op::Add,
            2 => Op::Mul,
            3 => Op::Inp,
            4 => Op::Out,
            5 => Op::Jit,
            6 => Op::Jif,
            7 => Op::Lt,
            8 => Op::Eq,
            9 => Op::Rba,
            99 => Op::End,
            value => return Err(IntCodeErr::InvalidOp(value)),
        };
        Ok(Self { p1, p2, p3, opcode })
    }
}

fn from_intcode(i: i64) -> Result<usize, IntCodeErr> {
    usize::try_from(i).map_err(|_| IntCodeErr::NegativeIndex)
}

fn get_op(pc: usize, program: &[i64]) -> Result<Oper, IntCodeErr> {
    let opcode: Oper = program[pc].try_into()?;
    Ok(opcode)
}

fn get_mode(pc: usize, rb: i64, mode: PMode, program: &[i64]) -> Result<i64, IntCodeErr> {
    match mode {
        PMode::Pos => get_pos(pc, 0, program),
        PMode::Rel => get_pos(pc, rb, program),
        PMode::Imm => get_imm(pc, program),
    }
}

fn get_pos(pc: usize, offset: i64, program: &[i64]) -> Result<i64, IntCodeErr> {
    let value = program
        .get(pc)
        .ok_or(IntCodeErr::OutOfBounds(pc))
        .and_then(|&idx| from_intcode(idx + offset))
        .and_then(|idx| program.get(idx).ok_or(IntCodeErr::OutOfBounds(idx)))?;
    Ok(*value)
}

fn get_imm(pc: usize, program: &[i64]) -> Result<i64, IntCodeErr> {
    let value = program.get(pc).ok_or(IntCodeErr::OutOfBounds(pc))?;
    Ok(*value)
}

fn set_mode(
    pc: usize,
    rb: i64,
    mode: PMode,
    program: &mut [i64],
    value: i64,
) -> Result<(), IntCodeErr> {
    match mode {
        PMode::Pos => set_pos(pc, 0, program, value),
        PMode::Rel => set_pos(pc, rb, program, value),
        PMode::Imm => Err(IntCodeErr::ImmediateOutputMode),
    }
}

fn set_pos(pc: usize, offset: i64, program: &mut [i64], update: i64) -> Result<(), IntCodeErr> {
    let value = program
        .get(pc)
        .ok_or(IntCodeErr::OutOfBounds(pc))
        .and_then(|&idx| from_intcode(idx + offset))
        .and_then(|idx| program.get_mut(idx).ok_or(IntCodeErr::OutOfBounds(idx)))?;
    *value = update;
    Ok(())
}

#[derive(Copy, Clone, Default, Debug)]
pub struct IntCode {
    pub pc: usize,
    pub rb: i64,
}

/// 4GiB
const MAX_ALLOWED_BRK: usize = 2usize.pow(32);

/// Resize program break to fit the out of bounds index.
/// All new cells are zero filled.
pub fn brk(oob: usize, program: &mut Vec<i64>) -> Result<(), IntCodeErr> {
    if oob.saturating_add(1) > MAX_ALLOWED_BRK {
        return Err(IntCodeErr::BrkTooLarge);
    }
    program.resize_with(oob + 1, i64::default);
    Ok(())
}

macro_rules! arith_op {
    ($self:ident, $p1:ident, $p2:ident, $p3:ident, $program:ident, $oper:path) => {{
        let p1 = get_mode($self.pc + 1, $self.rb, $p1, $program)?;
        let p2 = get_mode($self.pc + 2, $self.rb, $p2, $program)?;
        set_mode($self.pc + 3, $self.rb, $p3, $program, $oper(p1, p2))?;
        4
    }};
}

macro_rules! jmp_op {
    ($self:ident, $p1:ident, $p2:ident, $program:ident,  $oper:path) => {{
        let p1 = get_mode($self.pc + 1, $self.rb, $p1, $program)?;
        if $oper(p1, 0) {
            let p2 = get_mode($self.pc + 2, $self.rb, $p2, $program).and_then(from_intcode)?;
            $self.pc = p2;
            return Ok(None);
        }
        3
    }};
}

fn lt(p1: i64, p2: i64) -> i64 {
    i64::from(p1 < p2)
}

fn eq(p1: i64, p2: i64) -> i64 {
    i64::from(p1 == p2)
}

fn jeq(p1: i64, p2: i64) -> bool {
    p1 == p2
}

fn jne(p1: i64, p2: i64) -> bool {
    p1 != p2
}

impl IntCode {
    /// Calls `execute` repeatedly til the machine yields, due to IO or Error.
    pub fn execute_til(
        &mut self,
        program: &mut [i64],
        input: &mut Option<i64>,
    ) -> Result<i64, IntCodeErr> {
        loop {
            match self.execute(program, input) {
                Ok(None) => (),
                Ok(Some(out)) => break Ok(out),
                Err(e) => break Err(e),
            }
        }
    }
    /// Execute one operation in the machine.
    /// If the machine errors with anything other than `IntCodeErr::End`
    /// the machine can be resumed if you handle it. The machine will not be changed otherwise.
    ///
    /// e.g. Set input to Some(value) if one gets `IntCodeErr::NeedInput`
    ///      increase the program break with `brk` if you get `IntCodeErr::OutOfBounds`
    pub fn execute(
        &mut self,
        program: &mut [i64],
        input: &mut Option<i64>,
    ) -> Result<Option<i64>, IntCodeErr> {
        let Oper { p1, p2, p3, opcode } = get_op(self.pc, program)?;
        self.pc += match opcode {
            Op::Add => arith_op!(self, p1, p2, p3, program, std::ops::Add::add),
            Op::Mul => arith_op!(self, p1, p2, p3, program, std::ops::Mul::mul),
            Op::Inp => {
                if let Some(input) = input.take() {
                    set_mode(self.pc + 1, self.rb, p1, program, input)?;
                    2
                } else {
                    return Err(IntCodeErr::NeedInput);
                }
            }
            Op::Out => {
                let out = get_mode(self.pc + 1, self.rb, p1, program)?;
                self.pc += 2;
                return Ok(Some(out));
            }
            Op::Jit => jmp_op!(self, p1, p2, program, jne),
            Op::Jif => jmp_op!(self, p1, p2, program, jeq),
            Op::Lt => arith_op!(self, p1, p2, p3, program, lt),
            Op::Eq => arith_op!(self, p1, p2, p3, program, eq),
            Op::Rba => {
                let p1 = get_mode(self.pc + 1, self.rb, p1, program)?;
                self.rb += p1;
                2
            }
            Op::End => return Err(IntCodeErr::End),
        };
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::intcode::{IntCode, IntCodeErr};

    #[test]
    fn immediate_mode() {
        let mut program = vec![1001i64, 4, 11111, 0, 99];
        let mut intcode = IntCode::default();
        let Ok(None) = intcode.execute(&mut program, &mut None) else {
            panic!("unexpected error");
        };
        assert_eq!(intcode.pc, 4);
        assert_eq!(program[0], 99 + 11111);
    }

    #[test]
    fn input() {
        let mut program = vec![3, 2, 99];
        let mut input = Some(111);
        let mut intcode = IntCode::default();
        assert!(intcode.execute(&mut program, &mut input).is_ok());
        assert!(input.is_none());
        assert_eq!(program[2], 111);
    }

    #[test]
    fn output() {
        let mut program = vec![4, 2, 99];
        let mut intcode = IntCode::default();
        let Ok(Some(out)) = intcode.execute(&mut program, &mut None) else {
            panic!("unexpected no input.");
        };
        assert_eq!(out, 99);
    }

    #[test]
    fn relative_base_and_mode() {
        let mut program = vec![109, 19, 204, -34];
        let mut intcode = IntCode { pc: 0, rb: 2000 };
        intcode
            .execute(&mut program, &mut None)
            .expect("No errors.");
        assert_eq!(intcode.rb, 2019);
        let Err(IntCodeErr::OutOfBounds(1985)) = intcode.execute(&mut program, &mut None) else {
            panic!("Did not deref base address correctly; should be asking for memory index 1985.");
        };
    }
}
