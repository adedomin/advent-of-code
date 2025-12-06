use std::io;

use aoc_shared::read_input_to_string;

type Int = u64;

enum Op {
    Add,
    Mul,
}

fn parse_solve_1(i: &str) -> (Vec<(Int, Op)>, Int) {
    let mut itr = i.split('\n').rev();
    let operators = itr
        .next()
        .map(|opstr| {
            opstr
                .split_ascii_whitespace()
                .map(|op| match op {
                    "+" => (0, Op::Add),
                    "*" => (1, Op::Mul),
                    _ => panic!("invalid operator: {opstr}"),
                })
                .collect::<Vec<_>>()
        })
        .unwrap();
    let sum = itr
        .flat_map(|line| {
            line.split_ascii_whitespace()
                .enumerate()
                .map(|(i, num)| (i, num.parse::<Int>().expect("number")))
        })
        .fold(
            operators
                .iter()
                .map(|(start, _)| *start)
                .collect::<Vec<Int>>(),
            |mut acc, (i, n)| {
                acc[i] = match operators[i].1 {
                    Op::Add => acc[i] + n,
                    Op::Mul => acc[i] * n,
                };
                acc
            },
        )
        .into_iter()
        .sum();
    (operators, sum)
}

fn parse_solve_2(ops: Vec<(Int, Op)>, i: &str) -> Int {
    let mut nums_i = 0;
    let mut nums = vec![vec![]; ops.len()];
    let grid = i
        .split('\n')
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();
    // I think the numbers are properly padded...
    for x in 0..grid[0].len() {
        let mut num = None;
        for y in 0..grid.len() - 1 {
            let g = grid[y][x];
            if g.is_ascii_digit() {
                let g = (g - b'0') as Int;
                num = Some(num.unwrap_or(0) * 10 + g);
            }
        }
        if let Some(num) = num {
            nums[nums_i].push(num);
        } else {
            nums_i += 1;
        }
    }
    ops.into_iter()
        .zip(nums)
        .map(|((_, op), nums)| {
            nums.into_iter()
                .reduce(|acc, n| match op {
                    Op::Add => acc + n,
                    Op::Mul => acc * n,
                })
                .expect("one number")
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let input = input.trim();
    let (ops, part1) = parse_solve_1(input);
    let part2 = parse_solve_2(ops, input);
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
