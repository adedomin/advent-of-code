use std::io;

use y2019::intcode::{brk, read_intcode, IntCode, IntCodeErr};

fn run_program(mut program: Vec<i64>, i: i64) -> i64 {
    let mut intcode = IntCode::default();
    let mut input = Some(i);
    let mut last = 0;
    loop {
        match intcode.execute(&mut program, &mut input) {
            Ok(None) => (),
            Ok(Some(output)) => {
                #[cfg(debug_assertions)]
                {
                    println!("Diag: {output}");
                }
                last = output;
            }
            Err(IntCodeErr::NeedInput) => input = Some(i),
            Err(IntCodeErr::End) => break,
            Err(IntCodeErr::OutOfBounds(fault)) => {
                brk(fault, &mut program).expect("To resize the program break.")
            }
            Err(e) => panic!("{e}"),
        }
    }
    last
}

fn main() -> io::Result<()> {
    let program = read_intcode()?;
    let part1 = run_program(program.clone(), 1);
    let part2 = run_program(program.clone(), 2);
    println!("Part1 {part1}, Part2 {part2}");
    Ok(())
}
