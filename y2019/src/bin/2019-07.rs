use std::io;

use itertools::Itertools;
use y2019::intcode::{read_intcode, IntCode, IntCodeErr};

fn run_program(program: &[i64]) -> i64 {
    (0..5).permutations(5).fold(i64::MIN, |acc, config_inputs| {
        // let config_inputs = [0i64, 1, 2, 3, 4];
        let mut last = 0;
        config_inputs.into_iter().for_each(|input| {
            let mut intcode = IntCode::default();
            let mut program_cpy = program.to_vec();
            let mut machine_input = Some(input);
            let mut out = 0;
            last = loop {
                match intcode.execute(&mut program_cpy, &mut machine_input) {
                    Ok(None) => (),
                    Ok(Some(output)) => out = output,
                    Err(IntCodeErr::NeedInput) => machine_input = Some(last),
                    Err(IntCodeErr::End) => break out,
                    Err(e) => panic!("{e}"),
                }
            }
        });
        last.max(acc)
    })
}

struct Feedback {
    intcode: IntCode,
    input: Option<i64>,
    program: Vec<i64>,
}

fn run_feedback(
    Feedback {
        intcode,
        input,
        program,
    }: &mut Feedback,
    last_sig: i64,
) -> Option<i64> {
    loop {
        match intcode.execute(program, input) {
            Ok(None) => (),
            Ok(out) => return out,
            Err(IntCodeErr::NeedInput) => *input = Some(last_sig),
            Err(IntCodeErr::End) => return None,
            Err(e) => panic!("{e}"),
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
                    intcode: IntCode::default(),
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
            last.max(acc)
        })
}

fn main() -> io::Result<()> {
    let program = read_intcode()?;
    let part1 = run_program(&program);
    let part2 = feedback(&program);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
