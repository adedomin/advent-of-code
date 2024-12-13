use aoc_shared::{fold_decimal_from, read_input_to_string};
use itertools::Itertools;
use std::io;

type Int = i64;
type Output = Vec<Game>;

#[derive(Debug)]
struct Game {
    a: (Int, Int),
    b: (Int, Int),
    t: (Int, Int),
}

fn parse_input(input: &str) -> Output {
    input
        .split(|chr: char| !chr.is_ascii_digit())
        .filter(|word| !word.is_empty())
        .map(|word| fold_decimal_from(word.as_bytes()))
        .tuples()
        .map(|(a, b, c, d, e, f)| Game {
            a: (a, b),
            b: (c, d),
            t: (e, f),
        })
        .collect::<Vec<Game>>()
}

type Point = (Int, Int);

fn get_tokens((ax, ay): Point, (bx, by): Point, (tx, ty): Point) -> Int {
    let atok = (tx * by - ty * bx) / (by * ax - bx * ay);
    let btok = (tx * ay - ty * ax) / (ay * bx - by * ax);
    let (x, y) = (ax * atok + bx * btok, ay * atok + by * btok);
    if (x, y) == (tx, ty) {
        atok * 3 + btok
    } else {
        0
    }
}

fn solve(input: Output) -> (Int, Int) {
    input
        .into_iter()
        .fold((0, 0), |(p1, p2), Game { a, b, t }| {
            (
                p1 + get_tokens(a, b, t),
                p2 + get_tokens(a, b, (t.0 + TARGET_CORRECTION, t.1 + TARGET_CORRECTION)),
            )
        })
}

const TARGET_CORRECTION: Int = 10_000_000_000_000;

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let (part1, part2) = solve(parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
