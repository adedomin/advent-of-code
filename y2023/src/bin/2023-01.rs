use aoc_shared::{array_windows, destructure_or_none, read_input, Token, Tokenize};
use std::io;

fn parse_input_1(input: &[u8]) -> i32 {
    input
        .tokenize()
        .flat_map(|t| destructure_or_none!(Token::Something|word| = t))
        .map(|word| {
            let d1 = word
                .iter()
                .find(|d| d.is_ascii_digit())
                .expect("some lines do not have a digit")
                - b'0';
            let d2 = word.iter().rfind(|d| d.is_ascii_digit()).unwrap() - b'0';
            (d1 * 10 + d2) as i32
        })
        .sum()
}

fn extract_num(w: &[u8; 5]) -> u8 {
    match w {
        [b'1', _, _, _, _] => 1,
        [b'2', _, _, _, _] => 2,
        [b'3', _, _, _, _] => 3,
        [b'4', _, _, _, _] => 4,
        [b'5', _, _, _, _] => 5,
        [b'6', _, _, _, _] => 6,
        [b'7', _, _, _, _] => 7,
        [b'8', _, _, _, _] => 8,
        [b'9', _, _, _, _] => 9,
        [b'o', b'n', b'e', _, _] => 1,
        [b't', b'w', b'o', _, _] => 2,
        [b't', b'h', b'r', b'e', b'e'] => 3,
        [b'f', b'o', b'u', b'r', _] => 4,
        [b'f', b'i', b'v', b'e', _] => 5,
        [b's', b'i', b'x', _, _] => 6,
        [b's', b'e', b'v', b'e', b'n'] => 7,
        [b'e', b'i', b'g', b'h', b't'] => 8,
        [b'n', b'i', b'n', b'e', _] => 9,
        _ => 10,
    }
}

fn extract_num_back(w: &[u8; 5]) -> u8 {
    match w {
        [_, _, _, _, b'1'] => 1,
        [_, _, _, _, b'2'] => 2,
        [_, _, _, _, b'3'] => 3,
        [_, _, _, _, b'4'] => 4,
        [_, _, _, _, b'5'] => 5,
        [_, _, _, _, b'6'] => 6,
        [_, _, _, _, b'7'] => 7,
        [_, _, _, _, b'8'] => 8,
        [_, _, _, _, b'9'] => 9,
        [_, _, b'o', b'n', b'e'] => 1,
        [_, _, b't', b'w', b'o'] => 2,
        [b't', b'h', b'r', b'e', b'e'] => 3,
        [_, b'f', b'o', b'u', b'r'] => 4,
        [_, b'f', b'i', b'v', b'e'] => 5,
        [_, _, b's', b'i', b'x'] => 6,
        [b's', b'e', b'v', b'e', b'n'] => 7,
        [b'e', b'i', b'g', b'h', b't'] => 8,
        [_, b'n', b'i', b'n', b'e'] => 9,
        _ => 10,
    }
}
fn parse_input_2(input: &[u8]) -> i32 {
    input
        .tokenize()
        .flat_map(|t| destructure_or_none!(Token::Something|word| = t))
        .map(|word| {
            let mut word_pad = vec![0u8; 4];
            word_pad.extend(word);
            word_pad.extend([0u8; 4]);
            let w1 = extract_num(
                array_windows(&word_pad)
                    .find(|w| extract_num(w) != 10)
                    .expect("to find one number"),
            );
            let w2 = extract_num_back(
                array_windows(&word_pad)
                    .rfind(|w| extract_num_back(w) != 10)
                    .unwrap(),
            );
            (w1 * 10 + w2) as i32
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let part1 = parse_input_1(&input);
    let part2 = parse_input_2(&input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
