use std::{
    fmt::Debug,
    io,
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use aoc_shared::read_input;

type Vec2D<T> = Vec<Vec<T>>;

const BORDER: u8 = 255;

fn parse(input: Vec<u8>) -> Octopi {
    let row_width = input.iter().position(|&chr| chr == b'\n').unwrap();
    let col_len = ((input.len() - 1) / (row_width + 1)) + 1;

    let mut octopi = vec![vec![BORDER; row_width + 2]; col_len + 2];

    let mut i = 1usize;
    let mut j = 1usize;
    input.iter().for_each(|&el| {
        if el == b'\n' {
            i = 1;
            j += 1;
        } else if el != b'\n' {
            octopi[j][i] = el - b'0';
            i += 1;
        }
    });
    Octopi(octopi)
}

struct Octopi(Vec2D<u8>);

impl<Idx> Index<Idx> for Octopi
where
    Idx: SliceIndex<[std::vec::Vec<u8>]>,
{
    type Output = <Idx as SliceIndex<[Vec<u8>]>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl<Idx> IndexMut<Idx> for Octopi
where
    Idx: SliceIndex<[std::vec::Vec<u8>]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl Octopi {
    fn height(&self) -> usize {
        self.0.len() - 1
    }

    fn width(&self) -> usize {
        self.0[0].len() - 1
    }
}

impl Debug for Octopi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "= Octopi Map: Width {}, Height {}\n",
                self.width() - 1,
                self.height() - 1,
            )
            .as_str(),
        )?;
        for y in 1..self.height() {
            let mut line = String::new();
            for x in 1..self.width() {
                let octopi = self[y][x] + b'0';
                if octopi == b'0' {
                    line.push('*')
                } else {
                    line.push(' ');
                }
                line.push(octopi as char);
            }
            line.push('\n');
            f.write_str(&line)?;
        }
        Ok(())
    }
}

fn irridate_neighbors(octopi: &mut Octopi, flashset: &mut Vec2D<bool>, x: usize, y: usize) {
    let mut stack = vec![(x, y)];

    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();
        let octopus = &mut octopi[y][x];
        if *octopus == BORDER || flashset[y - 1][x - 1] {
            continue;
        }

        *octopus += 1;
        if *octopus == 10 {
            *octopus = 0;
            flashset[y - 1][x - 1] = true;
            stack.push((x, y - 1)); // north
            stack.push((x + 1, y - 1)); // north-east
            stack.push((x + 1, y)); // east
            stack.push((x + 1, y + 1)); // south-east
            stack.push((x, y + 1)); // south
            stack.push((x - 1, y + 1)); // south-west
            stack.push((x - 1, y)); // west
            stack.push((x - 1, y - 1)); // north-west
        }
    }
}

fn solve(mut octopi: Octopi) -> (usize, usize) {
    let mut synced = usize::MAX;
    let mut run = 0usize;
    let mut flashes = 0usize;

    let mut flashset = vec![vec![false; octopi.width() - 1]; octopi.height() - 1];

    // println!("run  -0: {:?}", octopi);
    loop {
        for y in 1..octopi.height() {
            for x in 1..octopi.width() {
                let octopus = &mut octopi[y][x];
                if *octopus == 9 {
                    irridate_neighbors(&mut octopi, &mut flashset, x, y);
                } else if !flashset[y - 1][x - 1] {
                    *octopus += 1;
                }
            }
        }
        run += 1;

        // count flashes and clear flash bitmap
        let mut total_flash = 0usize;
        for y in 0..flashset.len() {
            for x in 0..flashset[0].len() {
                let octopus = &mut flashset[y][x];
                if *octopus {
                    total_flash += 1;
                }
                *octopus = false;
            }
        }

        if run <= 100 {
            flashes += total_flash;
        }
        if synced == usize::MAX && total_flash == flashset.len() * flashset[0].len() {
            synced = run;
        }
        if run >= 100 && synced != usize::MAX {
            break;
        }
        // println!("run {:03}: {:?}", _runs + 1, octopi);
    }
    (flashes, synced)
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed = parse(input);
    let (p1, p2) = solve(parsed);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
