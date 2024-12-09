use aoc_shared::read_input;
use std::io;

type Output = Vec<(DiskUse, usize, usize)>;
type Id = usize;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum DiskUse {
    Free,
    Used(Id),
}

fn parse_input(input: &[u8]) -> Output {
    let mut id = 0;
    let mut totlen = 0;
    input
        .iter()
        .filter(|d| d.is_ascii_digit())
        .enumerate()
        .map(|(start, sz)| {
            let len = (sz - b'0') as usize;
            let r = if start & 1 == 0 {
                let r = (DiskUse::Used(id), totlen, totlen + len);
                id += 1;
                r
            } else {
                (DiskUse::Free, totlen, totlen + len)
            };
            totlen += len;
            r
        })
        .collect::<Output>()
}

fn part1_sol(mut input: Output) -> Id {
    let mut ans = 0;
    let mut i = 1; // we can skip the first (always zero and filled)
    let mut j = input.len() - 1;
    while i <= j {
        match input[i] {
            (DiskUse::Free, start, len) => {
                match input[j] {
                    (DiskUse::Free, _, _) => j -= 1,
                    (DiskUse::Used(id), s, l) => {
                        let min = if (l - s) >= (len - start) {
                            len - start
                        } else {
                            l - s
                        };
                        for s in start..start + min {
                            ans += s * id;
                            // use up spaces
                        }
                        input[i].1 += min;
                        input[j].2 -= min;
                        if input[i].1 == input[i].2 {
                            i += 1;
                        }
                        if input[j].1 == input[j].2 {
                            j -= 1;
                        }
                    }
                };
            }
            (DiskUse::Used(id), start, len) => {
                (start..len).for_each(|idx| ans += idx * id);
                input[i].2 -= len - start;
                i += 1;
            }
        }
    }
    ans
}

type FreeList = Vec<(usize, usize)>;
type UsedList = Vec<(Id, usize, usize)>;

fn part2_sol(input: Output) -> Id {
    let mut ans = 0;
    let mut freelist: FreeList = Vec::with_capacity(input.len());
    let mut usedlist: UsedList = Vec::with_capacity(input.len());
    input.into_iter().for_each(|(d, s, l)| match d {
        DiskUse::Free => freelist.push((s, l)),
        DiskUse::Used(id) => usedlist.push((id, s, l)),
    });

    usedlist.into_iter().rev().for_each(|(id, us, ul)| {
        if let Some((fstart, _)) = freelist
            .iter_mut()
            .find(|(fs, fl)| (ul - us) <= (fl - fs) && *fs < us)
        {
            (0..(ul - us)).for_each(|idx| ans += (*fstart + idx) * id);
            *fstart += ul - us;
        } else {
            (us..ul).for_each(|idx| ans += idx * id);
        }
    });
    ans
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(parsed_input.clone());
    let part2 = part2_sol(parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
