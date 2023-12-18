use aoc_shared::{atoi, destructure_or_none, read_input, Token, Tokenize};
use itertools::Itertools;
use std::io;

const ARBITRARY_START: (i64, i64) = (100_000, 100_000);

type Instructions = Vec<(u8, i64)>;

fn parse1(input: &[u8]) -> Instructions {
    input
        .tokenize()
        .flat_map(|word| destructure_or_none!(Token::Something|word| = word))
        .tuples()
        .map(|(dir, dist, _)| (dir[0], atoi::<i64, 10>(dist)))
        .collect_vec()
}

fn parse2(input: &[u8]) -> Instructions {
    input
        .tokenize()
        .flat_map(|word| destructure_or_none!(Token::Something|word| = word))
        .tuples()
        .map(|(_, _, hex)| {
            assert_eq!(hex.len(), 6);
            let (dist, dir) = hex.split_at(5);
            let dir = match dir[0] - b'0' {
                0 => b'R',
                1 => b'D',
                2 => b'L',
                3 => b'U',
                _ => panic!("Invalid direction given: {:?}", dir[0]),
            };
            let dist = atoi::<i64, 16>(dist);
            (dir, dist)
        })
        .collect_vec()
}

fn solve(instructions: Instructions) -> i64 {
    let (points, perimeter) = instructions.into_iter().fold(
        (vec![ARBITRARY_START], 0i64),
        |(mut acc, perim), (dir, amt)| {
            let &(x, y) = acc.last().unwrap();
            let next = match dir {
                b'U' => (x, y - amt),
                b'D' => (x, y + amt),
                b'L' => (x - amt, y),
                b'R' => (x + amt, y),
                _ => panic!("Invalid direction given: {:?}", dir),
            };
            acc.push(next);
            (acc, amt + perim)
        },
    );
    assert_eq!(points[0], *points.last().unwrap());
    (points
        .into_iter()
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
        .sum::<i64>()
        .abs()
        + perimeter) // unlike day 10, perimeter IS a part of the area.
        / 2
        + 1
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let instructions = parse1(&input);
    let part1 = solve(instructions);
    print!("Part1: {part1}, ");

    let instructions = parse2(&input);
    let part2 = solve(instructions);
    print!("Part2: {part2}");
    println!();
    Ok(())
}
