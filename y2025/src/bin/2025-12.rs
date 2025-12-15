use std::io;

use aoc_shared::read_input_to_string;

fn parse_solve(input: &str) -> usize {
    input
        .split('\n')
        .rev()
        .take_while(|line| !line.is_empty())
        .filter(|line| {
            let mut iter = line.split([':', ' ']);
            let (x, y) = iter
                .next()
                .expect("counts: AxB")
                .split_once('x')
                .expect("AxB");
            let x = x.parse::<u32>().expect("valid num");
            let y = y.parse::<u32>().expect("valid num");
            let areas = iter.flat_map(|n| n.parse::<u32>().ok()).sum::<u32>();
            x * y / 8 > areas
        })
        .count()
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let part1 = parse_solve(input.trim());
    println!("Part1 {part1}");
    Ok(())
}
