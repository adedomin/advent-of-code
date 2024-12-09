use aoc_shared::read_input;
use itertools::Itertools;
use std::io;

type Output = Vec<DiskUse>;
type Id = u64;

#[derive(Clone, Copy, Eq, PartialEq)]
enum DiskUse {
    Free,
    Used(Id),
}

fn digit_to_len(v: u8) -> usize {
    (v - b'0') as usize
}

fn parse_input(input: &[u8]) -> Output {
    let mut id = 0;
    let mut disk = vec![];
    input
        .iter()
        .filter(|d| d.is_ascii_digit())
        .enumerate()
        .for_each(|(i, &v)| {
            if (i & 1) == 0 {
                disk.extend(vec![DiskUse::Used(id); digit_to_len(v)]);
                id += 1;
            } else {
                disk.extend(vec![DiskUse::Free; digit_to_len(v)]);
            }
        });
    disk
}

fn part1_sol(input: &Output) -> Id {
    let mut ans = 0;
    let mut i = 0;
    let mut j = input.len() - 1;
    while i <= j {
        match input[i] {
            DiskUse::Free => loop {
                match input[j] {
                    DiskUse::Free => j -= 1,
                    DiskUse::Used(id) => {
                        ans += (i as Id) * id;
                        j -= 1;
                        break;
                    }
                };
            },
            DiskUse::Used(id) => ans += (i as Id) * id,
        }
        i += 1;
    }
    ans
}

fn get_freelist(input: &[DiskUse]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .map(|(start, a)| (*a, start, start + 1))
        .coalesce(|a, b| {
            if a.0 == b.0 {
                Ok((a.0, a.1, a.2 + 1))
            } else {
                Err((a, b))
            }
        })
        .filter_map(|(d, s, l)| match d {
            DiskUse::Free => Some((s, l)),
            DiskUse::Used(_) => None,
        })
        .collect::<Vec<(usize, usize)>>()
}

fn part2_sol(input: &Output) -> Id {
    let mut ans = 0;
    let mut freelist = get_freelist(input);
    let mut j = input.len() - 1;
    'out: while j != 0 {
        match input[j] {
            DiskUse::Free => (),
            DiskUse::Used(cid) => {
                let mut nj = j;
                while let DiskUse::Used(id) = input[nj] {
                    if cid != id {
                        break;
                    }
                    nj -= 1;
                    if nj == 0 {
                        break 'out;
                    }
                }
                let len = j - nj;
                j -= len - 1;
                if let Some((fstart, _)) =
                    freelist.iter_mut().find(|(s, l)| len <= (l - s) && j >= *l)
                {
                    (0..len).for_each(|idx| ans += ((*fstart + idx) as Id) * cid);
                    *fstart += len;
                } else {
                    (0..len).for_each(|idx| ans += ((j + idx) as Id) * cid);
                }
            }
        }
        j -= 1;
    }
    ans
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
