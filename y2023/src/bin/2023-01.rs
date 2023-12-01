use aoc_shared::{array_windows, atoi, destructure_or_none, read_input, Token, Tokenize};
use std::io;

fn parse_input_1(input: &[u8]) -> i32 {
    input
        .tokenize()
        .flat_map(|t| destructure_or_none!(Token::Something|word| = t))
        .map(|word| {
            let d1 = word
                .iter()
                .find(|d| d.is_ascii_digit())
                .expect("some lines do not have a digit");
            let d2 = word.iter().rfind(|d| d.is_ascii_digit()).unwrap();
            atoi::<i32, 10>(&[*d1, *d2])
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
        [b'1', _, _, _, _] => 1,
        [b'2', _, _, _, _] => 2,
        [b'3', _, _, _, _] => 3,
        [b'4', _, _, _, _] => 4,
        [b'5', _, _, _, _] => 5,
        [b'6', _, _, _, _] => 6,
        [b'7', _, _, _, _] => 7,
        [b'8', _, _, _, _] => 8,
        [b'9', _, _, _, _] => 9,
        [b'e', b'n', b'o', _, _] => 1,
        [b'o', b'w', b't', _, _] => 2,
        [b'e', b'e', b'r', b'h', b't'] => 3,
        [b'r', b'u', b'o', b'f', _] => 4,
        [b'e', b'v', b'i', b'f', _] => 5,
        [b'x', b'i', b's', _, _] => 6,
        [b'n', b'e', b'v', b'e', b's'] => 7,
        [b't', b'h', b'g', b'i', b'e'] => 8,
        [b'e', b'n', b'i', b'n', _] => 9,
        _ => 10,
    }
}
fn parse_input_2(input: &[u8]) -> i32 {
    input
        .tokenize()
        .flat_map(|t| destructure_or_none!(Token::Something|word| = t))
        .flat_map(|word| {
            let w1 = array_windows(word).find(|w| extract_num(w) != 10);
            if let Some(w1) = w1 {
                let w1 = extract_num(w1);
                let mut w2 = word.iter().rev().copied().collect::<Vec<u8>>();
                w2.extend([b'z'; 5].iter());
                let w2 = extract_num_back(
                    array_windows(&w2)
                        .find(|w| extract_num_back(w) != 10)
                        .unwrap(),
                );
                Some((w1 * 10 + w2) as i32)
            } else {
                None
            }
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    // let part1 = parse_input_1(&input);
    let part2 = parse_input_2(&input);
    println!("Part1: {{part1}}, Part2: {part2}");
    Ok(())
}
