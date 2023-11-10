use std::{collections::HashMap, io};
use aoc_shared::{fold_decimal, read_input};

type Output<'a> = HashMap<Vec<&'a [u8]>, usize>;

#[derive(PartialEq, Eq, Debug)]
enum Pstate<'a> {
    CommandCD(&'a [u8]),
    CommandCDBack,
    ListingDir(&'a [u8]),
    ListingFile(&'a [u8], usize),
}

const FS_AVAIL: usize = 70_000_000;
const FS_TARGET: usize = 30_000_000;

fn parse_input(input: &[u8]) -> Output {
    input
        .split(|&chr| chr == b'\n')
        .filter_map(|line| {
            let (words, _) = line.splitn(3, |&chr| chr == b' ').fold(
                ([b"" as &[u8]; 3], 0),
                |(mut acc, idx), word| {
                    acc[idx] = word;
                    (acc, idx + 1)
                },
            );
            Some(match words {
                [b"$", b"cd", dir] if dir == b".." => Pstate::CommandCDBack,
                [b"$", b"cd", dir] => Pstate::CommandCD(dir),
                [b"$", b"ls", _] => return None,
                [b"dir", dir, _] => Pstate::ListingDir(dir),
                [b"", b"", b""] => return None,
                [size, file, _] => {
                    Pstate::ListingFile(file, size.iter().fold(0usize, fold_decimal))
                }
            })
        })
        .fold(
            (HashMap::new(), Vec::new()),
            |(mut acc, mut dirstack), token| match token {
                Pstate::CommandCD(dir) => {
                    dirstack.push(dir);
                    (acc, dirstack)
                }
                Pstate::CommandCDBack => {
                    if dirstack.pop().is_some() {
                        (acc, dirstack)
                    } else {
                        panic!("dir stack is empty!")
                    }
                }
                Pstate::ListingDir(_dir) => (acc, dirstack),
                Pstate::ListingFile(_filename, size) => {
                    let mut slice = &dirstack[..];
                    while !slice.is_empty() {
                        if let Some(dir_sum) = acc.get_mut(slice) {
                            *dir_sum += size;
                        } else {
                            acc.insert(slice.to_vec(), size);
                        }
                        slice = &slice[0..slice.len() - 1];
                    }
                    (acc, dirstack)
                }
            },
        )
        .0
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let part1 = parsed_input
        .iter()
        .by_ref()
        .filter_map(|(_, sizeof)| {
            if *sizeof > 100_000 {
                None
            } else {
                Some(sizeof)
            }
        })
        .sum::<usize>();

    let rootfs: Vec<&[u8]> = vec![b"/"];
    let fs_used = *parsed_input.get(&rootfs).expect("no root directory");
    let fs_used = FS_AVAIL - fs_used;
    let target = fs_used.abs_diff(FS_TARGET);

    let part2 = parsed_input
        .iter()
        .filter_map(|(_, sizeof)| {
            if target <= *sizeof {
                Some(*sizeof)
            } else {
                None
            }
        })
        .min()
        .expect("No single directory big enough to meet filesystem free space goals.");

    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
