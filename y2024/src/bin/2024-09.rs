use aoc_shared::read_input;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    io, iter,
};

const MAX_FSIZE: usize = 9;
type FreeList = Vec<BinaryHeap<Reverse<(usize, usize)>>>;
type UsedList = Vec<Use>;
type Output = (Vec<DiskUse>, FreeList, UsedList);

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
    let mut free: FreeList = vec![BinaryHeap::new(); MAX_FSIZE];
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
                let end = start + len;
                disk.extend(iter::repeat_n(DiskUse::Used(id), len));
                assert!(len != 0, "What do we do if we have a zero sized file?");
                used.push(Use { id, start, end });
            } else {
                let start = disk.len();
                let end = start + len;
                disk.extend(iter::repeat_n(DiskUse::Free, len));
                // the secret weapon... we have a max file size.
                // so we can just store free space in a heap for each free size available
                free[..len]
                    .iter_mut()
                    .for_each(|heap| heap.push(Reverse((start, end))));
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

fn part2_sol(mut freelist: FreeList, mut usedlist: UsedList) -> usize {
    // since we store the free space for each size it could cover, we need to track when we use it.
    let mut already_used = HashSet::new();
    usedlist
        .iter_mut()
        .rev()
        .for_each(|Use { start, end, .. }| {
            loop {
                if let Some(Reverse((fs, fe))) = freelist[*end - *start - 1].pop() {
                    // left of this free space or we already seen this "free space"
                    if fs >= *start || !already_used.insert((fs, fe)) {
                        continue;
                    } else {
                        // reinsert smaller free space, if applicable
                        let len = (fe - fs) - (*end - *start);
                        freelist[..len]
                            .iter_mut()
                            .for_each(|heap| heap.push(Reverse((fe - len, fe))));
                    }
                    // move
                    *end = fs + (*end - *start);
                    *start = fs;
                }
                break;
            }
        });
    usedlist
        .into_iter()
        .map(|Use { id, start, end }| (start..end).sum::<usize>() * id)
        .sum()
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
