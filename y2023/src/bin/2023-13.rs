use aoc_shared::read_input_to_string;
use bitvec::vec::BitVec;
use std::io;

type Output = (Vec<BitVec>, Vec<BitVec>);

fn transpose_image(input: &[u8]) -> Output {
    let mut hori = vec![];
    let mut r = BitVec::new();
    input.iter().for_each(|&point| {
        if point == b'\n' {
            hori.push(std::mem::take(&mut r));
        } else {
            r.push(point == b'#');
        }
    });
    if !r.is_empty() {
        hori.push(r);
    }

    let mut x = 0usize;
    let mut vert = vec![BitVec::new(); hori[0].len()];
    input.iter().for_each(|&point| {
        if point == b'\n' {
            x = 0;
        } else {
            vert[x].push(point == b'#');
            x += 1;
        }
    });

    (hori, vert)
}

fn parse_input(input: &str) -> Vec<Output> {
    input
        .split("\n\n")
        .map(|split| transpose_image(split.as_bytes()))
        .collect::<Vec<Output>>()
}

/// due to part2, we re-write symmetry check to check for counts of asym at a given point
/// part2 introduces the concept of a off-by-one flaw, thus we accept all off-by-ones as sym
/// at given col/row.
fn asym<'a>(iter: impl Iterator<Item = (&'a BitVec, &'a BitVec)>) -> usize {
    iter.fold(0, |acc, (t, b)| {
        acc + t
            .iter()
            .zip(b.iter())
            .fold(0, |acc, (t1, b1)| acc + (t1 != b1) as usize)
    })
}

fn sym_check(v: &[BitVec], smudge_factor: usize) -> Result<(), usize> {
    (1..v.len()).try_fold((), |_, pos| {
        let (top, bot) = v.split_at(pos);
        if asym(top.iter().rev().zip(bot.iter())) == smudge_factor {
            Err(pos)
        } else {
            Ok(())
        }
    })
}

fn solve(maps: &[Output], smudge_factor: usize) -> usize {
    let (h, v) = maps.iter().fold((0, 0), |(h, v), (hor, ver)| {
        if let Err(x) = sym_check(hor, smudge_factor) {
            (h + x, v)
        } else if let Err(y) = sym_check(ver, smudge_factor) {
            (h, v + y)
        } else {
            panic!("An input pattern does not reflect vertically or horizontally!");
        }
    });
    h * 100 + v
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let patts = parse_input(&input);
    let part1 = solve(&patts, 0);
    let part2 = solve(&patts, 1);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
