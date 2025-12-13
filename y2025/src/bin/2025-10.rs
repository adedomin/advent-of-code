use std::io;

use aoc_shared::{Dijkstra, HeapState, read_input_to_string};
use z3::ast::Int;

#[derive(Debug)]
struct Machine {
    ind: u64,
    btn: Vec<u64>,
    jlt: Vec<u64>,
}

const TWO: u64 = 2;

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
                .enumerate()
                .fold(0, |acc, (pos, c)| {
                    acc | if *c == b'#' { TWO.pow(pos as u32) } else { 0 }
                });
            let mut btn = items
                .map(|x| {
                    x.split(['(', ')', '{', '}', ','])
                        .flat_map(|n| n.parse::<u64>().ok())
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<_>>();
            let jlt = btn.pop().expect("at least one btn group...");
            let btn = btn
                .into_iter()
                .map(|btns| btns.into_iter().fold(0, |acc, n| acc | TWO.pow(n as u32)))
                .collect::<Vec<_>>();
            Machine { ind, btn, jlt }
        })
        .collect::<Vec<_>>()
}

fn least_btn_for_patt(target: u64, btns: &[u64]) -> u64 {
    let mut heap = Dijkstra::new();
    heap.push(0, 0);
    while let Some(HeapState { key, cost }) = heap.pop() {
        if key == target {
            return cost;
        }
        btns.iter().for_each(|btn| {
            let mut key = key;
            key ^= btn;
            heap.push(key, cost + 1);
        });
    }
    0
}

fn solve(ms: &[Machine]) -> u64 {
    ms.iter().map(|m| least_btn_for_patt(m.ind, &m.btn)).sum()
}

fn solve2(ms: &[Machine]) -> u64 {
    let mut ans = 0;
    for m in ms {
        let optim = z3::Optimize::new();
        let vars = (0..m.btn.len())
            .map(|i| Int::fresh_const(&format!("x{i}")))
            .collect::<Vec<_>>();
        vars.iter().for_each(|v| optim.assert(&v.ge(0)));
        // vars.iter().for_each(|v| optim.minimize(v));

        m.jlt.iter().enumerate().for_each(|(i, jolt)| {
            let bit = 1 << i;
            let tot = m
                .btn
                .iter()
                .enumerate()
                .filter(|(_, btn)| *btn & bit > 0)
                .map(|(i, _)| &vars[i])
                .sum::<Int>();
            optim.assert(&tot.eq(*jolt));
        });
        let min = vars.iter().sum::<Int>();
        optim.minimize(&min);
        #[cfg(debug_assertions)]
        {
            println!("{optim}");
        }
        if matches!(optim.check(&[]), z3::SatResult::Sat) {
            let model = optim.get_model().expect("unsat");
            #[cfg(debug_assertions)]
            {
                println!("RES:");
                vars.iter().enumerate().for_each(|(i, v)| {
                    println!(
                        "btn {i}: {}",
                        model.eval(v, true).unwrap().as_u64().unwrap()
                    );
                });
                println!("----------------");
            }
            ans += model.eval(&min, true).unwrap().as_u64().unwrap();
        } else {
            panic!("UNSAT");
        }
    }
    ans
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let input = parse_input(input.trim());
    let part1 = solve(&input);
    let part2 = solve2(&input);
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
