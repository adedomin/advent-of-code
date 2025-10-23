use std::io;

use itertools::Itertools;
use y2019::intcode::{execute, read_intcode, Exec};

fn run_program(program: &[i64]) -> i64 {
    (0..5).permutations(5).fold(i64::MIN, |acc, config_inputs| {
        // let config_inputs = [0i64, 1, 2, 3, 4];
        let mut last = 0;
        config_inputs.into_iter().for_each(|input| {
            let mut pc = 0usize;
            let mut program_cpy = program.to_vec();
            let mut machine_input = Some(input);
            let mut out = 0;
            last = loop {
                match execute(pc, &mut program_cpy, &mut machine_input)
                    .expect("No execution errors")
                {
                    Exec::Ok(new_pc) => pc = new_pc,

                    Exec::Output(new_pc, output) => {
                        pc = new_pc;
                        out = output;
                    }
                    Exec::NeedInput => {
                        machine_input = Some(last);
                    }
                    Exec::End => break out,
                }
            }
        });
        std::cmp::max(last, acc)
    })
}

struct Feedback {
    pc: usize,
    input: Option<i64>,
    program: Vec<i64>,
}

fn run_feedback(Feedback { pc, input, program }: &mut Feedback, last_sig: i64) -> Option<i64> {
    loop {
        match execute(*pc, program, input).expect("No execution errors") {
            Exec::Ok(new_pc) => *pc = new_pc,

            Exec::Output(new_pc, out) => {
                *pc = new_pc;
                return Some(out);
            }
            Exec::NeedInput => {
                *input = Some(last_sig);
            }
            Exec::End => return None,
        }
    }
}

fn feedback(program: &[i64]) -> i64 {
    (5..10)
        .permutations(5)
        .fold(i64::MIN, |acc, config_inputs| {
            // let config_inputs = [0i64, 1, 2, 3, 4];
            let mut amps = config_inputs
                .into_iter()
                .map(|phase| Feedback {
                    pc: 0,
                    input: Some(phase),
                    program: program.to_vec(),
                })
                .collect::<Vec<_>>();
            let amps_len = amps.len();
            let mut m_idx = 0;
            let last = std::iter::successors(Some(0i64), move |&last| {
                let m = &mut amps[m_idx];
                m_idx = (m_idx + 1) % amps_len;
                run_feedback(m, last)
            })
            .last()
            .unwrap(); // never fails, is always Some(0) at a minimum.
            std::cmp::max(last, acc)
        })
}

fn main() -> io::Result<()> {
    let program = read_intcode()?;
    let part1 = run_program(&program);
    let part2 = feedback(&program);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
