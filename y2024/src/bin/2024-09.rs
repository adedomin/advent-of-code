use aoc_shared::read_input;
use itertools::Itertools;
use std::{fmt::Write, io};

type Output = Vec<DiskUse>;
type Id = u64;

#[derive(Clone, Copy, Eq, PartialEq)]
enum DiskUse {
    Free,
    Used(Id),
}

impl std::fmt::Debug for DiskUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskUse::Free => f.write_char('.'),
            DiskUse::Used(b) => f.write_char(char::from(*b as u8 + b'0')),
        }
    }
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

fn find_free_w_size(input: &[DiskUse], size: usize) -> Option<usize> {
    if let Some((_, _, start)) = input
        .iter()
        .enumerate()
        .map(|(start, a)| (*a, 1, start))
        .coalesce(|a, b| {
            if a.0 == b.0 {
                Ok((a.0, a.1 + 1, a.2))
            } else {
                Err((a, b))
            }
        })
        .find(|(d, len, _)| match d {
            DiskUse::Free if size <= *len => true,
            DiskUse::Free => false,
            DiskUse::Used(_) => false,
        })
    {
        Some(start)
    } else {
        None
    }
}

fn part2_sol(mut input: Output) -> Id {
    // #[cfg(debug_assertions)]
    // {
    //     input.iter().for_each(|d| print!("{d:?}"));
    //     println!();
    // }
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
                if let Some(free) = find_free_w_size(&input[..j], len) {
                    (0..len).for_each(|idx| {
                        input.swap(free + idx, j + idx);
                    });
                }
            }
        }
        #[cfg(debug_assertions)]
        {
            input.iter().for_each(|d| print!("{d:?}"));
            println!();
        }
        j -= 1;
    }
    input
        .into_iter()
        .enumerate()
        .map(|(i, d)| match d {
            DiskUse::Free => 0,
            DiskUse::Used(id) => (i as Id) * id,
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
