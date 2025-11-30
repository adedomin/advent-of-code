use std::{
    collections::{HashMap, VecDeque},
    fmt::{Display, Write as WriteF},
    io::{self, Write},
};

use aoc_shared::{
    debug, destructure_or_none, pad_to_flat2d,
    rot::{rot_left, rot_right, EAST as E, NORTH as N, SOUTH as S, WEST as W},
    FlatVec2D, Neighbor,
};
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
    let ret = pad_to_flat2d(&map, Scaff::Border);
    debug!("{ret:?}");
    let mut start = None;
    for (x, y) in ret.pad_xyrange() {
        if let Scaff::Bot(b) = ret[(x, y)] {
            start = Some(Pos {
                x,
                y,
                dx: b.0,
                dy: b.1,
            });
            break;
        }
    }
    (ret, start.expect("No start found."))
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
                    // for part2.
                    .inspect(|(x, y)| {
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
            Moves::Forward(l) if *l != 0 => (l.ilog10() + 1) as usize,
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
        _ => panic!("Invalid cardinal (dx, dy): ({dx}, {dy})."),
    }
}

fn replace_move_with_prog(m: &[Moves], from: &[Moves], with: &Prog) -> Vec<Prog> {
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

fn flatten_placeholders(p: &[Prog], from: &[Moves], with: &Prog) -> Vec<Prog> {
    p.iter()
        .cloned()
        .flat_map(|p| match p {
            Prog::A | Prog::B | Prog::C => vec![p],
            Prog::Place(items) => replace_move_with_prog(&items, from, with),
        })
        .collect_vec()
}

/// Get all substrings in a given slice.
fn all_sub<'a, T: 'a, U: AsRef<[T]> + ?Sized>(s: &'a U) -> impl Iterator<Item = &'a [T]> {
    let mut i = 0;
    let s = s.as_ref();
    let start = if s.is_empty() { None } else { Some(s) };
    // iter::successors() iterators Impl FusedIterator unlike iter::from_fn
    std::iter::successors(start, move |ns| {
        if s.len() == i + 1 {
            return None;
        }
        let ns = &ns[..ns.len() - 1];
        if ns.is_empty() {
            i += 1;
            return Some(&s[i..]);
        }
        Some(ns)
    })
}

// incl newline and comma and base10 ascii encoded numbers.
const MAX_INSTR_SIZE: usize = 20;

fn get_patt_perm<'a>(
    prog: &'a [Prog],
    with: &'a Prog,
) -> impl Iterator<Item = (&'a [Moves], Vec<Prog>)> + use<'a> {
    prog.iter()
        .flat_map(|bsub| destructure_or_none!(Prog::Place|p| = bsub))
        .flat_map(|sub| all_sub(sub).filter(|s| get_str_size(s) < MAX_INSTR_SIZE + 1))
        .map(|patt| (patt, flatten_placeholders(prog, patt, with)))
}

fn get_program(p: &[Moves]) -> Option<VecDeque<u8>> {
    let base = [Prog::Place(p.to_vec())];
    for (a_pattern, a_prog) in get_patt_perm(&base, &Prog::A) {
        for (b_pattern, b_prog) in get_patt_perm(&a_prog, &Prog::B) {
            for (c_pattern, c_prog) in get_patt_perm(&b_prog, &Prog::C) {
                if c_prog
                    .iter()
                    .all(|m| matches!(m, Prog::A | Prog::B | Prog::C))
                {
                    let main_r = c_prog.into_iter().join(",");
                    if main_r.len() > MAX_INSTR_SIZE {
                        continue;
                    }
                    let a = a_pattern.iter().join(",");
                    let b = b_pattern.iter().join(",");
                    let c = c_pattern.iter().join(",");
                    debug!("{main_r}\n{a}\n{b}\n{c}\nn\n");
                    let mut vd = VecDeque::new();
                    write!(vd, "{main_r}\n{a}\n{b}\n{c}\nn\n").unwrap();
                    return Some(vd);
                }
            }
        }
    }
    None
}

/// Finding all valid branching paths takes a very long time.
/// It seems like there are many valid programs and paths however.
/// This is the lowest coding effort sln.
fn get_path_and_program(
    map: &FlatVec2D<Scaff>,
    mut inter: Intersect,
    Pos {
        mut x,
        mut y,
        mut dx,
        mut dy,
    }: Pos,
) -> VecDeque<u8> {
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
    assert!(
        inter.into_values().all(|d| d.into_iter().all(|v| v)),
        "Did not cross through every intersection. Bad path."
    );
    // concat Forward movements between turns.
    let path = path
        .into_iter()
        .coalesce(|l, r| match (l, r) {
            (Moves::Forward(l), Moves::Forward(r)) => Ok(Moves::Forward(l + r)),
            _ => Err((l, r)),
        })
        .collect_vec();
    get_program(&path).expect("Valid Intcode ascii program")
}

fn part2(mut prog: Vec<i64>, mut ascii: VecDeque<u8>) -> i64 {
    let mut last = 0;
    let mut intcode = IntCode::default();
    prog[0] = 2;
    loop {
        match intcode.execute_til(&mut prog, &mut ascii) {
            Ok(o) => last = o,
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
