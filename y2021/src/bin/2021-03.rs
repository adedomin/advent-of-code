use std::{io, mem};

use aoc_shared::read_input;

#[derive(PartialEq, Debug)]
enum Sensor {
    O2,
    CO2,
}

fn parse(input: Vec<u8>) -> ([i32; 128], Vec<u128>, usize) {
    let (gamma_arr, input_parsed, _, bitsize) = input.iter().rfold(
        ([0i32; 128], Vec::<u128>::new(), 0usize, 0usize),
        |(mut gamma_arr, mut acc, idx, bitsize), &digit| match digit {
            b'1' => {
                if idx == 0 {
                    acc.push(1);
                } else if let Some(last) = acc.last_mut() {
                    *last |= 1 << idx;
                } else {
                    unreachable!();
                }
                gamma_arr[idx] += 1;
                (gamma_arr, acc, idx + 1, bitsize)
            }
            b'0' => {
                if acc.last().is_none() || idx == 0 {
                    acc.push(0);
                }
                gamma_arr[idx] -= 1;
                (gamma_arr, acc, idx + 1, bitsize)
            }
            b'\n' => (gamma_arr, acc, 0, if idx > bitsize { idx } else { bitsize }),
            _ => panic!("Unknown file format."),
        },
    );
    (gamma_arr, input_parsed, bitsize)
}

fn find_oxy_co2_rating(input: Vec<u128>, bitsize: usize) -> u128 {
    let mut ones = vec![];
    let mut zeroes = vec![];

    let mut o2 = 0;
    let mut co2 = 0;

    for stype in [Sensor::O2, Sensor::CO2] {
        let mut iter_arr = input.clone();
        for i in (0..bitsize).rev() {
            let filt = 1 << i;
            for &num in &iter_arr {
                if num & filt == filt {
                    ones.push(num);
                } else {
                    zeroes.push(num);
                }
            }
            if ones.is_empty() {
                mem::swap(&mut iter_arr, &mut zeroes);
            } else if zeroes.is_empty() {
                mem::swap(&mut iter_arr, &mut ones);
            } else {
                match stype {
                    Sensor::O2 => {
                        if ones.len() >= zeroes.len() {
                            mem::swap(&mut iter_arr, &mut ones);
                        } else {
                            mem::swap(&mut iter_arr, &mut zeroes);
                        }
                    }
                    Sensor::CO2 => {
                        if ones.len() >= zeroes.len() {
                            mem::swap(&mut iter_arr, &mut zeroes);
                        } else {
                            mem::swap(&mut iter_arr, &mut ones);
                        }
                    }
                }
            }
            ones.clear();
            zeroes.clear();
        }
        match stype {
            Sensor::O2 => o2 = iter_arr[0],
            Sensor::CO2 => co2 = iter_arr[0],
        }
    }
    o2 * co2
}

fn find_gamma_epsilon(input: [i32; 128], bitsize: usize) -> u128 {
    let common_bits = &input[..bitsize];
    let gamma =
        common_bits.iter().enumerate().fold(
            0u128,
            |acc, (idx, &num)| {
                if num >= 0 {
                    acc | 1 << idx
                } else {
                    acc
                }
            },
        );
    let epsilon = gamma ^ (2u128.pow(bitsize as u32) - 1);
    gamma * epsilon
}

fn solve(input: Vec<u8>) -> (u128, u128) {
    let (common_bits, parsed, bitsize) = parse(input);

    (
        find_gamma_epsilon(common_bits, bitsize),
        find_oxy_co2_rating(parsed, bitsize),
    )
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let (p1, p2) = solve(input);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
