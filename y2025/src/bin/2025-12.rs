use std::io;

use aoc_shared::read_input_to_string;

fn parse_solve(input: &str) -> usize {
    input
        .split('\n')
        .rev()
        .take_while(|line| !line.is_empty())
        .filter(|line| {
            let mut iter = line.split([':', ' ']);
            let (w, h) = iter
                .next()
                .expect("counts: AxB")
                .split_once('x')
                .expect("AxB");
            let w = w.parse::<u32>().expect("valid num");
            let h = h.parse::<u32>().expect("valid num");
            let sum_pres = iter.flat_map(|n| n.parse::<u32>().ok()).sum::<u32>();
            w * h / 8 > sum_pres
        })
        .count()
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let part1 = parse_solve(input.trim());
    println!("Part1 {part1}");
    Ok(())
}
