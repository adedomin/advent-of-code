use std::io;

use aoc_shared::read_input_to_string;

fn parse_input(i: &str) -> Vec<Vec<u8>> {
    i.split_ascii_whitespace()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| {
                    let res = b - b'0';
                    assert!(
                        (1..=9).contains(&res),
                        "battery value must be between 1 and 9"
                    );
                    res
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>()
}

// fn solve(arr: &[Vec<u8>]) -> u32 {
//     arr.iter()
//         .map(|bank| {
//             assert!(
//                 bank.len() > 1,
//                 "battery bank too small! must be greater than 1."
//             );
//             bank[2..]
//                 .iter()
//                 .fold(u32::from(bank[0] * 10 + bank[1]), |acc, &batt| {
//                     let rep_hi = acc % 10 * 10 + batt as u32;
//                     let rep_lo = acc / 10 * 10 + batt as u32;
//                     acc.max(rep_hi).max(rep_lo)
//                 })
//         })
//         .sum()
// }

fn solve2<const N: usize>(arr: &[Vec<u8>]) -> u64 {
    let mut ans: Vec<u8> = Vec::with_capacity(N);
    arr.iter()
        .map(|bank| {
            assert!(bank.len() >= N, "battery bank too small!");
            ans.clear();
            bank.iter().enumerate().for_each(|(i, &batt)| {
                while bank.len() + ans.len() - i > N && ans.pop_if(|val| *val < batt).is_some() {}
                if ans.len() < N {
                    ans.push(batt);
                }
            });
            ans.iter().fold(0, |acc, &n| acc * 10 + n as u64)
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let input = parse_input(input.trim());
    let part1 = solve2::<2>(&input);
    let part2 = solve2::<12>(&input);
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
