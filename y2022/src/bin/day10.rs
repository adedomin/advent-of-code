use std::{
    fmt::{Display, Write},
    io,
};
use y2022::{fold_decimal, read_input, AoCTokenizer, Token};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Add,
    AddX(i64),
}

const CRT_W: usize = 40;
const CRT_H: usize = 6;
const CRT_DIM: usize = CRT_W * CRT_H;
struct CrtStates(Vec<i64>);

impl Display for CrtStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &x_reg) in self.0[0..CRT_DIM].iter().enumerate() {
            let col = (i % CRT_W) as i64;

            if x_reg - 1 == col || x_reg == col || x_reg + 1 == col {
                f.write_char('█')?;
            } else {
                f.write_char('▁')?;
            }

            if col == 39 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

fn parse_input(input: Vec<u8>) -> Vec<Instruction> {
    let tokenizer = AoCTokenizer::new(&input);
    tokenizer
        .fold(
            (Vec::new(), None, false),
            |(mut acc, instruction, is_neg), token| match token {
                Token::Something(word) if instruction.is_none() => match word {
                    b"noop" => {
                        acc.push(Instruction::Noop);
                        (acc, None, false)
                    }
                    b"addx" => {
                        acc.push(Instruction::Add);
                        (acc, Some(Instruction::Add), false)
                    }
                    _ => {
                        let word = word.escape_ascii();
                        panic!("Invalid instruction: {word}")
                    }
                },
                Token::Something(word) => {
                    let num = word.iter().fold(0, fold_decimal) * if is_neg { -1 } else { 1 };
                    if let Some(instr) = instruction {
                        if instr == Instruction::Add {
                            acc.push(Instruction::AddX(num));
                        }
                    }
                    (acc, None, false)
                }
                Token::Delimiter(delim) if instruction.is_some() => {
                    (acc, instruction, delim == b'-')
                }
                _ => (acc, instruction, is_neg),
            },
        )
        .0
}

fn solve(input: &[Instruction]) -> (i64, CrtStates) {
    let sum_arr = input
        .iter()
        .fold((vec![1], 1i64), |(mut acc, sum), &instr| {
            let sum = if let Instruction::AddX(val) = instr {
                val + sum
            } else {
                sum
            };
            acc.push(sum);
            (acc, sum)
        })
        .0;

    let part1 = (sum_arr[19] * 20)
        + (sum_arr[59] * 60)
        + (sum_arr[99] * 100)
        + (sum_arr[139] * 140)
        + (sum_arr[179] * 180)
        + (sum_arr[219] * 220);

    (part1, CrtStates(sum_arr))
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed = parse_input(input);
    let (part1, part2) = solve(&parsed);
    print!("Part1: {part1}, Part2:\n{part2}");
    Ok(())
}
