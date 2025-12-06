use std::io;

use aoc_shared::read_input_to_string;

type Int = u64;

fn parse_opt(i: &str) -> Vec<impl Fn(Int, Int) -> Int> {
    i.split('\n')
        .rev()
        .skip_while(|line| line.is_empty())
        .take(1)
        .next()
        .map(|opstr| {
            opstr
                .split_ascii_whitespace()
                .map(|op| match op {
                    "+" => std::ops::Add::add,
                    "*" => std::ops::Mul::mul,
                    _ => panic!("Invalid Op: {op}"),
                })
                .collect::<Vec<_>>()
        })
        .expect("one line")
}

fn parse_p1(i: &str, oplen: usize) -> Vec<Vec<Int>> {
    let mut ret = vec![vec![]; oplen];
    i.split_ascii_whitespace()
        .flat_map(|s| s.parse::<Int>().ok())
        .enumerate()
        .for_each(|(i, n)| ret[i % oplen].push(n));
    ret
}

fn parse_p2(i: &str, oplen: usize) -> Vec<Vec<Int>> {
    let mut ret_i = 0;
    let mut ret = vec![vec![]; oplen];
    let mut grid = i
        .split('\n')
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();
    _ = grid.pop(); // remove last line
    assert!(!grid.is_empty(), "grid should have numbers.");
    let max_x = grid.iter().max_by_key(|line| line.len()).unwrap().len();
    for x in 0..max_x {
        let mut n = None;
        for y in 0..grid.len() {
            if let Some(g) = grid.get(y).and_then(|gl| gl.get(x))
                && g.is_ascii_digit()
            {
                n = Some(n.unwrap_or(0) * 10 + (g - b'0') as Int);
            }
        }
        if let Some(n) = n.take() {
            ret[ret_i].push(n);
        } else {
            // spacer
            ret_i += 1;
        }
    }
    ret
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let ops = parse_opt(&input);
    let [p1, p2] = [parse_p1(&input, ops.len()), parse_p2(&input, ops.len())].map(|p| {
        p.into_iter()
            .zip(ops.iter())
            .map(|(nums, op)| nums.into_iter().reduce(op).unwrap())
            .sum::<Int>()
    });
    println!("Part1 {p1}  Part2 {p2}");
    Ok(())
}
