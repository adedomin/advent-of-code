use std::io;

use y2021::read_input;

fn syn_score(delim: u8) -> i32 {
    if delim == b')' {
        3
    } else if delim == b']' {
        57
    } else if delim == b'}' {
        1197
    } else {
        /* if delim == b'>' */
        25137
    }
}

fn comp_score(delim: u8) -> i64 {
    if delim == b'(' {
        1
    } else if delim == b'[' {
        2
    } else if delim == b'{' {
        3
    } else {
        /* if delim == b'<' */
        4
    }
}

pub fn solve(input: Vec<u8>) -> (i32, i64) {
    let mut syntax_err = 0i32;
    let mut comp = vec![];
    let mut stack = vec![];
    let mut good_line = true;
    for token in input {
        match token {
            b'(' | b'[' | b'{' | b'<' if good_line => stack.push(token),
            b')' | b']' | b'}' | b'>' if good_line => {
                if let Some(pop) = stack.pop() {
                    if pop.abs_diff(token) != 2 && pop.abs_diff(token) != 1 {
                        syntax_err += syn_score(token);
                        good_line = false;
                    }
                } else {
                    syntax_err += syn_score(token);
                    good_line = false;
                }
            }
            b'\n' if good_line && !stack.is_empty() => {
                let mut cur_score = 0i64;
                while let Some(top) = stack.pop() {
                    cur_score = cur_score * 5 + comp_score(top);
                }
                comp.push(cur_score);
            }
            b'\n' => {
                stack.clear();
                good_line = true;
            }
            _ => (),
        }
    }
    let comp_half = comp.len() / 2;
    let (_, real_comp_score, _) = comp.select_nth_unstable(comp_half);
    (syntax_err, *real_comp_score)
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let (p1, p2) = solve(input);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
