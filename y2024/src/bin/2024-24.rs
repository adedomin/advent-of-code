use aoc_shared::{read_input_to_string, try_atoi};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::io;

type Int = u64;
type Output<'a> = (
    Vec<(&'a str, bool)>,
    Vec<(Op, [Option<bool>; 2], [&'a str; 2], &'a str)>,
    FxHashMap<&'a str, Vec<(usize, usize)>>,
);

#[derive(Clone, Copy, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl TryFrom<&str> for Op {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(()),
        }
    }
}

impl Op {
    fn exec(&self, v: [Option<bool>; 2]) -> Option<bool> {
        match (self, v) {
            (Op::And, [None, _]) => None,
            (Op::And, [_, None]) => None,
            (Op::And, [Some(a), Some(b)]) => Some(a && b),
            (Op::Or, [None, _]) => None,
            (Op::Or, [_, None]) => None,
            (Op::Or, [Some(a), Some(b)]) => Some(a || b),
            (Op::Xor, [None, _]) => None,
            (Op::Xor, [_, None]) => None,
            (Op::Xor, [Some(a), Some(b)]) => Some(a ^ b),
        }
    }
}

fn parse_input<'a>(input: &'a str) -> Output<'a> {
    let (start, instructions) = input
        .split_once("\n\n")
        .expect("input needs to be delimited.");
    let start = start
        .split([':', ' ', '\n'])
        .filter(|v| !v.is_empty())
        .tuples()
        .map(|(ident, val)| (ident, val == "1"))
        .collect::<Vec<(_, bool)>>();
    let mut wiremap: FxHashMap<&'a str, Vec<(usize, usize)>> = FxHashMap::default();
    let wires = instructions
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(i, line)| {
            let mut items = line.split_ascii_whitespace();
            let from = items.next().expect("Need a left hand to operation");
            wiremap.entry(from).or_default().push((i, 0));

            let op: Op = items
                .next()
                .expect("Need op")
                .try_into()
                .expect("Invalid Operation.");

            let to = items.next().expect("need right hand op");
            wiremap.entry(to).or_default().push((i, 1));

            items.next().unwrap();
            let out = items.next().expect("need out");

            (op, [None, None], [from, to], out)
        })
        .collect::<Vec<_>>();
    (start, wires, wiremap)
}

fn convert_to(ident: &str, b: bool) -> Int {
    let shl = try_atoi::<u32, 10>(&ident.as_bytes()[1..]).expect("not a number!");
    (b as Int)
        .checked_shl(shl)
        .expect("shl value too large: {shl}")
}

fn part1_sol<'a>(
    start: &[(&'a str, bool)],
    wiremap: &FxHashMap<&'a str, Vec<(usize, usize)>>,
    mut wires: Vec<(Op, [Option<bool>; 2], [&'a str; 2], &'a str)>,
) -> Int {
    let mut ret: Int = 0;
    start.iter().for_each(|&(ident, v)| {
        let mut propagate = vec![(ident, v)];
        while let Some((ident, v)) = propagate.pop() {
            if let Some(idc) = wiremap.get(ident) {
                propagate.extend(idc.iter().filter_map(|&(i, pos)| {
                    wires[i].1[pos] = Some(v);
                    wires[i].0.exec(wires[i].1).map(|b| (wires[i].3, b))
                }))
            } else if ident.starts_with("z") {
                ret |= convert_to(ident, v);
            } else {
                panic!("unknown ident! {ident}");
            }
        }
    });
    ret
}

fn gen_order_pair<'a>(a: &'a str, b: &'a str) -> (&'a str, &'a str) {
    match a.cmp(b) {
        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => (a, b),
        std::cmp::Ordering::Greater => (b, a),
    }
}

// 32361484689977 + 30808958377647 =
// 1110010111010000000011110100011100000011101000
// incorrect =
// 1110010111001110000100000100100100000011101000

// XOR add, AND carry.
// first error at 15?
// z14 carry:
// wkm OR pwq -> tpr
// z15:
// x15: 1, y15: 0
// x15 XOR y15 -> -kfm- *dwp*
// kfm OR knt -> phm
//
// x15 AND y15 -> -dwp- *kfm* -> is carry.
//
// dwp AND tpr -> knt <- what?
//
// tpr XOR -dwp- *kfm* -> z15
//
//
// knt
// SWAP kfm - dwp
//
// Next bad: 23?
// x23: 1, y23: 0
// x23 AND y23 -> mkf -> 0
// x23 XOR y23 -> qtk -> 1
//   qtk AND spp -> vsq <-- carry + xor
//   vsq OR mkf -> hrv
//   spp XOR qtk -> z23 <- carry into
// bad 22?
// x22: 1, y22: 0
// y22 AND x22 -> -z22- <-- bad.
// y22 XOR x22 -> hgq
//   pgt XOR hgq -> -gjh- <-- very likely bad.

// SWAP z22, gjh
//
// next 32?
// x32: 0, y32: 1
// x32 XOR y32 -> wvk
//   jdr XOR wvk -> z32
//   wvk AND jdr -> bhw
// x32 AND y32 -> dwn
//   bhw OR dwn -> rbj
// ...looks ok, check 31.
//
// x31: 1, y32: 0
// y31 XOR x31 -> rns
//   rns XOR hnn -> -jdr-
//   rns AND hnn -> vhw
// x31 AND y31 -> ctt
//   ctt OR vhw -> -z31-
//
// SWAP jdr z31
//
// WE NOW HAVE MATCHING SUMS, but what is the last pair??!?!
// rbs OR rvg -> z45 <- overflow to 45th.
fn part2_sol(
    part1: Int,
    start: &[(&str, bool)],
    wiremap: &FxHashMap<&str, Vec<(usize, usize)>>,
    wires: Vec<(Op, [Option<bool>; 2], [&str; 2], &str)>,
) {
    let mut x: Int = 0;
    let mut y: Int = 0;
    start.iter().for_each(|&(label, value)| {
        if label.starts_with("x") {
            x |= convert_to(label, value);
        } else if label.starts_with("y") {
            y |= convert_to(label, value);
        } else {
            panic!("Invalid input label: {label}");
        }
    });
    println!("solved? {}", x + y == part1);
    println!("{x} + {y} =");
    println!("{:0b}", x + y);
    println!("incorrect (?) = ");
    println!("{:0b}", part1);
    println!("incorrect (start) = ");
    println!("{:0b}", 63168299811048u64);

    for i in 0..45 {
        let x = format!("x{i:02}");
        // let z = format!("z{i:02}");
        // let y = format!("y{i:02}");
        let ws = wiremap.get(x.as_str()).unwrap();
        if ws.len() != 2 {
            println!("bad at {x}, {}", ws.len());
        }
        ws.iter().for_each(|&(w, _)| match wires[w].0 {
            Op::And if i != 0 => {
                println!("{x} AND y{i:02} -> {}", wires[w].3);
                let wx = wiremap.get(wires[w].3).unwrap();
                if wx.len() != 1 {
                    println!("maybe bad at {x} - carry -> {}, {}", wires[w].3, ws.len());
                } else if !matches!(wires[wx[0].0].0, Op::Or) {
                    println!(
                        "maybe bad at {} - carry/carry -> {},",
                        wires[w].3, wires[wx[0].0].3,
                    );
                } else {
                    let (a, b) = gen_order_pair(wires[wx[0].0].2[0], wires[wx[0].0].2[1]);
                    println!(
                        "  {} {:?} {} -> {}",
                        a, wires[wx[0].0].0, b, wires[wx[0].0].3
                    );
                }
            }
            Op::Or => println!("bad line at {x}"),
            Op::Xor if i != 0 => {
                println!("{x} XOR y{i:02} -> {}", wires[w].3);
                let wx = wiremap.get(wires[w].3).unwrap();
                if wx.len() != 2 {
                    println!("maybe bad at {x} - XOR -> {}, {}", wires[w].3, ws.len());
                }
                wx.iter().for_each(|&(i, _)| {
                    let (a, b) = gen_order_pair(wires[i].2[0], wires[i].2[1]);
                    println!("  {} {:?} {} -> {}", a, wires[i].0, b, wires[i].3);
                });
            }
            _ => (),
        });
        println!("------------------");
    }
    // find all non full 1bit adders...
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let (start, wires, wiremap) = parse_input(&input);
    let part1 = part1_sol(&start, &wiremap, wires.clone());
    part2_sol(part1, &start, &wiremap, wires);
    println!("Part1: {part1}");
    println!("Part2: Read comments above part2_sol");
    println!("       dwp,ffj,gjh,jdr,kfm,z08,z22,z31");
    Ok(())
}
