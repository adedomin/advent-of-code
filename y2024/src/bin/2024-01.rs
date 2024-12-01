use aoc_shared::read_input_to_string;
use std::{collections::HashMap, io};

type Output = [Vec<u32>; 2];
type Solved = u32;

fn parse_input(input: &str) -> Output {
    let mut ret = [vec![], vec![]];
    input
        .split_ascii_whitespace()
        .enumerate()
        .for_each(|(i, num)| {
            let idx = i & 1;
            ret[idx].push(num.parse::<_>().expect("expected a number"));
        });
    ret
}

fn part1_sol([left, right]: &Output) -> Solved {
    let mut nleft = left.clone();
    nleft.sort_unstable();
    let mut nright = right.clone();
    nright.sort_unstable();
    nleft
        .into_iter()
        .zip(nright)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<Solved>()
}

fn part2_sol([left, right]: Output) -> Solved {
    let mut memo = HashMap::new();
    left.into_iter()
        .map(|num| {
            if let Some(v) = memo.get(&num) {
                num * v
            } else {
                let cnt = right.iter().filter(|&&n| n == num).count() as u32;
                memo.insert(num, cnt);
                num * cnt
            }
        })
        .sum::<Solved>()
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
