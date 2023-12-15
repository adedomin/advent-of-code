use std::io;

use aoc_shared::{array_windows, read_input, AoCTokenizer, Token};

const ALPHAMAX: usize = 26;
const ALPHADIM: usize = 26 * 26;

fn pair_to_num(pair: (u8, u8)) -> usize {
    (pair.0 - b'A') as usize * ALPHAMAX + (pair.1 - b'A') as usize
}

// fn num_to_str(pair: usize) -> String {
//     let p1 = (pair / 26) as u8 + b'A';
//     let p2 = (pair % 26) as u8 + b'A';
//     String::from_utf8_lossy(&[p1, p2]).to_string()
// }

fn parse(input: Vec<u8>) -> ([u64; ALPHADIM], [(usize, usize); ALPHADIM], u8) {
    let (start_cond, rules) = input.split_at(input.iter().position(|&chr| chr == b'\n').unwrap());

    let mut vector_hist = [0u64; ALPHADIM];
    array_windows(start_cond).for_each(|&[st, ed]| {
        vector_hist[pair_to_num((st, ed))] += 1;
    });

    let mut last_pair = (0, 0);
    // if default tuples get accessed, it should go out of bounds.
    let mut prod_rules = [(ALPHADIM, ALPHADIM); ALPHADIM];
    AoCTokenizer::new(rules).for_each(|token| match token {
        Token::Something(pair) if pair.len() == 2 => {
            last_pair.0 = pair[0];
            last_pair.1 = pair[1];
        }
        Token::Something(prod) if prod.len() == 1 => {
            let prod = prod[0];
            prod_rules[pair_to_num(last_pair)] = (
                pair_to_num((last_pair.0, prod)),
                pair_to_num((prod, last_pair.1)),
            );
        }
        _ => (),
    });
    (vector_hist, prod_rules, start_cond.last().unwrap() - b'A')
}

fn solve_for_x_rounds(
    input_histogram: &[u64; ALPHADIM],
    product_rules: &[(usize, usize); ALPHADIM],
    last: u8,
    runs: u64,
) -> u64 {
    let mut hist = *input_histogram;
    for _ in 0..runs {
        hist = hist
            .iter()
            .enumerate()
            .filter(|(_, v)| **v != 0)
            .map(|(idx, &v)| (product_rules[idx], v))
            .fold([0u64; ALPHADIM], |mut acc, ((p1, p2), v)| {
                acc[p1] += v;
                acc[p2] += v;
                acc
            });
    }

    let mut alpha_hist = hist
        .iter()
        .enumerate()
        .fold([0u64; ALPHAMAX], |mut acc, (idx, cnt)| {
            acc[idx / 26] += cnt;
            acc
        });
    alpha_hist[last as usize] += 1;
    alpha_hist.sort_unstable();
    let (max, min) = alpha_hist.iter().filter(|&&cnt| cnt != 0).fold(
        (u64::MIN, u64::MAX),
        |(max, min), &val| {
            if max < val && min > val {
                (val, val)
            } else if max < val {
                (val, min)
            } else if min > val {
                (max, val)
            } else {
                (max, min)
            }
        },
    );
    max - min
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let (input_hist, rules, last) = parse(input);
    let p1 = solve_for_x_rounds(&input_hist, &rules, last, 10);
    let p2 = solve_for_x_rounds(&input_hist, &rules, last, 40);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
