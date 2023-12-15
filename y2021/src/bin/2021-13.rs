use std::{
    collections::HashSet,
    io::{self, Write},
};

use aoc_shared::{fold_decimal, read_input, AoCTokenizer, Token};

#[derive(Copy, Clone, Debug)]
enum Instruction {
    FoldUp(i32),
    FoldLeft(i32),
}

fn parse(input: Vec<u8>) -> (Vec<Instruction>, Vec<(i32, i32)>) {
    let mut instructions = vec![];
    let mut paper = Vec::<(i32, i32)>::new();

    let mut is_instruction = false;
    let (mut x, mut y) = (-1, -1);

    for token in AoCTokenizer::new(&input) {
        match token {
            Token::Something(word) if word.eq(b"fold") || word.eq(b"along") => {
                is_instruction = true;
            }
            Token::Something(value) if !is_instruction && x == -1 => {
                x = value.iter().fold(0i32, fold_decimal);
            }
            Token::Something(value) if !is_instruction && y == -1 => {
                y = value.iter().fold(0i32, fold_decimal);
            }
            Token::Something(word) => {
                if word[0] == b'y' {
                    x = -2;
                }
            }
            Token::Delimiter(_) => is_instruction = false,
            Token::Newline | Token::DoubleNewline => {
                if x > -1 && y > -1 {
                    paper.push((x, y));
                } else if x > -1 {
                    instructions.push(Instruction::FoldLeft(x));
                } else if y > -1 {
                    instructions.push(Instruction::FoldUp(y));
                }
                x = -1;
                y = -1;
                is_instruction = false;
            }
            Token::Space => (),
            Token::End => (),
        }
    }

    (instructions, paper)
}

fn transform_paper(instruction: Instruction, paper: &mut [(i32, i32)]) {
    for (x, y) in paper.iter_mut() {
        match instruction {
            Instruction::FoldUp(ypos) => {
                // y.cmp(ypos) IS NOT THE SAME THING
                #[allow(clippy::comparison_chain)]
                if *y > ypos {
                    *y -= (y.abs_diff(ypos) * 2) as i32;
                } else if *y == ypos {
                    *y = -1;
                }
            }
            Instruction::FoldLeft(xpos) => {
                // x.cmp(xpos) IS NOT THE SAME THING
                #[allow(clippy::comparison_chain)]
                if *x > xpos {
                    *x -= (x.abs_diff(xpos) * 2) as i32;
                } else if *x == xpos {
                    *x = -1;
                }
            }
        }
    }
}

fn solve(instructions: Vec<Instruction>, mut paper: Vec<(i32, i32)>) -> io::Result<()> {
    // part1
    transform_paper(instructions[0], &mut paper);
    let p1 = paper
        .clone()
        .drain(..)
        .filter(|&(x, y)| x > -1 && y > -1)
        .collect::<HashSet<(i32, i32)>>()
        .len();
    println!("Part1 {}", p1);
    instructions.iter().skip(1).for_each(|&instruction| {
        transform_paper(instruction, &mut paper);
    });
    let &(max_x, _) = paper
        .iter()
        .max_by(|&(lx, _), &(rx, _)| lx.cmp(rx))
        .unwrap();
    let &(_, max_y) = paper
        .iter()
        .max_by(|&(_, ly), &(_, ry)| ly.cmp(ry))
        .unwrap();
    let mut display = vec![vec![b'.'; (max_x + 1) as usize]; (max_y + 1) as usize];
    paper.iter().for_each(|&(x, y)| {
        display[y as usize][x as usize] = b'#';
    });
    println!("Part2 Below:");
    for line in display {
        io::stdout().lock().write_all(&line)?;
        io::stdout().lock().write_all(&[b'\n'])?;
    }
    Ok(())
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let (instructions, paper) = parse(input);
    solve(instructions, paper)?;
    Ok(())
}
