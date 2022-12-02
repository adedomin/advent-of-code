#![feature(if_let_guard)]

use std::io;
use y2022::{read_input, AoCTokenizer, Token};

const LHS_PLAY: u8 = b'A' - 1u8;
// const LHS_START: u8 = b'A';
const RHS_PLAY: u8 = b'X' - 1u8;
// const RHS_START: u8 = b'X';
const LOSE: u64 = 0;
const DRAW: u64 = 3;
const WIN: u64 = 6;

const ROCK: u64 = 1;
const PAPER: u64 = 2;
const SCISSORS: u64 = 3;

fn main() -> io::Result<()> {
    let input = read_input()?;
    let tokenizer = AoCTokenizer::new(&input);
    let (part1, part2, _) = tokenizer.fold((0, 0, 0), |(acc, acc2, partial), token| match token {
        Token::Something(val) => {
            let val = val[0];
            if RHS_PLAY < val {
                let point_val = (val - RHS_PLAY) as u64;
                let (round_pts, pt2_round_pts) = match (partial, point_val) {
                    // Generated with the following:
                    // ```zsh
                    // print -lr '('{ROCK,PAPER,SCISSORS}', '{'ROCK) => (_ + ROCK, LOSE + _)','PAPER) => (_ + PAPER, DRAW + _)','SCISSORS) => (_ + SCISSORS, WIN + _)'}
                    // ```
                    (ROCK, ROCK) => (DRAW + ROCK, LOSE + SCISSORS),
                    (ROCK, PAPER) => (WIN + PAPER, DRAW + ROCK),
                    (ROCK, SCISSORS) => (LOSE + SCISSORS, WIN + PAPER),
                    (PAPER, ROCK) => (LOSE + ROCK, LOSE + ROCK),
                    (PAPER, PAPER) => (DRAW + PAPER, DRAW + PAPER),
                    (PAPER, SCISSORS) => (WIN + SCISSORS, WIN + SCISSORS),
                    (SCISSORS, ROCK) => (WIN + ROCK, LOSE + PAPER),
                    (SCISSORS, PAPER) => (LOSE + PAPER, DRAW + SCISSORS),
                    (SCISSORS, SCISSORS) => (DRAW + SCISSORS, WIN + ROCK),
                    _ => panic!("Invalid input of {val}"),
                };
                (acc + round_pts, acc2 + pt2_round_pts, 0)
            } else {
                let point_val = (val - LHS_PLAY) as u64;
                (acc, acc2, point_val)
            }
        }
        _ => (acc, acc2, partial),
    });
    println!("Part 1: {part1}, Part 2: {part2}");
    Ok(())
}
