use std::{collections::VecDeque, io};

use aoc_shared::read_input;

type Vec2D<T> = Vec<Vec<T>>;

const VISITED: u8 = 10;

fn parse(input: Vec<u8>) -> Vec2D<u8> {
    let row_width = input.iter().position(|&chr| chr == b'\n').unwrap();
    let col_len = ((input.len() - 1) / (row_width + 1)) + 1;

    let mut cave = vec![vec![9u8; row_width + 2]; col_len + 2];

    let mut i = 1usize;
    let mut j = 1usize;
    input.iter().for_each(|&el| {
        if el == b'\n' {
            i = 1;
            j += 1;
        } else if el != b'\n' {
            cave[j][i] = el - b'0';
            i += 1;
        }
    });
    cave
}

fn get_lows(cave: &Vec2D<u8>) -> (Vec<(usize, usize)>, u64) {
    let mut low_risk = 0u64;
    let mut lows = vec![];
    for y in 1..(cave.len() - 1) {
        for x in 1..(cave[0].len() - 1) {
            let val_at = cave[y][x];
            if cave[y - 1][x] > val_at
                && cave[y][x + 1] > val_at
                && cave[y + 1][x] > val_at
                && cave[y][x - 1] > val_at
            {
                lows.push((x, y));
                low_risk += 1 + val_at as u64;
            }
        }
    }
    (lows, low_risk)
}

fn find_basins(mut cave: Vec2D<u8>, lows: Vec<(usize, usize)>) -> u64 {
    let mut largest_basins = [0u64, 0u64, 0u64];
    for (x, y) in lows {
        if cave[y][x] == VISITED {
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back((x, y));
        let mut sum = 0;

        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            let val_at = &mut cave[y][x];
            if *val_at == VISITED || *val_at == 9 {
                continue;
            }

            *val_at = 10;
            sum += 1;

            queue.push_back((x, y - 1));
            queue.push_back((x + 1, y));
            queue.push_back((x, y + 1));
            queue.push_back((x - 1, y));
        }

        if largest_basins[0] < sum {
            largest_basins[0] = sum;
            largest_basins.sort_unstable();
        }
    }
    largest_basins[0] * largest_basins[1] * largest_basins[2]
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed = parse(input);
    let (lows, p1) = get_lows(&parsed);
    let p2 = find_basins(parsed, lows);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
