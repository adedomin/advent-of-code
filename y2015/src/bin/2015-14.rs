use aoc_shared::{fold_decimal, read_input};
use regex::bytes::Regex;
use std::io;

type Output<'a> = Vec<(&'a [u8], Piecewise)>;

#[derive(Debug)]
struct Piecewise {
    linear: i64,
    upto: i64,
    rest: i64,
}

impl Piecewise {
    fn eval(&self, sec: i64) -> i64 {
        let tot_sec = self.upto + self.rest;
        let linear_full = (sec / tot_sec) * self.upto * self.linear;
        let rest = (sec % tot_sec).min(self.upto);
        linear_full + (rest * self.linear)
    }
}

fn parse_input(input: &'_ [u8]) -> Output<'_> {
    let regex = Regex::new(r##"(?m)^(?<rein>[A-Za-z]+) can fly (?<linear>[[:digit:]]+) km/s for (?<upto>[[:digit:]]+) seconds, but then must rest for (?<rest>[[:digit:]]+) seconds.$"##)
            .unwrap();

    regex
        .captures_iter(input)
        .map(|cap| {
            let reindeer = cap.name("rein").expect("to get reindeer name").as_bytes();
            let linear = cap
                .name("linear")
                .expect("to get linear fn")
                .as_bytes()
                .iter()
                .fold(0, fold_decimal);
            let upto = cap
                .name("upto")
                .expect("to get upto sec")
                .as_bytes()
                .iter()
                .fold(0, fold_decimal);
            let rest = cap
                .name("rest")
                .expect("to get rest sec")
                .as_bytes()
                .iter()
                .fold(0, fold_decimal);
            (reindeer, Piecewise { linear, upto, rest })
        })
        .collect()
}

fn solve(r: &[(&[u8], Piecewise)], time: i64) -> i64 {
    println!("{}", r.len());
    r.iter()
        .map(|(_, p)| p.eval(time))
        .max()
        .expect("could not find a min")
}

fn solve_p2(r: &[(&[u8], Piecewise)], time: i64) -> i64 {
    let mut scores = Vec::with_capacity(r.len());

    for (_, pf) in r {
        scores.push((pf, 0));
    }

    // no point evaling 0, it's going to be zero for all of them.
    for i in 1..time {
        let curr_win_idx = scores
            .iter()
            .enumerate()
            .fold((i64::MIN, vec![]), |(topscore, mut acc), (pos, (pf, _))| {
                let eval = pf.eval(i);
                match topscore.cmp(&eval) {
                    std::cmp::Ordering::Less => (eval, vec![pos]),
                    std::cmp::Ordering::Equal => {
                        acc.push(pos);
                        (topscore, acc)
                    }
                    std::cmp::Ordering::Greater => (topscore, acc),
                }
            })
            .1;
        for winners in curr_win_idx {
            scores[winners].1 += 1;
        }
    }

    let last_pos = scores.len() - 1;
    scores
        .select_nth_unstable_by_key(last_pos, |(_, score)| *score)
        .1
         .1
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);

    let part1 = solve(&parsed_input, 2503);
    let part2 = solve_p2(&parsed_input, 2503);

    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
