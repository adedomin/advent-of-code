use std::{collections::HashMap, io};

use y2019::intcode::{read_intcode, IntCode, IntCodeErr};

const MAX_MEM: usize = 2usize.pow(32);

fn rot_left(xy: (isize, isize)) -> (isize, isize) {
    rot_right(rot_right(rot_right(xy)))
}

fn rot_right((x, y): (isize, isize)) -> (isize, isize) {
    (-y, x)
}

enum OutState {
    PaintColor,
    Rotate,
}

fn run_program(mut program: Vec<i64>, start: i64) -> i64 {
    let (mut x, mut y) = (0isize, 0isize);
    let (mut dx, mut dy) = (0isize, -1isize); // pointing up
    let (mut minx, mut miny, mut maxx) = (0isize, 0isize, 0isize); // for output mapping

    let mut panels: HashMap<(isize, isize), bool> = HashMap::default();
    let mut uniq_painted = 0;

    let mut intcode = IntCode::default();
    let mut input = Some(start);
    let mut outstate = OutState::PaintColor;
    loop {
        match intcode.execute(&mut program, &mut input) {
            Ok(None) => (),
            Ok(Some(output)) => {
                outstate = match outstate {
                    OutState::PaintColor => {
                        if let Some(panel) = panels.get_mut(&(x, y)) {
                            *panel = output == 1
                        } else {
                            panels.insert((x, y), output == 1);
                            uniq_painted += 1;
                        }
                        OutState::Rotate
                    }
                    OutState::Rotate => {
                        (dx, dy) = if output == 0 {
                            rot_left((dx, dy))
                        } else {
                            rot_right((dx, dy))
                        };
                        x += dx;
                        y += dy;
                        minx = std::cmp::min(minx, x);
                        miny = std::cmp::min(miny, y);
                        maxx = std::cmp::max(maxx, x);
                        OutState::PaintColor
                    }
                };
            }
            Err(IntCodeErr::NeedInput) => {
                if let Some(panel) = panels.get(&(x, y)) {
                    input = Some(*panel as i64);
                } else {
                    input = Some(false as i64);
                }
            }
            Err(IntCodeErr::End) => break,
            Err(IntCodeErr::OutOfBounds(fault)) => {
                if fault + 1 > MAX_MEM {
                    panic!("The VM demanded too much memory! {fault}");
                }
                program.resize_with(fault + 1, i64::default);
            }
            Err(e) => panic!("{e}"),
        }
    }
    // part2
    if start == 1 {
        let mut white_panels = panels
            .into_iter()
            .filter(|(_, v)| *v)
            .map(|(k, _)| k)
            .collect::<Vec<_>>();
        white_panels.sort_unstable_by_key(|k| k.1 /* we only need to sort by y axis */);
        let mut last_y = miny;
        let mut outline = vec!['.'; (maxx.abs() + minx.abs()) as usize];
        for (x, y) in white_panels {
            if y != last_y {
                last_y = y;
                println!("{}", outline.drain(..).collect::<String>());
                outline.resize_with((maxx.abs() + minx.abs()) as usize, || '.');
            }
            outline[(x + minx.abs()) as usize] = '#';
        }
        println!("{}", outline.drain(..).collect::<String>());
    }
    uniq_painted
}

fn main() -> io::Result<()> {
    let program = read_intcode()?;
    let part1 = run_program(program.clone(), 0);
    println!("Part1: {part1}");
    println!("Part2:");
    let _part2 = run_program(program, 1);
    Ok(())
}
