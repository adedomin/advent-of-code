use aoc_shared::{read_input, Token, Tokenize};
use std::{collections::HashMap, io};

type Output = HashMap<[u8; 3], [[u8; 3]; 2]>;

const AAA: [u8; 3] = [b'A', b'A', b'A'];
const PRIMES_TO_1009: [u32; 169] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009,
];

fn factorize(n: u32) -> Vec<(usize, u32)> {
    let mut ret = [0u32; PRIMES_TO_1009.len()];
    let mut i = 0;
    let mut n = n;
    while n > 1 {
        if n % PRIMES_TO_1009[i] == 0 {
            n /= PRIMES_TO_1009[i];
            ret[i] += 1;
        } else {
            i += 1;
        }
    }

    ret.iter()
        .copied()
        .enumerate()
        .filter(|(_, cnt)| *cnt > 0)
        .collect::<Vec<_>>()
}

fn least_common_mult(factors: Vec<(usize, u32)>) -> u64 {
    let mut resi = [0u32; PRIMES_TO_1009.len()];

    for (prime, cnt) in factors {
        let curr = resi[prime as usize];
        resi[prime as usize] = std::cmp::max(cnt, curr);
    }

    resi.iter()
        .enumerate()
        .filter(|(_, &cnt)| cnt != 0)
        .map(|(p, pow)| {
            let prime = PRIMES_TO_1009[p];
            prime.pow(*pow) as u64
        })
        .product()
}

fn parse_input(input: &[u8]) -> (Vec<u8>, Output) {
    let mut itr = input.tokenize();
    let instructions = if let Token::Something(instructions) = itr.next().unwrap() {
        instructions
            .iter()
            .map(|chr| match chr {
                b'L' => 0,
                _ => 1,
            })
            .collect::<Vec<u8>>()
    } else {
        panic!("Invalid input");
    };

    let map = itr
        .fold(
            (HashMap::new(), [[0u8; 3]; 3], 0usize),
            |(mut acc, mut resi, last), token| match token {
                Token::Something(word) if last < 3 => {
                    resi[last] = word.try_into().unwrap();
                    (acc, resi, last + 1)
                }
                Token::Newline | Token::DoubleNewline if last == 3 => {
                    let (k, vs) = resi.split_at(1);
                    let k = k[0];
                    let left = vs[0];
                    let right = vs[1];
                    acc.insert(k, [left, right]);
                    (acc, [[0u8; 3]; 3], 0)
                }
                _ => (acc, resi, last),
            },
        )
        .0;

    (instructions, map)
}

fn solve(start: [u8; 3], instructions: &[u8], input: &Output) -> u32 {
    instructions
        .iter()
        .cycle()
        .try_fold((start, 0usize), |(acc, count), instr| {
            let instr = *instr as usize;
            let choice = input.get(&acc).unwrap()[instr];
            if choice[2] == b'Z' {
                Err(count + 1)
            } else {
                Ok((choice, count + 1))
            }
        })
        .unwrap_err() as u32
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (instructions, map) = parse_input(&input);
    let part1 = solve(AAA, &instructions, &map);
    let factored_p2 = map
        .iter()
        .filter(|(k, _)| k[2] == b'A')
        .map(|(k, _)| solve(*k, &instructions, &map))
        .flat_map(factorize)
        .collect::<Vec<_>>();
    let part2 = least_common_mult(factored_p2);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
