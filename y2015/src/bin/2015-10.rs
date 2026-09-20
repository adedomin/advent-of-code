use aoc_shared::read_input;
use core::fmt::NumBuffer;
use itertools::Itertools;
use std::io;

fn main() -> io::Result<()> {
    let mut input = read_input()?
        .iter()
        .copied()
        .filter(|&chr| chr != b'\n' && chr != b'\r')
        .collect::<Vec<u8>>();

    // p1, 40 times
    for _ in 0..40 {
        input = input
            .iter()
            .dedup_with_count()
            .flat_map(|(count, el)| {
                let mut b = NumBuffer::new();
                let mut b = count.format_into(&mut b).as_bytes().to_owned();
                b.push(*el);
                b
            })
            .collect::<Vec<u8>>();
    }
    let p1 = input.len();

    // p2 is 40+10
    for _ in 0..10 {
        input = input
            .iter()
            .dedup_with_count()
            .flat_map(|(count, el)| {
                let mut b = NumBuffer::new();
                let mut b = count.format_into(&mut b).as_bytes().to_owned();
                b.push(*el);
                b
            })
            .collect::<Vec<u8>>();
    }
    let p2 = input.len();

    println!("Part1: {p1}, Part2: {p2}");
    Ok(())
}
