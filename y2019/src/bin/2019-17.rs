use std::{
    collections::{HashMap, VecDeque},
    fmt::{Display, Write as WriteF},
    io::{self, Write},
};

use aoc_shared::{destructure_or_none, FlatVec2D, Neighbor};
use itertools::Itertools;
use y2019::intcode::{brk, read_intcode, IntCode, IntCodeErr};

#[derive(Default, Copy, Clone)]
enum Scaff {
    Beam,  // #
    Space, // .
    Bot((isize, isize)),
    BotDead,
    #[default]
    Border,
}

const fn rot_right((x, y): (isize, isize)) -> (isize, isize) {
    (-y, x)
}
const fn rot_left((x, y): (isize, isize)) -> (isize, isize) {
    rot_right(rot_right(rot_right((x, y))))
}

const N: (isize, isize) = (0, -1);
const E: (isize, isize) = rot_right(N);
const S: (isize, isize) = rot_right(E);
const W: (isize, isize) = rot_right(S);

impl From<u8> for Scaff {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Scaff::Beam,
            b'.' => Scaff::Space,
            b'^' => Scaff::Bot(N),
            b'>' => Scaff::Bot(E),
            b'v' => Scaff::Bot(S),
            b'<' => Scaff::Bot(W),
            b'X' => Scaff::BotDead,
            _ => Scaff::Border,
        }
    }
}

impl std::fmt::Debug for Scaff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Beam => f.write_char('#'),
            Self::Space => f.write_char('.'),
            Self::Bot(N) => f.write_char('^'),
            Self::Bot(E) => f.write_char('>'),
            Self::Bot(S) => f.write_char('v'),
            Self::Bot(W) => f.write_char('<'),
            Self::Bot(_) => f.write_char('!'),
            Self::BotDead => f.write_char('X'),
            Self::Border => f.write_char('0'),
        }
    }
}

#[derive(Default, Debug)]
struct Pos {
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
}

fn get_scaffolding(mut prog: Vec<i64>) -> (FlatVec2D<Scaff>, Pos) {
    let mut map = vec![];
    let mut intcode = IntCode::default();
    loop {
        match intcode.execute_til(&mut prog, &mut None) {
            Ok(o) => map.push(u8::try_from(o).expect("Valid ASCII")),
            Err(IntCodeErr::OutOfBounds(newbrk)) => {
                brk(newbrk, &mut prog).expect("to increase program brk")
            }
            Err(IntCodeErr::End) => break,
            Err(e) => panic!("Unexpected ASCII failure: {e}"),
        }
    }
    let row_width = map.iter().position(|&chr| chr == b'\n').unwrap() + 2;
    // TODO: Have to fix pad_to_flat2d for odd numbers
    let col_len = (map.len() - 1).div_ceil(row_width + 1) + 4;
    let mut ret = FlatVec2D::new(row_width, col_len);

    let mut i = 1;
    let mut j = 1;
    let mut start = Pos::default();
    map.iter().for_each(|&el| {
        if el == b'\n' {
            i = 1;
            j += 1;
        } else if el != b'\n' {
            let s = el.into();
            if let Scaff::Bot(b) = s {
                start = Pos {
                    x: i,
                    y: j,
                    dx: b.0,
                    dy: b.1,
                };
            }
            ret[(i, j)] = s;
            i += 1;
        }
    });
    (ret, start)
}

type Intersect = HashMap<(usize, usize), [bool; 2]>;

fn part1(map: &FlatVec2D<Scaff>) -> (usize, Intersect) {
    let mut intersect = HashMap::default();
    let p1 = map
        .pad_xyrange()
        .flat_map(|(x, y)| {
            if let Scaff::Beam = map[(x, y)] {
                map.get_neigh_cardinal(x, y)
                    .iter()
                    .all(|Neighbor(s, _, _)| matches!(s, Scaff::Beam))
                    .then_some((x - 1, y - 1))
                    .inspect(|(x, y)| {
                        // for part2.
                        intersect.insert((x + 1, y + 1), [false; 2]);
                    })
            } else {
                None
            }
        })
        .map(|(x, y)| x * y)
        .sum();
    (p1, intersect)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Moves {
    Left,
    Right,
    Forward(u8),
}

fn get_str_size(s: &[Moves]) -> usize {
    let mut sep = s.len(); // commas are (len - 1) + 1 for newline
    for m in s {
        sep += match m {
            Moves::Forward(l) => {
                let is_ten = l / 10;
                if is_ten == 0 {
                    1
                } else if (1..=9).contains(&is_ten) {
                    2
                } else {
                    3
                }
            }
            _ => 1,
        }
    }
    sep
}

impl Display for Moves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Moves::Left => f.write_str("L"),
            Moves::Right => f.write_str("R"),
            Moves::Forward(v) => f.write_fmt(format_args!("{v}")),
        }
    }
}

#[derive(Clone, Debug)]
enum Prog {
    A,
    B,
    C,
    Place(Vec<Moves>),
}

impl Display for Prog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => f.write_str("A"),
            Self::B => f.write_str("B"),
            Self::C => f.write_str("C"),
            _ => panic!("Invalid state."),
        }
    }
}

fn crd(x: usize, dx: isize) -> usize {
    usize::try_from(x as isize + dx).expect("not to go out of bounds")
}

fn get_inter_door((dx, dy): (isize, isize)) -> usize {
    match (dx, dy) {
        N | S => 0,
        E | W => 1,
        _ => 999,
    }
}

fn replace_move_with_prog(m: &[Moves], from: &[Moves], with: Prog) -> Vec<Prog> {
    let mut store = vec![];
    let mut ret = vec![];
    for el in m.iter().cloned() {
        store.push(el);
        if store.ends_with(from) {
            let (l, _) = store.split_at(store.len() - from.len());
            if !l.is_empty() {
                ret.push(Prog::Place(l.to_vec()));
            }
            ret.push(with.clone());
            store.clear();
        }
    }
    if !store.is_empty() {
        ret.push(Prog::Place(store));
    }
    ret
}

fn flatten_placeholders(p: &[Prog], from: &[Moves], with: Prog) -> Vec<Prog> {
    p.iter()
        .cloned()
        .flat_map(|p| match p {
            Prog::A | Prog::B | Prog::C => vec![p],
            Prog::Place(items) => replace_move_with_prog(&items, from, with.clone()),
        })
        .collect_vec()
}

fn all_sub(s: &[Moves]) -> impl Iterator<Item = &[Moves]> {
    let mut i = 0;
    let mut j = 1;
    std::iter::from_fn(move || {
        if i == s.len() {
            return None;
        }
        let ret = Some(&s[i..j]);
        if j == s.len() {
            i += 1;
            j = i + 1;
        } else {
            j += 1;
        }
        ret
    })
}

fn get_program(p: &[Moves]) -> Option<String> {
    for a_pattern in (2..p.len())
        .rev()
        .map(|i| &p[..i])
        .filter(|p| get_str_size(p) < 21)
    {
        let a_prog = replace_move_with_prog(p, a_pattern, Prog::A);
        for b_pattern in a_prog
            .iter()
            .flat_map(|bsub| destructure_or_none!(Prog::Place|p| = bsub))
            .flat_map(|sub| all_sub(sub).filter(|s| get_str_size(s) < 21))
        {
            let b_prog = flatten_placeholders(&a_prog, b_pattern, Prog::B);
            for c_pattern in b_prog
                .iter()
                .flat_map(|csub| destructure_or_none!(Prog::Place|p| = csub))
                .flat_map(|sub| all_sub(sub).filter(|s| get_str_size(s) < 21))
            {
                let c_prog = flatten_placeholders(&b_prog, c_pattern, Prog::C);
                if c_prog
                    .iter()
                    .all(|m| matches!(m, Prog::A | Prog::B | Prog::C))
                {
                    let c_p = c_prog.into_iter().map(|m| m.to_string()).join(",");
                    if c_p.len() > 19 {
                        continue;
                    }
                    return Some(format!(
                        "{}\n{}\n{}\n{}\nn\n",
                        c_p,
                        a_pattern.iter().map(|m| m.to_string()).join(","),
                        b_pattern.iter().map(|m| m.to_string()).join(","),
                        c_pattern.iter().map(|m| m.to_string()).join(","),
                    ));
                }
            }
        }
    }
    None
}

/// I'm so lost. finding all valid branching paths that terminate take 2^14 paths.
/// none of them seem valid though? not sure... this is a secondary attempt based on whiteboarding.
fn get_path_and_program(
    map: &FlatVec2D<Scaff>,
    mut inter: Intersect,
    Pos {
        mut x,
        mut y,
        mut dx,
        mut dy,
    }: Pos,
) -> String {
    let mut path = vec![];
    loop {
        // blow right through intersections, do not branch and traverse all permutations.
        while let Scaff::Beam = map[(crd(x, dx), crd(y, dy))] {
            // sanity check.
            if let Some(m) = inter.get_mut(&(x, y)) {
                let door = get_inter_door((dx, dy));
                assert!(!m[door], "We should only go through a door once.");
                m[door] = true;
            }
            x = crd(x, dx);
            y = crd(y, dy);
            path.push(Moves::Forward(1));
        }
        let (ldx, ldy) = rot_left((dx, dy));
        let (rdx, rdy) = rot_right((dx, dy));
        // since we blow right through intersections, we can only have a left or right or dead end.
        if let Scaff::Beam = map[(crd(x, ldx), crd(y, ldy))] {
            dx = ldx;
            dy = ldy;
            path.push(Moves::Left);
        } else if let Scaff::Beam = map[(crd(x, rdx), crd(y, rdy))] {
            dx = rdx;
            dy = rdy;
            path.push(Moves::Right);
        } else {
            break;
        }
    }
    // sanity check. we should have crossed through all intersections (both N&S / E&W doors "open").
    assert!(inter.into_values().all(|d| d.into_iter().all(|v| v)));
    let path = path
        .into_iter()
        .coalesce(|l, r| match (l, r) {
            (Moves::Forward(l), Moves::Forward(r)) => Ok(Moves::Forward(l + r)),
            _ => Err((l, r)),
        })
        .collect_vec();
    get_program(&path).expect("Valid intcode ascii program")
}

fn part2(mut prog: Vec<i64>, ascii: String) -> i64 {
    let mut ascii_inp = VecDeque::<u8>::new();
    write!(ascii_inp, "{ascii}").unwrap();
    let mut intcode = IntCode::default();
    let mut input = ascii_inp.pop_front().map(i64::from);
    let mut last = 0;
    prog[0] = 2;
    loop {
        match intcode.execute_til(&mut prog, &mut input) {
            Ok(o) => last = o,
            Err(IntCodeErr::NeedInput) if !ascii_inp.is_empty() => {
                input = ascii_inp.pop_front().map(i64::from);
            }
            Err(IntCodeErr::OutOfBounds(newbrk)) => {
                brk(newbrk, &mut prog).expect("to increase program brk")
            }
            Err(IntCodeErr::End) => break last,
            Err(e) => panic!("Unexpected ASCII failure: {e}"),
        }
    }
}

fn main() -> io::Result<()> {
    let program = read_intcode()?;
    let (map, pos) = get_scaffolding(program.clone());
    let (p1, intersections) = part1(&map);
    let ascii = get_path_and_program(&map, intersections, pos);
    let p2 = part2(program, ascii);
    println!("Part1: {p1}, Part2: {p2}");
    Ok(())
}
