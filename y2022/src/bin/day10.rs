use std::{
    fmt::{Display, Write},
    io,
};
use y2022::{fold_decimal, read_input, AoCTokenizer, Sentinel, Token};

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
                f.write_char('#')?;
            } else {
                f.write_char('.')?;
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
            (Vec::new(), Sentinel::Unset(Instruction::Noop), false),
            |(mut acc, instruction, is_neg), token| match token {
                Token::Something(word) if instruction.is_unset() => match word {
                    b"noop" => (acc, Sentinel::Value(Instruction::Noop), false),
                    b"addx" => (acc, Sentinel::Value(Instruction::Add), false),
                    _ => {
                        let word = word.escape_ascii();
                        panic!("Invalid instruction: {word}")
                    }
                },
                Token::Something(word) => {
                    let num = word.iter().fold(0, fold_decimal) * if is_neg { -1 } else { 1 };
                    (
                        acc,
                        instruction.map(|&instr| {
                            if instr == Instruction::Add {
                                Instruction::AddX(num)
                            } else {
                                instr
                            }
                        }),
                        false,
                    )
                }
                Token::Delimiter(is_neg) if is_neg == b'-' => (acc, instruction, true),
                Token::Newline | Token::End if !instruction.is_unset() => {
                    instruction.map_mv(|instr| {
                        if instr == Instruction::Add {
                            panic!("add operator does not have a value!.");
                        }
                        acc.push(instr);
                    });
                    (acc, Sentinel::Unset(Instruction::Noop), false)
                }
                _ => (acc, instruction, is_neg),
            },
        )
        .0
}

fn solve(input: &[Instruction]) -> (i64, CrtStates) {
    let mut sum_arr = vec![0i64; input.len() * 2];
    sum_arr[0] = 1;
    let mut x_reg = sum_arr[0];
    let mut pc = 0usize;
    input.iter().for_each(|&instr| {
        pc += 1;
        if let Instruction::AddX(val) = instr {
            sum_arr[pc] = x_reg;
            pc += 1;
            x_reg += val;
        }
        sum_arr[pc] = x_reg;
    });

    //println!("{sum_arr:?}");

    // let twenty = sum_arr[0..20].iter().sum::<i64>();
    // let sixty = twenty + sum_arr[20..60].iter().sum::<i64>();
    // let hundred = sixty + sum_arr[60..100].iter().sum::<i64>();
    // let hundred_forty = hundred + sum_arr[100..140].iter().sum::<i64>();
    // let hundred_eighty = hundred_forty + sum_arr[140..180].iter().sum::<i64>();
    // let two_hundred_twenty = hundred_eighty + sum_arr[180..220].iter().sum::<i64>();
    // let part1 = (twenty * 20)
    //     + (sixty * 60)
    //     + (hundred * 100)
    //     + (hundred_forty * 140)
    //     + (hundred_eighty * 180)
    //     + (two_hundred_twenty * 220);
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
