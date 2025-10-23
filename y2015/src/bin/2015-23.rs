use aoc_shared::read_input_to_string;
use std::io;

type Output = Vec<Instruction>;
type Solved = u32;

#[derive(Debug)]
enum Instruction {
    HlfA,
    HlfB,
    TplA,
    TplB,
    IncA,
    IncB,
    Jmp(i32),
    JieA(i32),
    JieB(i32),
    JioA(i32),
    JioB(i32),
}

#[derive(Clone, Copy, Debug)]
struct Computer {
    pc: i32,
    a: u32,
    b: u32,
}

impl Computer {
    // looks a lot like Collatz conjecture.
    fn step(self, instructions: &[Instruction]) -> Result<Computer, u32> {
        if self.pc < 0 {
            return Err(self.b);
        }
        instructions
            .get(self.pc as usize)
            .map(|ins| match ins {
                Instruction::HlfA => Computer {
                    pc: self.pc + 1,
                    a: self.a / 2,
                    b: self.b,
                },
                Instruction::HlfB => Computer {
                    pc: self.pc + 1,
                    a: self.a,
                    b: self.b / 2,
                },
                Instruction::TplA => Computer {
                    pc: self.pc + 1,
                    a: self.a * 3,
                    b: self.b,
                },
                Instruction::TplB => Computer {
                    pc: self.pc + 1,
                    a: self.a,
                    b: self.b * 3,
                },
                Instruction::IncA => Computer {
                    pc: self.pc + 1,
                    a: self.a + 1,
                    b: self.b,
                },
                Instruction::IncB => Computer {
                    pc: self.pc + 1,
                    a: self.a,
                    b: self.b + 1,
                },
                Instruction::Jmp(addr) => Computer {
                    pc: self.pc + addr,
                    ..self
                },
                Instruction::JieA(addr) => Computer {
                    pc: self.pc + if self.a.is_multiple_of(2) { *addr } else { 1 },
                    ..self
                },
                Instruction::JieB(addr) => Computer {
                    pc: self.pc + if self.b.is_multiple_of(2) { *addr } else { 1 },
                    ..self
                },
                Instruction::JioA(addr) => Computer {
                    pc: self.pc + if self.a == 1 { *addr } else { 1 },
                    ..self
                },
                Instruction::JioB(addr) => Computer {
                    pc: self.pc + if self.b == 1 { *addr } else { 1 },
                    ..self
                },
            })
            .ok_or(self.b)
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let split = value
            .split_terminator(|c: char| c.is_whitespace() || c == ',')
            .filter(|sub| !sub.is_empty())
            .collect::<Vec<&str>>();
        match split[0] {
            "hlf" if split[1] == "a" => Self::HlfA,
            "hlf" if split[1] == "b" => Self::HlfB,
            "tpl" if split[1] == "a" => Self::TplA,
            "tpl" if split[1] == "b" => Self::TplB,
            "inc" if split[1] == "a" => Self::IncA,
            "inc" if split[1] == "b" => Self::IncB,
            "jmp" => Self::Jmp(split[1].parse::<_>().expect("Valid numeric address")),
            "jie" if split[1] == "a" => {
                Self::JieA(split[2].parse::<_>().expect("Valid numeric address"))
            }
            "jie" if split[1] == "b" => {
                Self::JieB(split[2].parse::<_>().expect("Valid numeric address"))
            }
            "jio" if split[1] == "a" => {
                Self::JioA(split[2].parse::<_>().expect("Valid numeric address"))
            }
            "jio" if split[1] == "b" => {
                Self::JioB(split[2].parse::<_>().expect("Valid numeric address"))
            }
            _ => panic!("Invalid input: {value}"),
        }
    }
}

fn parse_input(input: &str) -> Output {
    input
        .split('\n')
        .filter_map(|s| if s.is_empty() { None } else { Some(s.into()) })
        .collect::<Vec<_>>()
}

fn part1_sol<const A_START: u32>(input: &Output) -> Solved {
    let mut computer = Computer {
        pc: 0,
        a: A_START,
        b: 0,
    };
    loop {
        match computer.step(input) {
            Ok(com) => computer = com,
            Err(res) => break res,
        }
    }
}

// fn part2_sol(input: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol::<0>(&parsed_input);
    let part2 = part1_sol::<1>(&parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
