use aoc_shared::{array_windows, fold_decimal_from, read_input_to_string};
use std::io;

type Output = Vec<Vec<Solved>>;
type Solved = i32;

fn parse_input(input: &str) -> Output {
    input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(
                    line.split_ascii_whitespace()
                        .map(|num| fold_decimal_from(num.as_bytes()))
                        .collect::<Vec<_>>(),
                )
            }
        })
        .collect::<Output>()
}

fn safe_distance(l: Solved, r: Solved, sign: Solved) -> bool {
    (1..4).contains(&((r - l) * sign))
}

fn is_safe(report: &[i32], part2: bool) -> bool {
    let sign = if report.len() > 1 {
        (report[1] - report[0]).signum()
    } else {
        panic!("Invalid report, must have at least 2 numbers");
    };
    let itr = array_windows(report).position(|&[l, r]| !safe_distance(l, r, sign));
    match itr {
        // note, array_windows effectively shrinks max len by 1 so i+1 is always safe, unlike 0 - 1
        Some(i) if part2 => if i == 0 { i..i + 2 } else { i - 1..i + 2 }.any(|i| {
            let (l, r) = report.split_at(i);
            let r = &r[1..];
            if r.is_empty() {
                true
            } else {
                let sign = l
                    .iter()
                    .chain(r.iter())
                    .take(2)
                    .copied()
                    .reduce(|l, r| r - l)
                    .expect("Two numbers should be in this report???")
                    .signum();
                let check_break = l.last().is_none_or(|&ll| safe_distance(ll, r[0], sign));
                let rem = array_windows(r).all(|&[l, r]| safe_distance(l, r, sign));
                check_break && rem
            }
        }),
        Some(_) => false,
        None => true,
    }
}

fn part1_sol(input: Output) -> (Solved, Solved) {
    input.into_iter().fold((0, 0), |(p1, p2), report| {
        let p1t = is_safe(&report, false);
        let p2t = is_safe(&report, true);
        (p1 + p1t as Solved, p2 + p2t as Solved)
    })
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let (part1, part2) = part1_sol(parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
