use std::io;

use y2021::read_input;

// we don't care about invalid input, yolo
fn ascii_str_to_u64(input: &[u8]) -> i64 {
    input
        .iter()
        .take(20) // digits bigger than this should not exist (u64)
        .fold(0, |a, c| a * 10 + (c - b'0') as i64)
}

fn parse_and_solve(input: Vec<u8>) -> (i64, i64) {
    let (_, hor, aim, dep) = input
        .split(|&chr| chr == b' ' || chr == b'\n') /* perf says this is the devil */
        .filter(|&l| !l.is_empty())
        .fold((b'f', 0, 0, 0), |(dir, hor, aim, dep), tok| {
            let pre = unsafe { *tok.get_unchecked(0) };
            match pre {
                b'f' | b'd' | b'u' => (pre, hor, aim, dep),
                _ => {
                    let vec = ascii_str_to_u64(tok);
                    match dir {
                        b'f' => (dir, hor + vec, aim, dep + vec * aim),
                        b'd' => (dir, hor, aim + vec, dep),
                        b'u' => (dir, hor, aim - vec, dep),
                        _ => unreachable!(),
                    }
                }
            }
        });
    (hor * aim /* part 1 */, hor * dep /* part 2 */)
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let (p1, p2) = parse_and_solve(input);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
