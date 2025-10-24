use std::io;

use y2019::intcode::{read_intcode, IntCode};

const NOUN_OFF: usize = 1;
const VERB_OFF: usize = 2;
#[allow(clippy::inconsistent_digit_grouping)]
const P2_TARGET: i64 = 1969_07_20;

fn cycle_machine(n: i64, v: i64, mut program: Vec<i64>) -> i64 {
    program[NOUN_OFF] = n;
    program[VERB_OFF] = v;

    let mut intcode = IntCode::default();
    while intcode.execute(&mut program, &mut None).is_ok() {}
    program[0]
}

fn main() -> io::Result<()> {
    let program = read_intcode()?;
    // part1
    let part1 = cycle_machine(12, 2, program.clone());
    print!("Part1: {part1} ");
    // part2
    let part2 = (0..99)
        .flat_map(|v| (0..99).map(move |n| (n, v)))
        .find(|&(n, v)| cycle_machine(n, v, program.clone()) == P2_TARGET)
        .expect("One pair of values to reach {P2_TARGET}");
    println!("Part2: {}", 100 * part2.0 + part2.1);
    Ok(())
}
