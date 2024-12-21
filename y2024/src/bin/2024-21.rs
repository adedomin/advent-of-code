use aoc_shared::{atoi, read_input_to_string};
use core::panic;
use rustc_hash::FxHashMap;
use std::{io, mem};

type Int = usize;
type Output<'a> = Vec<&'a [u8]>;

fn numpad_pos(c: u8) -> (i8, i8) {
    match c {
        b'1' => (0, 2),
        b'2' => (1, 2),
        b'3' => (2, 2),
        b'4' => (0, 1),
        b'5' => (1, 1),
        b'6' => (2, 1),
        b'7' => (0, 0),
        b'8' => (1, 0),
        b'9' => (2, 0),
        b'0' => (1, 3),
        b'A' => (2, 3),
        _ => panic!("Invalid numpad button {c}."),
    }
}

fn dirpad_pos(c: u8) -> (i8, i8) {
    match c {
        b'^' => (1, 0),
        b'<' => (0, 1),
        b'v' => (1, 1),
        b'>' => (2, 1),
        b'A' => (2, 0),
        _ => panic!("Invalid directional pad button."),
    }
}

// fn get_slope((fx, fy): (i8, i8), (tx, ty): (i8, i8)) -> (i8, i8) {
//     (tx - fx, ty - fy)
// }

// fn inverse_type(s: &str) {
//     let mut last = (2, 0);
//     s.chars().for_each(|c| match c {
//         '^' => last = (last.0, last.1 - 1),
//         '<' => last = (last.0 - 1, last.1),
//         'v' => last = (last.0, last.1 + 1),
//         '>' => last = (last.0 + 1, last.1),
//         'A' => match last {
//             (1, 0) => print!("^"),
//             (2, 0) => print!("A"),
//             (0, 1) => print!("<"),
//             (1, 1) => print!("v"),
//             (2, 1) => print!(">"),
//             _ => panic!("invalid pos."),
//         },
//         _ => panic!("invalid move."),
//     });
//     println!();
// }

// fn type_num(s: &str) {
//     let mut last = numpad_pos('A');
//     s.chars().for_each(|c| match c {
//         '^' => last = (last.0, last.1 - 1),
//         '<' => last = (last.0 - 1, last.1),
//         'v' => last = (last.0, last.1 + 1),
//         '>' => last = (last.0 + 1, last.1),
//         'A' => match last {
//             (0, 2) => println!("1"),
//             (1, 2) => println!("2"),
//             (2, 2) => println!("3"),
//             (0, 1) => println!("4"),
//             (1, 1) => println!("5"),
//             (2, 1) => println!("6"),
//             (0, 0) => println!("7"),
//             (1, 0) => println!("8"),
//             (2, 0) => println!("9"),
//             (1, 3) => println!("0"),
//             (2, 3) => println!("A"),
//             _ => panic!("Invalid numpad button {c}."),
//         },
//         _ => panic!("invalid move."),
//     });
//     println!();
// }

// fn expand(mut s: String) -> String {
//     let mut expand = String::new();
//     let mut last = dirpad_pos('A');
//     s.drain(..).for_each(|dir| {
//         let (x, y) = get_slope(last, dirpad_pos(dir));
//         let vert = match y.cmp(&0) {
//             std::cmp::Ordering::Less => "^".repeat(-y as usize),
//             std::cmp::Ordering::Equal => "".to_owned(),
//             std::cmp::Ordering::Greater => "v".repeat(y as usize),
//         };
//         let hori = match x.cmp(&0) {
//             std::cmp::Ordering::Less => "<".repeat(-x as usize),
//             std::cmp::Ordering::Equal => "".to_owned(),
//             std::cmp::Ordering::Greater => ">".repeat(x as usize),
//         };
//         expand.push_str(&hori);
//         expand.push_str(&vert);
//         expand.push('A');
//         last = dirpad_pos(dir);
//     });
//     expand
// }

fn parse_input(input: &str) -> Output<'_> {
    input
        .split('\n')
        .filter(|num| !num.is_empty())
        .map(|num| num.as_bytes())
        .collect::<Output>()
}

const DIRPAD_DEADZONE: (i8, i8) = (0, 0);
const NUMPAD_DEADZONE: (i8, i8) = (0, 3);
type ToFrom = (i8, i8);
type Key = (ToFrom, ToFrom, u8);

#[derive(Clone, Copy)]
enum ExpandFrom {
    Num,
    Dir,
}

impl ExpandFrom {
    fn map_coord(&self, v: u8) -> (i8, i8) {
        match self {
            Self::Num => numpad_pos(v),
            Self::Dir => dirpad_pos(v),
        }
    }
}

fn expand_all(memo: &mut FxHashMap<Key, Int>, expn: ExpandFrom, s: &[u8], depth: u8) -> Int {
    let mut start = expn.map_coord(b'A');
    s.iter()
        .map(|&v| expn.map_coord(v))
        .map(|to| expand_pad_pattern(memo, expn, mem::replace(&mut start, to), to, depth))
        .sum()
}

fn clone_push(p: &[u8], n: u8) -> Vec<u8> {
    let mut ret = p.to_vec();
    ret.push(n);
    ret
}

fn expand_pad_pattern(
    memo: &mut FxHashMap<Key, Int>,
    expn: ExpandFrom,
    from: ToFrom,
    to: ToFrom,
    depth: u8,
) -> Int {
    if let Some(&m) = memo.get(&(from, to, depth)) {
        return m;
    }

    // each subpattern effectively branches wildly into many different valid sizes.
    let mut min = usize::MAX;
    let mut stack = vec![(from, vec![])];
    while let Some((from, pattern)) = stack.pop() {
        match expn {
            ExpandFrom::Num if from == NUMPAD_DEADZONE => continue,
            ExpandFrom::Dir if from == DIRPAD_DEADZONE => continue,
            _ => (),
        }

        if from == to && depth == 0 {
            // + 1 for A
            min = min.min(pattern.len() + 1);
        } else if from == to {
            let len = expand_all(
                memo,
                ExpandFrom::Dir,
                &clone_push(&pattern, b'A'),
                depth - 1,
            );
            min = min.min(len);
        } else {
            // since patterns like >^> can be more optimal... we have to try them all.
            match from.0.cmp(&to.0) {
                std::cmp::Ordering::Less => {
                    stack.push(((from.0 + 1, from.1), clone_push(&pattern, b'>')))
                }
                std::cmp::Ordering::Greater => {
                    stack.push(((from.0 - 1, from.1), clone_push(&pattern, b'<')))
                }
                _ => (),
            }
            match from.1.cmp(&to.1) {
                std::cmp::Ordering::Less => {
                    stack.push(((from.0, from.1 + 1), clone_push(&pattern, b'v')))
                }
                std::cmp::Ordering::Greater => {
                    stack.push(((from.0, from.1 - 1), clone_push(&pattern, b'^')))
                }
                _ => (),
            }
        }
    }

    *memo.entry((from, to, depth)).insert_entry(min).get()
}

fn solve(input: &Output) -> (Int, Int) {
    let mut p1memo = FxHashMap::default();
    let mut p2memo = FxHashMap::default();
    input.iter().fold((0, 0), |(p1, p2), numpad| {
        let numeric = atoi::<Int, 10>(numpad);
        let p1len = expand_all(&mut p1memo, ExpandFrom::Num, numpad, 2);
        let p2len = expand_all(&mut p2memo, ExpandFrom::Num, numpad, 25);
        (p1 + numeric * p1len, p2 + numeric * p2len)
    })
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let (part1, part2) = solve(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
