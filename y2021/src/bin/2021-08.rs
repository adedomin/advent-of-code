use std::io;

use aoc_shared::read_input_to_string;

fn set_sixer(arr: &mut [u8; 3], bits: u8) -> Result<(), ()> {
    for a in arr {
        if *a == bits {
            return Ok(());
        } else if *a == 0 {
            *a = bits;
            return Ok(());
        }
    }
    Err(())
}

fn solve(signals: &[u8], digits: &[u8]) -> u64 {
    // 0 6 9 -> 6 bits
    let mut sixers: [u8; 3] = [0; 3];
    // 2 3 5 -> 5 bits, unnessary to track.
    // let mut fivers

    // 2 bits
    let mut one = 8;
    // 4 bits
    let mut four = 8;
    // 3 bits.
    let mut seven = 8;
    // 7 bits. all set.
    let eight = 0b0111_1111;

    for &segment in signals.iter().chain(digits) {
        match segment.count_ones() {
            2 => one = segment,
            3 => seven = segment,
            4 => four = segment,
            6 => set_sixer(&mut sixers, segment).expect("too many 6 bit segments."),
            5..=7 => (),
            _ => panic!("bad input"),
        }
    }

    if one == 8 || four == 8 || seven == 8 || sixers.contains(&0) {
        panic!("Not enough input");
    }

    // Parts:
    //     a
    //    __
    // g |_c| b
    // f |__| d
    //     e

    let a = one ^ seven;
    let ef = four ^ a ^ eight;
    let abcdg = four ^ a;

    let nine = *sixers
        .iter()
        .find(|&&seg| (seg ^ abcdg).count_ones() == 1)
        .expect("Could not find nine.");

    let e = abcdg ^ nine;
    let f = ef ^ e;

    let six = *sixers
        .iter()
        .find(|&&seg| seg != nine && (seg ^ abcdg ^ one).count_ones() == 3)
        .expect("Could not find six.");

    let b = six ^ nine ^ f;
    let d = one ^ b;

    let zero = *sixers
        .iter()
        .find(|&&seg| seg != six && seg != nine)
        .expect("Could not find zero.");

    let c = zero ^ eight;
    let g = abcdg ^ a ^ b ^ c ^ d;

    // resolve the 5 bits from parts.
    let two = a | b | c | e | f;
    let three = a | b | c | d | e;
    let five = a | c | d | e | g;

    let segement_digits = [zero, one, two, three, four, five, six, seven, eight, nine];

    digits.iter().fold(0u64, |acc, &digit| {
        let decoded = segement_digits
            .iter()
            .position(|&d| d == digit)
            .expect("Digit not found.") as u64;
        acc * 10 + decoded
    })
}

fn parse(input: &str) -> Vec<u8> {
    input
        .split_ascii_whitespace()
        .filter(|word| *word != "|")
        .map(|word| {
            word.as_bytes()
                .iter()
                .fold(0, |acc, &d| acc | 1 << (d - b'a'))
        })
        .collect()
}

pub fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let sets = parse(&input);
    // input contains 10 signal patterns | 4 digit patterns, per line.
    assert!(
        sets.len().is_multiple_of(14),
        "input is malformed; missing patterns."
    );
    let (p1, p2) = sets.chunks_exact(14).fold((0, 0), |(p1, p2), segments| {
        let (signal, digits) = segments.split_at(10);
        (
            p1 + digits.iter().fold(0, |acc, dig| {
                acc + [2, 3, 4, 7].contains(&dig.count_ones()) as i32
            }),
            p2 + solve(signal, digits),
        )
    });
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
