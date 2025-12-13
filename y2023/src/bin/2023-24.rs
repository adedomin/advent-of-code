use aoc_shared::{read_input, try_atoi, Token, Tokenize};
use itertools::Itertools;
use std::io;
use z3::{ast::Int, Solver};

#[derive(Debug)]
struct Line {
    px: i64,
    py: i64,
    pz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

type XY = (f64, f64);

const P1_SEG_START: f64 = 200000000000000f64;
// const P1_SEG_START: f64 = 7f64;
const P1_SEG_END: f64 = 400000000000000f64;
// const P1_SEG_END: f64 = 21f64;

impl Line {
    fn p12d_segment(&self) -> (XY, XY) {
        let xs = self.px as f64;
        let xe = (self.px + self.vx * 1000) as f64;
        let ys = self.py as f64;
        let ye = (self.py + self.vy * 1000) as f64;

        ((xs, ys), (xe, ye))
    }

    pub fn p12d_insersects(&self, other: &Self) -> Option<(f64, f64)> {
        let ((x1, y1), (x2, y2)) = self.p12d_segment();
        let ((x3, y3), (x4, y4)) = other.p12d_segment();

        let s21x = x2 - x1;
        let s21y = y2 - y1;
        let s43x = x4 - x3;
        let s43y = y4 - y3;

        let denom = s21x * s43y - s43x * s21y;
        // intersects.
        if denom == 0f64 {
            return None;
        }

        let s13x = x1 - x3;
        let s13y = y1 - y3;

        let sn = s21x * s13y - s21y * s13x;
        if sn < 0f64 && denom > 0f64 {
            return None;
        }
        let tn = s43x * s13y - s43y * s13x;
        if tn < 0f64 && denom > 0f64 {
            return None;
        }

        if (sn > denom) != (denom > 0f64) || (tn > denom) != (denom > 0f64) {
            return None;
        }
        let t = tn / denom;
        Some((x1 + (t * s21x), y1 + (t * s21y)))
    }
}

type Output = Vec<Line>;

enum Neg {
    Num(i64),
    IsNeg,
}

fn parse_input(input: &[u8]) -> Output {
    input
        .tokenize()
        .flat_map(|t| {
            if let Token::Something(x) = t {
                try_atoi::<i64, 10>(x).map(Neg::Num)
            } else if let Token::Delimiter(b'-') = t {
                Some(Neg::IsNeg)
            } else {
                None
            }
        })
        .fold((Vec::new(), 1), |(mut acc, is_neg), tok| match tok {
            Neg::Num(x) => {
                acc.push(x * is_neg);
                (acc, 1)
            }
            Neg::IsNeg => (acc, -1),
        })
        .0
        .chunks(6)
        .map(|c| Line {
            px: c[0],
            py: c[1],
            pz: c[2],
            vx: c[3],
            vy: c[4],
            vz: c[5],
        })
        .collect::<Vec<_>>()
}

fn solve2d(i: &[Line]) -> usize {
    i.iter()
        .tuple_combinations()
        .flat_map(|(l1, l2)| l1.p12d_insersects(l2))
        .filter(|(x, y)| {
            (P1_SEG_START..=P1_SEG_END).contains(x) && (P1_SEG_START..=P1_SEG_END).contains(y)
        })
        .count()
}

fn solve3d(i: Vec<Line>) -> Option<u64> {
    let solver = Solver::new();
    let x = Int::fresh_const("x");
    let y = Int::fresh_const("y");
    let z = Int::fresh_const("z");
    let dx = Int::fresh_const("dx");
    let dy = Int::fresh_const("dy");
    let dz = Int::fresh_const("dz");
    // time should be greater than zero and constrains the finder appropriately, otherwise
    // it will loop for some time.
    let zero = Int::from(0);
    i.into_iter().enumerate().for_each(
        |(
            pos,
            Line {
                px,
                py,
                pz,
                vx,
                vy,
                vz,
            },
        )| {
            let ti = Int::fresh_const(&format!("t{pos}"));
            let vx_dx = vx + &dx;
            let vy_dy = vy + &dy;
            let vz_dz = vz + &dz;
            solver.assert(ti.ge(&zero));
            solver.assert((px + &ti * vx_dx).eq(&x));
            solver.assert((py + &ti * vy_dy).eq(&y));
            solver.assert((pz + &ti * vz_dz).eq(&z));
        },
    );
    match solver.check() {
        z3::SatResult::Unsat => None,
        z3::SatResult::Unknown => None,
        z3::SatResult::Sat => {
            let m = solver.get_model()?;
            Some(
                m.get_const_interp(&x)
                    .expect("x should be solved for.")
                    .as_u64()
                    .unwrap()
                    + m.get_const_interp(&y)
                        .expect("y should be solved for.")
                        .as_u64()
                        .unwrap()
                    + m.get_const_interp(&z)
                        .expect("z should be solved for.")
                        .as_u64()
                        .unwrap(),
            )
        }
    }
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let part1 = solve2d(&parsed_input);
    print!("Part1: {part1}, ");
    let part2 = solve3d(parsed_input).expect("expected an intercept point.");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
