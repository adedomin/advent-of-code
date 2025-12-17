use std::io;

use aoc_shared::read_input_to_string;

fn parse_solve(input: &str) -> usize {
    input
        .split('\n')
        .rev()
        .take_while(|line| !line.is_empty())
        .filter(|line| {
            let (dimensions, presents) = line.split_once(": ").expect("WxH: num1 num2 ...");
            let (w, h) = dimensions.split_once('x').expect("WxH");
            let w = w.parse::<u32>().expect("valid num: w");
            let h = h.parse::<u32>().expect("valid num: h");
            let sum_pres = presents
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().expect("valid num: presents"))
                .sum::<u32>();
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
