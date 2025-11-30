use aoc_shared::{read_input, try_atoi};
use nom::{
    bytes::complete::{tag, take_while1},
    character::is_digit,
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use std::{collections::HashMap, io};

type Output = Vec<(u16, Line)>;

fn parse_input(input: &[u8]) -> Output {
    fn parse_pair(i: &[u8]) -> IResult<&[u8], Line> {
        let mut parse_pair = separated_list1(
            tag(b","),
            map_res(take_while1(is_digit), |digits| {
                try_atoi::<u32, 10>(digits).ok_or("Impossible.")
            }),
        );
        let (i, start) = parse_pair(i)?;
        let (i, _) = tag(b"~")(i)?;
        let (i, end) = parse_pair(i)?;

        if start.len() != 3 || end.len() != 3 {
            panic!("Invalid input");
        }

        let s = (start[0], start[1], start[2]);
        let e = (end[0], end[1], end[2]);

        Ok((i, Line::from((s, e))))
    }
    let (input, lines) =
        separated_list1(tag(b"\n"), parse_pair)(input).expect("expected to parse coords");
    if !input.is_empty() && input != b"\n" {
        panic!("Residue left in input string.");
    }

    // add idents to line.
    let mut lines = (1u16..).zip(lines).collect::<Output>();

    lines.sort_unstable_by_key(|(_, Line((_, _, z1), (_, _, z2)))| *z1.min(z2));
    // lines
    //     .iter()
    //     .for_each(|(u, l)| println!("{}: {l:?}", char::from(b'A' + (*u as u8 - 1))));
    lines
}

#[derive(Debug)]
struct Line(pub (u32, u32, u32), pub (u32, u32, u32));

fn reorder_tuple((a, b): (u32, u32)) -> (u32, u32) {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => (a, b),
        std::cmp::Ordering::Equal => (a, b),
        std::cmp::Ordering::Greater => (b, a),
    }
}

impl From<((u32, u32, u32), (u32, u32, u32))> for Line {
    fn from(value: ((u32, u32, u32), (u32, u32, u32))) -> Self {
        let ((xs, ys, zs), (xe, ye, ze)) = value;

        let (xs, xe) = reorder_tuple((xs, xe));
        let (ys, ye) = reorder_tuple((ys, ye));
        let (zs, ze) = reorder_tuple((zs, ze));

        Self((xs, ys, zs), (xe, ye, ze))
    }
}

impl Line {
    // the pairs are sorted From parsed.
    fn footprint(&self) -> Vec<(u32, u32)> {
        let &(xs, ys, _) = &self.0;
        let &(xe, ye, _) = &self.1;

        if xs == xe {
            (ys..=ye).map(|y| (xs, y)).collect::<Vec<_>>()
        } else if ys == ye {
            (xs..=xe).map(|x| (x, ys)).collect::<Vec<_>>()
        } else {
            std::iter::once((xs, ys)).collect::<Vec<_>>()
        }
    }
}

fn preprocess(i: Output) -> Vec<Vec<u16>> {
    let mut heightmap = HashMap::<(u32, u32), (u32, u16)>::new();
    let max_id = *i.iter().map(|(id, _)| id).max().unwrap() + 1;
    let mut connected = vec![vec![]; max_id as usize];

    i.into_iter().for_each(|(ident, line)| {
        let footprint = line.footprint();
        // find z for block
        let (reachable, new_z) = footprint
            .iter()
            .map(|coord| *heightmap.get(coord).unwrap_or(&(0, 0)))
            .fold((Vec::new(), 0), |(mut acc, mz), (z, ident)| {
                match z.cmp(&mz) {
                    std::cmp::Ordering::Greater => return (vec![ident], z),
                    std::cmp::Ordering::Equal => acc.push(ident),
                    _ => (),
                };
                (acc, mz)
            });
        let new_z = new_z + 1;

        // set new Zs;
        let (_, _, zs) = line.0;
        let (_, _, ze) = line.1;
        // we move the starting interval to lowest z -> highest z
        let zdiff = zs.abs_diff(ze);
        let new_z = zdiff + new_z;
        let nh = (new_z, ident);

        // fixup heighmap
        footprint
            .into_iter()
            .for_each(|coord| _ = heightmap.entry(coord).and_modify(|h| *h = nh).or_insert(nh));

        // add to connected directed graph
        reachable.into_iter().for_each(|id| {
            connected[id as usize].push(ident);
        });
    });
    connected
}

fn solve(connected: Vec<Vec<u16>>) -> (u16, u32) {
    let mut ret = 0;
    let mut ret2 = 0;
    let max_id = connected.len() as u16;
    for i in 1..max_id {
        let mut reachable = vec![false; max_id as usize];
        let mut stack = vec![0u16];
        while let Some(id) = stack.pop() {
            if reachable[id as usize] {
                continue;
            }
            if id == i {
                continue;
            }
            reachable[id as usize] = true;
            stack.extend(connected[id as usize].clone());
        }

        let unmoved = reachable.into_iter().map(|b| b as u16).sum::<u16>();
        let moved = max_id - unmoved - 1;
        ret += (unmoved == max_id - 1) as u16;
        ret2 += moved as u32;
    }

    (ret, ret2)
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed = parse_input(&input);
    let parsed = preprocess(parsed);
    let (part1, part2) = solve(parsed);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
