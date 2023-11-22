use std::io;

use aoc_shared::{read_input, AoCTokenizer, Token};

const PIPE: u8 = 0b1000_0000;

fn contains_bits(arr: u32, bits: u8) -> bool {
    let (high, mid, low) = decompose_bit_arr(arr);
    high == bits || mid == bits || low == bits
}

fn decompose_bit_arr(arr: u32) -> (u8, u8, u8) {
    (
        (arr & 0b1111_1111) as u8,
        ((arr & 0b1111_1111_0000_0000) >> 8) as u8,
        ((arr & 0b1111_1111_0000_0000_0000_0000) >> 16) as u8,
    )
}

fn set_len_bit_arr(arr: u32, value: u32) -> u32 {
    (arr & 0b1111_1111_1111_1111_1111_1111) | value << 24
}

fn solve(segments: &[u8]) -> u64 {
    // 0 6 9
    let mut sixers = 0u32;
    // 2 3 5
    let mut fivers = 0u32;

    let mut divider = 0usize;

    // zero
    let mut one = 8;
    // two
    // three
    let mut four = 8;
    // five
    // six
    let mut seven = 8;
    let eight = 0b0111_1111; // eight is always 7bits

    for (idx, &segment) in segments.iter().enumerate() {
        match segment.count_ones() {
            1 => divider = idx,
            2 => one = segment,
            3 => seven = segment,
            4 => four = segment,
            5 if !contains_bits(fivers, segment) && (fivers >> 24) < 3 => {
                let pos = fivers >> 24;
                fivers |= (segment as u32) << (pos * 8);
                fivers |= set_len_bit_arr(fivers, pos + 1);
            }
            6 if !contains_bits(sixers, segment) && (sixers >> 24) < 3 => {
                let pos = sixers >> 24;
                sixers |= (segment as u32) << (pos * 8);
                sixers = set_len_bit_arr(sixers, pos + 1);
            }
            5..=7 => (),
            _ => panic!("bad input"),
        }
    }

    if one == 8 || four == 8 || seven == 8 {
        panic!("Not enough input");
    }

    // Parts:

    //    __  a
    // g |_c| b
    // f |__| d
    //    e
    // a
    // b
    // c
    // d
    // e
    // f
    // g

    let a = one ^ seven;
    let ef = four ^ a ^ eight;
    let abcdg = four ^ a;

    let (sixer_1, sixer_2, sixer_3) = decompose_bit_arr(sixers);
    let nine = if (sixer_1 ^ abcdg).count_ones() == 1 {
        sixer_1
    } else if (sixer_2 ^ abcdg).count_ones() == 1 {
        sixer_2
    } else if (sixer_3 ^ abcdg).count_ones() == 1 {
        sixer_3
    } else {
        panic!("Could not find nine");
    };

    let e = abcdg ^ nine;
    let f = ef ^ e;
    let six = if sixer_1 != nine && (sixer_1 ^ abcdg ^ one).count_ones() == 3 {
        sixer_1
    } else if sixer_2 != nine && (sixer_2 ^ abcdg ^ one).count_ones() == 3 {
        sixer_2
    } else if sixer_3 != nine && (sixer_3 ^ abcdg ^ one).count_ones() == 3 {
        sixer_3
    } else {
        panic!("Could not find six");
    };

    let b = six ^ nine ^ f;
    let d = one ^ b;
    let zero = if sixer_1 != six && sixer_1 != nine {
        sixer_1
    } else if sixer_2 != six && sixer_2 != nine {
        sixer_2
    } else if sixer_3 != six && sixer_3 != nine {
        sixer_3
    } else {
        panic!("Could not find zero");
    };

    let c = zero ^ eight;
    let g = abcdg ^ a ^ b ^ c ^ d;

    let two = a | b | c | e | f;
    let three = a | b | c | d | e;
    let five = a | c | d | e | g;

    let segement_digits = [zero, one, two, three, four, five, six, seven, eight, nine];
    //    println!(
    //        "
    //        a = {:#09b}
    //{:#09b} = g     b = {:#09b}
    //        c = {:#09b}
    //{:#09b} = f     d = {:#09b}
    //        e = {:#09b}",
    //        a, b, c, d, e, f, g
    //    );

    let (_, display) = segments.split_at(divider);
    display.iter().skip(1).fold(0u64, |acc, &digit| {
        let decoded = segement_digits.iter().position(|&d| d == digit).unwrap() as u64;
        acc * 10 + decoded
    })
}

fn parse(input: Vec<u8>) -> (u64, u64) {
    let mut numcnt = 0u64;
    let mut sum = 0u64;

    let mut past_delim = false;
    let mut parsed = vec![];
    for token in AoCTokenizer::new(&input) {
        match token {
            Token::Something(digits) => {
                numcnt += if past_delim {
                    let digit_cnt = digits.len();
                    if (2..=4).contains(&digit_cnt) || digit_cnt == 7 {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                };
                parsed.push(digits.iter().fold(0u8, |acc, &d| acc | 1 << (d - b'a')));
            }
            Token::Delimiter(delim) if delim == b'|' => {
                parsed.push(PIPE);
                past_delim = true;
            }
            Token::Newline | Token::DoubleNewline => {
                sum += solve(&parsed);
                parsed.clear();
                past_delim = false;
            }
            Token::Space | Token::Delimiter(_) | Token::End => (),
        }
    }
    (numcnt, sum)
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let (p1, p2) = parse(input);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
