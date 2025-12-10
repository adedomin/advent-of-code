use std::io;

use aoc_shared::{Dijkstra, HeapState, read_input_to_string};

type Int = usize;

#[derive(Debug)]
struct Machine {
    ind: Vec<bool>,
    btn: Vec<Vec<Int>>,
    jlt: Vec<Int>,
}

fn parse_input(i: &str) -> Vec<Machine> {
    i.split('\n')
        .map(|line| {
            let mut items = line.split_ascii_whitespace();
            let indicators = items.next().expect("Need indicator: e.g. [...##.#.#.]");
            let ind = indicators
                .as_bytes()
                .iter()
                .skip(1)
                .take(indicators.len() - 2)
                .map(|c| *c == b'#')
                .collect::<Vec<bool>>();
            let mut btn = items
                .map(|x| {
                    x.split(['(', ')', '{', '}', ','])
                        .flat_map(|n| n.parse::<Int>().ok())
                        .collect::<Vec<Int>>()
                })
                .collect::<Vec<_>>();
            let jlt = btn.pop().expect("at least one btn group...");
            Machine { ind, btn, jlt }
        })
        .collect::<Vec<_>>()
}

fn solve(ms: &[Machine]) -> Int {
    let mut ans = 0;
    'out: for m in ms {
        let mut heap = Dijkstra::new();
        heap.push(vec![false; m.ind.len()], 0);
        while let Some(HeapState { key, cost }) = heap.pop() {
            if key == m.ind {
                ans += cost;
                continue 'out;
            }
            m.btn.iter().for_each(|btn| {
                let mut key = key.clone();
                btn.iter().for_each(|&i| key[i] = !key[i]);
                heap.push(key, cost + 1);
            });
        }
    }
    ans
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let input = parse_input(input.trim());
    let part1 = solve(&input);
    println!("Part1 {part1}");
    Ok(())
}
