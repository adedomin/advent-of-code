use std::io;

use y2019::intcode::{brk, read_intcode, Input, IntCode};

enum Coord {
    XY(i64, i64),
    Y(i64),
    None,
}

impl Input for Coord {
    fn peek(&self) -> Option<i64> {
        match self {
            Coord::XY(x, _) => Some(*x),
            Coord::Y(y) => Some(*y),
            Coord::None => None,
        }
    }

    fn consume(&mut self) {
        match self {
            Coord::XY(_, y) => *self = Coord::Y(*y),
            Coord::Y(_) => *self = Coord::None,
            Coord::None => (),
        }
    }
}

fn find_tractor_beam(program: &[i64]) -> i64 {
    let mut output = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut input = Coord::XY(x, y);
            let mut intcode = IntCode::default();
            let mut prog = program.to_vec();
            loop {
                match intcode.execute_til(&mut prog, &mut input) {
                    Ok(out) => {
                        #[cfg(debug_assertions)]
                        if out == 1 {
                            print!("#");
                        } else {
                            print!(".");
                        }
                        output += out;
                    }
                    Err(y2019::intcode::IntCodeErr::OutOfBounds(oob)) => {
                        brk(oob, &mut prog).expect("Out of Memory.")
                    }
                    Err(y2019::intcode::IntCodeErr::End) => break,
                    Err(e) => panic!("Unexpected error: {e}"),
                }
            }
        }
        #[cfg(debug_assertions)]
        {
            println!();
        }
    }
    output
}

// TODO: beam shape does not look linear, better solution possible?
fn find_santa_space_sled_dist(program: Vec<i64>) -> i64 {
    let mut last_x = 0;
    // guess, beam shape likely won't fit a 100x100 til at least here.
    for y in 200.. {
        let first_x = (last_x..2i64.pow(20))
            .find(|&x| {
                let mut input = Coord::XY(x, y);
                let mut intcode = IntCode::default();
                let mut prog = program.to_vec();
                loop {
                    match intcode.execute_til(&mut prog, &mut input) {
                        Ok(1) => break true,
                        Ok(_) => break false,
                        Err(y2019::intcode::IntCodeErr::OutOfBounds(oob)) => {
                            brk(oob, &mut prog).expect("Out of Memory.")
                        }
                        Err(e) => panic!("Unexpected error: {e}"),
                    }
                }
            })
            .expect("Empty line post Y=100?");
        // draw a triangular base through the beam. should be big enough based on growth pattern being triangular like.
        debug_assert!(last_x <= first_x);
        last_x = first_x;
        let mut input = Coord::XY(first_x + 99, y - 99);
        let mut intcode = IntCode::default();
        let mut prog = program.to_vec();
        loop {
            match intcode.execute_til(&mut prog, &mut input) {
                Ok(1) => return first_x * 10000 + (y - 99),
                Ok(_) => break,
                Err(y2019::intcode::IntCodeErr::OutOfBounds(oob)) => {
                    brk(oob, &mut prog).expect("Out of Memory.")
                }
                Err(e) => panic!("Unexpected error: {e}"),
            }
        }
    }
    0
}

fn main() -> io::Result<()> {
    let program = read_intcode()?;
    let part1 = find_tractor_beam(&program);
    let part2 = find_santa_space_sled_dist(program);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
