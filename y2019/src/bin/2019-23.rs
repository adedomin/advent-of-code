use std::{array, collections::VecDeque, io};

use y2019::intcode::{brk, read_intcode, IntCode, IntCodeErr};

#[derive(Copy, Clone)]
enum Packet {
    Init(i64),
    X(i64, i64),
    XY(i64, i64, i64),
}

impl Packet {
    fn next(self, out: i64) -> Self {
        match self {
            Packet::Init(n) => Self::X(n, out),
            Packet::X(n, x) => Self::XY(n, x, out),
            Packet::XY(_, _, _) => Self::Init(out),
        }
    }
}

struct Nic {
    i: IntCode,
    p: Vec<i64>,
}

fn build_nics(prog: Vec<i64>) -> [Nic; 50] {
    let ret = array::from_fn(|i| {
        let mut i = Some(i as i64);
        let mut n = Nic {
            i: IntCode::default(),
            p: prog.clone(),
        };
        // run machine til it takes its unique ID
        while i.is_some() {
            match n.i.execute(&mut n.p, &mut i) {
                Ok(None) => (),
                Err(IntCodeErr::OutOfBounds(oob)) => brk(oob, &mut n.p).unwrap(),
                e => panic!("unexpected output: {e:?}"),
            }
        }
        n
    });
    ret
}

fn solve(mut nics: [Nic; 50]) -> (i64, i64) {
    let mut p1 = None;
    let mut p2_cnt = 0;
    let mut nat = None;
    // cycle every machine at start.
    // this works because any new inputs come after these -1s are cleared,
    // giving each machine a non-blocking chance to make outputs
    // side-steps ordering concerns.
    let s: [(_, _); 50] = array::from_fn(|i| (i, Some(-1)));
    let mut q = VecDeque::from(s);
    loop {
        while let Some((i, mut inp)) = q.pop_front() {
            let Nic {
                i: intcode,
                p: prog,
            } = &mut nics[i];
            let mut packet = Packet::XY(-1, -1, -1);
            loop {
                match intcode.execute_til(prog, &mut inp) {
                    Ok(out) => {
                        packet = packet.next(out);
                        if let Packet::XY(i, x, y) = packet {
                            if i == 255 {
                                if p1.is_none() {
                                    p1 = Some(y);
                                }

                                if let Some((nx, ny)) = nat {
                                    if nx == x && ny == y {
                                        p2_cnt += 1;
                                        // problem asks for twice, so we have to reach this cond at least 3 times.
                                        if p2_cnt == 3 {
                                            return (p1.unwrap(), y);
                                        }
                                    } else {
                                        p2_cnt = 0;
                                    }
                                }
                                nat = Some((x, y));
                            } else {
                                q.extend([(i as usize, Some(x)), (i as usize, Some(y))]);
                            }
                        }
                    }
                    // this is where it gets tricky.
                    // problem asks you to feed the machine with -1, but -1 input does not appear to do anything.
                    // we can just exhaust packet queue instead...
                    Err(IntCodeErr::NeedInput) => break, // defer to loop
                    Err(IntCodeErr::OutOfBounds(oob)) => brk(oob, prog).unwrap(),
                    Err(e) => panic!("unexpected err: {e}"),
                }
            }
        }
        // out of packets, inject NAT packet.
        if let Some((x, y)) = nat {
            q.extend([(0, Some(x)), (0, Some(y))]);
        } else {
            panic!("Ran out of packets without a NAT packet to restart, failing!");
        }
    }
}

fn main() -> io::Result<()> {
    let program = read_intcode()?;
    let nics = build_nics(program);
    let (part1, part2) = solve(nics);
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
