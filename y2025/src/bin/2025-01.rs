use std::io;

use aoc_shared::read_input_to_string;

type Int = i32;

fn parse_input(i: &str) -> Vec<Int> {
    i.split_ascii_whitespace()
        .map(|line| {
            if let Some(num) = line.strip_prefix('R') {
                num.parse::<Int>().expect("Number")
            } else if let Some(num) = line.strip_prefix('L') {
                -num.parse::<Int>().expect("Number")
            } else {
                panic!("Not a rotation!");
            }
        })
        .collect::<Vec<_>>()
}

const DIAL_LEN: Int = 100;
const DIAL_START: Int = 50;

fn solve(i: &[Int]) -> (Int, Int) {
    i.iter()
        .fold(((0, 0), DIAL_START), |((p1, p2), dial), rot| {
            let sum = dial + rot;
            let ndial = sum.rem_euclid(DIAL_LEN);
            let rot_around = if *rot < 0 {
                (DIAL_LEN - dial - *rot) / DIAL_LEN - if dial == 0 { 1 } else { 0 } // edge-case, div returns 1 extra on 0.
            } else {
                sum / DIAL_LEN
            };
            ((p1 + Int::from(ndial == 0), p2 + rot_around), ndial)
        })
        .0
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let input = parse_input(input.trim());
    let (part1, part2) = solve(&input);
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
