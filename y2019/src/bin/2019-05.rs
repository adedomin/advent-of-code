use std::io;

use y2019::intcode::{execute, read_intcode, Exec};

fn run_program(mut program: Vec<i64>, i: i64) -> i64 {
    let mut pc = 0usize;
    let mut input = None;
    let mut last = 0;
    loop {
        match execute(pc, &mut program, &mut input).expect("No execution errors") {
            Exec::Ok(new_pc) => pc = new_pc,

            Exec::Output(new_pc, output) => {
                pc = new_pc;
                #[cfg(debug_assertions)]
                {
                    println!("Diag: {output}");
                }
                last = output;
            }
            Exec::NeedInput => input = Some(i),
            Exec::End => break,
        }
    }
    last
}

fn main() -> io::Result<()> {
    let program = read_intcode()?;
    let part1 = run_program(program.clone(), 1);
    let part2 = run_program(program.clone(), 5);
    println!("Part1 {part1}, Part2: {part2}");
    Ok(())
}
