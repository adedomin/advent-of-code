use aoc_shared::read_input_to_string;
use std::io::{self};

type Int = i64;
type Output = Vec<(Input, Vec<Instr>)>;

#[derive(Debug)]
enum Imm {
    #[allow(dead_code)]
    Reg(usize),
    Imm(Int),
}

#[derive(Debug)]
struct Instr {
    #[allow(dead_code)]
    op: fn(Int, Int) -> Int,
    #[allow(dead_code)]
    lhs: usize,
    rhs: Imm,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Input(usize);

const W: usize = 0;
const X: usize = 1;
const Y: usize = 2;
const Z: usize = 3;

fn s_to_reg(s: &str) -> usize {
    match s {
        "x" => X,
        "y" => Y,
        "z" => Z,
        _ => W,
    }
}

fn add(a: Int, b: Int) -> Int {
    a + b
}

fn mul(a: Int, b: Int) -> Int {
    a * b
}

fn div(a: Int, b: Int) -> Int {
    a / b
}

fn modu(a: Int, b: Int) -> Int {
    a % b
}

fn eql(a: Int, b: Int) -> Int {
    (a == b) as Int
}

fn parse_input(input: &str) -> Output {
    let mut ret = vec![];
    let mut itr = input.split_ascii_whitespace();
    while let Some(word) = itr.next() {
        let lhs = s_to_reg(itr.next().expect("need register"));
        let op = match word {
            "inp" => {
                ret.push((Input(lhs), vec![]));
                continue;
            }
            "add" => add,
            "mul" => mul,
            "div" => div,
            "mod" => modu,
            "eql" => eql,
            _ => panic!("invalid opcode"),
        };

        let (_, inst) = ret
            .last_mut()
            .expect("Input should be the first instruction");
        let reg_imm = itr.next().expect("expected another value for instruction");
        let rhs = if let Ok(imm) = reg_imm.parse::<Int>() {
            Imm::Imm(imm)
        } else {
            Imm::Reg(s_to_reg(reg_imm))
        };
        inst.push(Instr { op, lhs, rhs });
    }
    ret
}

// These are the only things that seem to change between input runs...
// Compare the segments delimited by `inp w` side by side.
// is this specific to my input? not happy about this.
const SECRET_OPS: [usize; 3] = [3, 4, 14];
fn extract_secret_vals(i: &[Instr]) -> (Int, Int, Int) {
    match (
        &i[SECRET_OPS[0]].rhs,
        &i[SECRET_OPS[1]].rhs,
        &i[SECRET_OPS[2]].rhs,
    ) {
        (Imm::Imm(dz), Imm::Imm(ax), Imm::Imm(ay)) => (*dz, *ax, *ay),
        _ => panic!("input is not as expected?"),
    }
}

/// Since it's only div/mod by 26 or 1 for z...
/// When div z 1, add x is always by a factor greater than 9...
/// Thus:
///
/// ```rust
///   if dz == 1 {
///     z *= 26;
///     z += inp[i] + ay;
///   } else /* dz is 26 every other case */ {
///     if z % 26 + ax == inp[i] {
///       z /= 26;
///     } else {
///       z /= 26;
///       z += inp[i] + ay;
///     }
///   }
/// ```
///
fn part1_sol(mut ws: [Int; 14], input: &Output) -> [Int; 14] {
    let mut stack = vec![];
    (0..14).for_each(|i| {
        let (dz, ax, ay) = extract_secret_vals(&input[i].1);
        if dz == 1 {
            // we can look at the div/mul by 26 as push / pop on a stack.
            // the add instructions AFTER are basically storing value(s) inside this area.
            // we store the w (since we need to correct it, the idx)
            stack.push((i, ay));
        } else
        /* see above CAN ONLY BE 26 OTHERWISE */
        {
            // we divided by 26, thus we are exposing these juicy values we stored before.
            let (j, oy) = stack.pop().unwrap();
            // calculate the *corrected* input based on ALU algo.
            let w = ws[j] + oy + ax;
            // we over or underflow into inp[j]
            (ws[i], ws[j]) = if w > 9 {
                (9, ws[j] - (w - 9))
            } else if w < 1 {
                (1, ws[j] + (1 - w))
            } else {
                (w, ws[j])
            };
        }
    });
    assert_eq!(
        stack.len(),
        0,
        "If this isn't zero, something is wrong with our assumption."
    );
    ws
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let p1ws: [Int; 14] = vec![Int::from(9); parsed_input.len() /* should be 14 */]
        .try_into()
        .expect("instruction length should be 14 inputs.");
    let part1 = part1_sol(p1ws, &parsed_input);
    print!("Part1: ");
    part1.into_iter().for_each(|w| print!("{w}"));
    let p2ws: [Int; 14] = vec![Int::from(1); parsed_input.len() /* should be 14 */]
        .try_into()
        .expect("instruction length should be 14 inputs.");
    let part2 = part1_sol(p2ws, &parsed_input);
    print!(", Part2: ");
    part2.into_iter().for_each(|w| print!("{w}"));
    println!();
    Ok(())
}
