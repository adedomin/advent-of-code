use std::{
    collections::VecDeque,
    io::{self, Write as _},
};

use y2019::intcode::{brk, read_intcode, IntCode};

// (     we must jump if 3 tiles ahead is a hole: NOT C T because there are "islands like #.#.#
//   AND we must land 4 tiles ahead if we are to jump: AND D T, while concerned about island
// ) OR  we must jump if 1 tile ahead is a hole
const P1_SPRING_SCRIPT: &str = "\
NOT C T
AND D T
OR T J
NOT A T
OR T J
WALK
";

// I'll be honest, I just started with P1 and just shoved instructions in til it worked.
//
// Same as p1, jump onto islands with one new caveat.
// one of the island problems looked like: #.#.##.##.#
// we need to check 8 (H) ahead and cancel out our C & D Jump condition if true.
//
// Another Island pair: #.##.##..#
// forces us to jump right away after landing on #.#<-#. by checking for the dead spot in 2 (B)
// But only, again, if D (4) can be landed on.
const P2_SPRING_SCRIPT: &str = "\
NOT C T
AND D T
OR T J
AND H J
NOT B T
AND D T
OR T J
NOT A T
OR T J
RUN
";

fn solve(mut program: Vec<i64>, spring_script: &str) -> i64 {
    let mut output = 0;
    let mut springscript = VecDeque::new();
    write!(springscript, "{spring_script}").unwrap();
    let mut intcode = IntCode::default();
    loop {
        match intcode.execute_til(&mut program, &mut springscript) {
            Ok(out) => {
                if out / 2i64.pow(7) > 0 {
                    output = out;
                } else {
                    #[cfg(debug_assertions)]
                    print!("{}", char::from(out as u8));
                }
            }
            Err(y2019::intcode::IntCodeErr::OutOfBounds(oob)) => {
                brk(oob, &mut program).expect("Out of Memory.")
            }
            Err(y2019::intcode::IntCodeErr::End) => break,
            Err(e) => panic!("Unexpected error: {e}"),
        }
    }
    output
}

fn main() -> io::Result<()> {
    let program = read_intcode()?;
    let part1 = solve(program.clone(), P1_SPRING_SCRIPT);
    let part2 = solve(program, P2_SPRING_SCRIPT);
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
