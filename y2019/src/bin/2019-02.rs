use std::io;

use y2019::intcode::{execute, read_intcode, IntCodeErr};

const NOUN_OFF: usize = 1;
const VERB_OFF: usize = 2;
#[allow(clippy::inconsistent_digit_grouping)]
const P2_TARGET: i64 = 1969_07_20;

fn cycle_machine(n: i64, v: i64, mem: &[i64]) -> i64 {
    let mut mem = mem.to_vec();
    mem[NOUN_OFF] = n;
    mem[VERB_OFF] = v;

    let mut pc = 0;
    loop {
        match execute(pc, &mut mem) {
            Ok(new_pc) => pc = new_pc,
            Err(IntCodeErr::End) => break,
            Err(e) => panic!("{e}"),
        }
    }
    mem[0]
}

fn main() -> io::Result<()> {
    let parsed_input = read_intcode()?;
    // part1
    let part1 = cycle_machine(12, 2, &parsed_input);
    print!("Part1: {part1} ");
    // part2
    let part2 = (0..99)
        .flat_map(|v| (0..99).map(move |n| (n, v)))
        .find(|&(n, v)| cycle_machine(n, v, &parsed_input) == P2_TARGET)
        .expect("One pair of values to reach {P2_TARGET}");
    println!("Part2: {}", 100 * part2.0 + part2.1);
    Ok(())
}
