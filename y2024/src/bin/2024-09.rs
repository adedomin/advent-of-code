use aoc_shared::read_input;
use std::{io, iter};

type UsedList = Vec<Use>;
type Output = (Vec<DiskUse>, UsedList, UsedList);

#[derive(Clone, Copy)]
enum DiskUse {
    Free,
    Used(usize),
}

#[derive(Clone, Copy)]
struct Use {
    id: usize,
    start: usize,
    end: usize,
}

fn parse_input(input: &[u8]) -> Output {
    let mut disk = vec![];
    let mut free = vec![];
    let mut used = vec![];
    input
        .iter()
        .filter(|d| d.is_ascii_digit())
        .enumerate()
        .for_each(|(start, sz)| {
            let len = (sz - b'0') as usize;
            if start % 2 == 0 {
                let id = start / 2;
                let start = disk.len();
                disk.extend(iter::repeat_n(DiskUse::Used(id), len));
                used.push(Use {
                    id,
                    start,
                    end: disk.len(),
                });
            } else {
                let start = disk.len();
                disk.extend(iter::repeat_n(DiskUse::Free, len));
                free.push(Use {
                    id: 0,
                    start,
                    end: disk.len(),
                });
            };
        });
    (disk, free, used)
}

fn part1_sol(input: Vec<DiskUse>) -> usize {
    let mut ans = 0;
    let mut i = 0;
    let mut j = input.len() - 1;
    while i <= j {
        match input[i] {
            DiskUse::Free => loop {
                match input[j] {
                    DiskUse::Free => j -= 1,
                    DiskUse::Used(id) => {
                        ans += i * id;
                        j -= 1;
                        break;
                    }
                }
            },
            DiskUse::Used(id) => ans += i * id,
        }
        i += 1;
    }
    ans
}

fn part2_sol(mut freelist: UsedList, usedlist: UsedList) -> usize {
    let mut ans = 0;
    usedlist
        .into_iter()
        .rev()
        .for_each(|Use { id, start, end }| {
            if let Some(Use { start: fstart, .. }) = freelist.iter_mut().find(
                |Use {
                     start: fs, end: fe, ..
                 }| (end - start) <= (fe - fs) && *fs < start,
            ) {
                (0..(end - start)).for_each(|idx| ans += (*fstart + idx) * id);
                *fstart += end - start;
            } else {
                (start..end).for_each(|idx| ans += idx * id);
            }
        });
    ans
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (disk, freelist, usedlist) = parse_input(&input);
    let part1 = part1_sol(disk);
    let part2 = part2_sol(freelist, usedlist);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
