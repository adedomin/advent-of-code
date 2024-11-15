use aoc_shared::{read_input, try_atoi, GroupTokenize, Token};
use std::{collections::HashMap, io};

type Output = Vec<(Ident, Vec<(Ident, usize)>)>;
type Solved = usize;

type Ident = usize;

fn parse_input(input: &[u8]) -> (Output, Ident) {
    let mut ident_map = HashMap::new();
    let mut lines = input
        .group_tokens(Token::Newline)
        .filter(|group| !group.is_empty())
        .map(|group| {
            let mut sp = group.splitn(2, |tok| matches!(tok, Token::Something(b"contain")));
            let ident = sp.next().expect("Expected at least one ident.");
            let production = sp
                .next()
                .expect("production rules missing; no 'contain' token?");
            let mut k_ident = vec![];
            ident
                .iter()
                .filter_map(|tok| match tok {
                    Token::Something(w) if w != b"bags" => Some(*w),
                    _ => None,
                })
                .for_each(|tok| k_ident.extend_from_slice(tok));
            let ident = {
                let icnt = ident_map.len();
                *ident_map.entry(k_ident).or_insert(icnt)
            };

            let production = production
                .split(|tok| matches!(tok, Token::Delimiter(b',')))
                .filter_map(|tokgrp| {
                    let mut toks = tokgrp.iter().filter_map(|tok| match tok {
                        Token::Something(w) if w != b"bags" && w != b"bag" => Some(*w),
                        _ => None,
                    });
                    let cnt = toks
                        .next()
                        .expect("Expected at least one value after contain.");
                    if cnt == b"no" {
                        return None;
                    }
                    let cnt = match try_atoi::<usize, 10>(cnt) {
                        Some(num) => num,
                        None => panic!("Invalid production rule, no count!"),
                    };

                    let mut k_ident = vec![];
                    toks.for_each(|t| k_ident.extend_from_slice(t));
                    let ident = {
                        let icnt = ident_map.len();
                        *ident_map.entry(k_ident).or_insert(icnt)
                    };
                    Some((ident, cnt))
                })
                .collect::<Vec<_>>();
            (ident, production)
        })
        .collect::<Vec<_>>();

    lines.sort_by_key(|(k, _)| *k);
    #[cfg(debug_assertions)]
    {
        ident_map.iter().for_each(|(k, v)| {
            println!("{}, ident:{v}", std::str::from_utf8(k).unwrap());
        });
        lines
            .iter()
            .for_each(|(i, rules)| println!("{i} {rules:?}"));
    }
    (
        lines,
        *ident_map
            .get(&b"shinygold"[..])
            .expect("Did not find a shiny gold bag in any of the rules."),
    )
}
fn part1_sol(input: &Output, bag: Ident) -> Solved {
    fn rec(input: &Output, bag: Ident, curr: Ident) -> bool {
        input[curr]
            .1
            .iter()
            .any(|(i, _)| *i == bag || rec(input, bag, *i))
    }
    input
        .iter()
        .filter(|(i, _)| {
            if *i != bag {
                rec(input, bag, *i)
            } else {
                false
            }
        })
        .count()
}

fn part2_sol(input: &Output, bag: Ident) -> Solved {
    fn rec(input: &Output, curr: Ident) -> Solved {
        input[curr].1.iter().fold(1, |acc, &(bagident, count)| {
            acc + rec(input, bagident) * count
        })
    }
    rec(input, bag) - 1
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (parsed_input, shinygold_ident) = parse_input(&input);
    let part1 = part1_sol(&parsed_input, shinygold_ident);
    let part2 = part2_sol(&parsed_input, shinygold_ident);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
