use aoc_shared::read_input_to_string;
use itertools::Itertools;
use std::io;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl From<(&str, &str)> for Instruction {
    fn from(value: (&str, &str)) -> Self {
        let num = value
            .1
            .parse::<i32>()
            .expect("expected a parsable number here");
        match value.0 {
            "acc" => Self::Acc(num),
            "jmp" => Self::Jmp(num),
            _ => Self::Nop(num),
        }
    }
}

impl Instruction {
    fn toggle(self) -> Self {
        match self {
            Self::Acc(v) => Self::Acc(v),
            Self::Jmp(v) => Self::Nop(v),
            Self::Nop(v) => Self::Jmp(v),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Console {
    pc: i32,
    accumulate: i32,
}

impl Console {
    fn new() -> Self {
        Console {
            pc: 0,
            accumulate: 0,
        }
    }

    fn is_done(&self, instr: &[Instruction]) -> bool {
        instr.get(self.pc as usize).is_none()
    }

    fn step(self, instr: &[Instruction]) -> Result<Console, i32> {
        if self.pc < 0 {
            return Err(self.accumulate);
        }
        instr
            .get(self.pc as usize)
            .map(|inst| match inst {
                Instruction::Acc(num) => Console {
                    pc: self.pc + 1,
                    accumulate: self.accumulate + num,
                },
                Instruction::Jmp(num) => Console {
                    pc: self.pc + num,
                    accumulate: self.accumulate,
                },
                Instruction::Nop(_) => Console {
                    pc: self.pc + 1,
                    accumulate: self.accumulate,
                },
            })
            .ok_or(self.accumulate)
    }
}

type Output = Vec<Instruction>;
type Solved = i32;

fn parse_input(input: &str) -> Output {
    input
        .split_ascii_whitespace()
        .tuples()
        .map(|(x, y)| (x, y).into())
        .collect::<Vec<Instruction>>()
}

/// basic Floyd cycle detector. the program counter is used to determine cycle.
fn floyd_cycle(input: &Output) -> Result<(i32, i32), i32> {
    let mut tortoise = Console::new();
    let mut hare = Console::new();

    loop {
        tortoise = tortoise.step(input)?;
        hare = hare.step(input)?.step(input)?;
        if tortoise.pc == hare.pc {
            break;
        }
    }

    let mut mu = 0;
    tortoise = Console::new();
    loop {
        tortoise = tortoise.step(input)?;
        hare = hare.step(input)?;
        mu += 1;
        if tortoise.pc == hare.pc {
            break;
        }
    }

    let mut lambda = 0;
    hare = tortoise.step(input)?;
    loop {
        hare = hare.step(input)?;
        lambda += 1;
        if tortoise.pc == hare.pc {
            break;
        }
    }

    Ok((mu, lambda))
}

fn part1_sol(input: &Output) -> Solved {
    let (mu, lambda) = floyd_cycle(input).expect("There should be a cycle in this program.");
    // rerun a new program up to the first cycling instruction.
    let mut acc = Console::new();
    for _ in 0..(mu + lambda) {
        acc = acc.step(input).expect("to not terminate");
    }
    acc.accumulate
}

fn part2_sol(input: Output) -> Solved {
    for i in 0..input.len() {
        let mut input = input.clone();
        input[i] = input[i].toggle();
        match floyd_cycle(&input) {
            Ok(_) => (), // still contains a loop
            Err(acc) => return acc,
        }
    }
    panic!("Could not clear the cycle!");
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
