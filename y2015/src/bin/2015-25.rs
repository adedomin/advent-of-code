use aoc_shared::{read_input_to_string, try_atoi, FlatVec2D};
use std::{borrow::BorrowMut, collections::HashMap, io};

type Output = (usize, usize);
type Solved = i64;

fn parse_input(input: &str) -> Output {
    let i = input
        .split_terminator(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
        .filter_map(|num| {
            if num.is_empty() {
                None
            } else {
                try_atoi::<_, 10>(num.as_bytes())
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(i.len(), 2);
    (i[0], i[1])
}

const START: i64 = 20151125;

fn part1_sol((x, y): (usize, usize)) -> Solved {
    let mut manual_code_page = HashMap::new();
    manual_code_page.insert((0, 0), START);
    let mut r = 1usize;
    let mut c = 0usize;
    loop {
        let last = if c == 0 { (0, r - 1) } else { (r + 1, c - 1) };
        let last_num = manual_code_page
            .get(&last)
            .expect("Expected to be visited.");
        manual_code_page.insert((r, c), last_num * 252533 % 33554393);
        if (r, c) == (x - 1, y - 1) {
            break *manual_code_page
                .get(&(x - 1, y - 1))
                .expect("Should be set.");
        }
        match r.checked_sub(1) {
            Some(rn) => {
                r = rn;
                c += 1;
            }
            None => {
                r = c + 1;
                c = 0;
            }
        }
    }
}

// fn part2_sol(input: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(parsed_input);
    // let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    // print!("Part2: {part2}");
    println!();
    Ok(())
}
