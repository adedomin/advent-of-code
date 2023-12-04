use aoc_shared::{atoi, destructure_or_none, read_input, GroupTokenize, Token};
use std::io;

fn parse_input(input: &[u8]) -> Vec<(Vec<i32>, Vec<i32>)> {
    input
        .group_tokens(aoc_shared::Token::Newline)
        .map(|tokens| {
            let sep = tokens
                .iter()
                .position(|tok| matches!(tok, aoc_shared::Token::Delimiter(b'|')))
                .expect("expected to find divider \"|\".");
            let (winning, numbers) = tokens.split_at(sep);
            let winning = winning
                .iter()
                .skip_while(|tok| !matches!(tok, Token::Delimiter(b':')))
                .flat_map(|tok| destructure_or_none!(Token::Something|word| = tok))
                .map(|word| atoi::<i32, 10>(word))
                .collect::<Vec<i32>>();
            let numbers = numbers
                .iter()
                .skip(1)
                .flat_map(|tok| destructure_or_none!(Token::Something|word| = tok))
                .map(|word| atoi::<i32, 10>(word))
                .collect::<Vec<i32>>();
            (winning, numbers)
        })
        .collect::<Vec<(Vec<i32>, Vec<i32>)>>()
}

fn count_winning<'a>(i: &'a [(Vec<i32>, Vec<i32>)]) -> impl Iterator<Item = i32> + 'a {
    i.iter().map(|(win, nums)| {
        win.iter()
            .fold(0, |acc, w| if nums.contains(w) { acc + 1 } else { acc })
    })
}

fn solve(i: &[(Vec<i32>, Vec<i32>)]) -> i32 {
    count_winning(i)
        .filter(|cnt| *cnt > 0)
        .map(|cnt| 2i32.pow(cnt as u32 - 1))
        .sum()
}

fn solve2(i: &[(Vec<i32>, Vec<i32>)]) -> i32 {
    let winning_tables = count_winning(i).collect::<Vec<i32>>();
    let mut residue = vec![1i32; winning_tables.len()];
    let res_len = residue.len();

    winning_tables.iter().enumerate().for_each(|(pos, &win)| {
        let next = pos + 1;
        if win != 0 && next < residue.len() {
            let cur_res = residue[pos];
            let copy = next..std::cmp::min(res_len, next + (win as usize));
            residue[copy].iter_mut().for_each(|cell| *cell += cur_res);
        };
    });

    residue.iter().sum()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let part1 = solve(&parsed_input);
    let part2 = solve2(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
