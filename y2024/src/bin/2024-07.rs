use aoc_shared::{fold_decimal_from, read_input_to_string};
use std::io;

type Int = u64;
type Output = Vec<(Int, Vec<Int>)>;

fn parse_input(input: &str) -> Output {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace().enumerate().map(|(pos, num)| {
                if pos == 0 {
                    let blen = num.as_bytes().len();
                    fold_decimal_from(&num.as_bytes()[..blen - 1])
                } else {
                    fold_decimal_from(num.as_bytes())
                }
            });
            let target = iter.next().unwrap();
            let values = iter.collect::<Vec<Int>>();
            (target, values)
        })
        .collect::<Output>()
}

fn test_calibration(target: Int, values: &[Int], part2: bool) -> bool {
    let mut stack = vec![(values[0], 1)];
    while let Some((acc, i)) = stack.pop() {
        if i == values.len() && acc == target {
            return true;
        } else if i == values.len() || acc > target {
            continue;
        }

        stack.push((acc + values[i], i + 1));
        stack.push((acc * values[i], i + 1));
        if part2 {
            let num_digits = 1 + values[i].ilog(10);
            let pow: Int = 10;
            stack.push((acc * pow.pow(num_digits) + values[i], i + 1));
        }
    }
    false
}

fn solve(input: &Output) -> (Int, Int) {
    input.iter().fold((0, 0), |(p1, p2), (target, values)| {
        if test_calibration(*target, values, false) {
            (p1 + *target, p2 + *target)
        } else if test_calibration(*target, values, true) {
            (p1, p2 + *target)
        } else {
            (p1, p2)
        }
    })
}

// fn part2_sol(input: &Output) -> Int {}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let (part1, part2) = solve(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
