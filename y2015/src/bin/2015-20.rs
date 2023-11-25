use aoc_shared::{atoi, read_input};
use std::io;

fn primes_to(n: i64) -> Vec<i64> {
    let n = n as usize;
    if n < 2 {
        Vec::new()
    } else {
        let mut is_prime = vec![true; n - 1];
        let limit = f64::from(n as i32).sqrt() as usize;
        for i in 2..limit + 1 {
            let mut it = is_prime[i - 2..].iter_mut().step_by(i);
            if let Some(true) = it.next() {
                it.for_each(|x| *x = false);
            }
        }
        is_prime
    }
    .iter()
    .enumerate()
    .filter_map(|(e, &b)| if b { Some((e + 2) as i64) } else { None })
    .collect::<Vec<i64>>()
}

fn sigma_from(p: &[(i64, u32)]) -> Option<i64> {
    p.iter().try_fold(1, |acc: i64, (prime, power)| {
        if *power == 0 {
            Some(acc)
        } else {
            let to_pow = prime.checked_pow(power + 1)?;
            acc.checked_mul((to_pow - 1) / (prime - 1))
        }
    })
}

fn into_num(p: &[(i64, u32)]) -> Option<i64> {
    p.iter().try_fold(1, |acc, (prime, power)| {
        let to_pow = prime.checked_pow(*power)?;
        Some(acc * to_pow)
    })
}

// here, we're trying to find the lowest possible powers of the given primes from below.
fn find_lowest(sigma_n: i64, pfact: &[i64]) -> Option<i64> {
    let mut parts = pfact
        .iter()
        .map(|&p| (p, 0u32))
        .collect::<Vec<(i64, u32)>>();
    let mut min = None;

    // impl of stars and bars solver: K ingredients of N grams.
    let mut loop_ctrs = Vec::with_capacity(pfact.len());
    // 11 powers is a total random guess.
    loop_ctrs.push((0usize, 0u32, 11u32));
    while let Some((pos, cur, lim)) = loop_ctrs.pop() {
        parts[pos].1 = cur;
        if pos == parts.len() - 1 {
            if let Some(nsig) = sigma_from(&parts) {
                if nsig >= sigma_n {
                    if let Some(num) = into_num(&parts) {
                        if min.map_or(true, |m| num < m) {
                            min = Some(num);
                        }
                    }
                }
            }
            if cur < lim {
                loop_ctrs.push((pos, cur + 1, lim));
            }
        } else if cur < lim {
            loop_ctrs.push((pos, cur + 1, lim));
            loop_ctrs.push((pos + 1, 0, lim));
        }
    }

    min
}

// in this case, we're finding the minimum base case of p^a where sigma(p^a) >= target sigma.
fn find_lowest_that_exceeds(sigma_n: i64, primes: &[i64]) -> i64 {
    let mut piter = primes.iter();
    let mut prime_list = vec![];
    let mut sigma_v = 1;
    while sigma_v < sigma_n {
        let &p = piter.next().expect("not enough primes");
        prime_list.push(p);
        sigma_v *= (p.pow(2) - 1) / (p - 1);
    }

    find_lowest(sigma_n, &prime_list).expect("to find lowest number")
}

// unfortunately, part2 is no longer based on sigma, so brand new.
// ... we're just going to build someing big and yolo it.
fn find_lowest_given_new_cond(input: i64) -> usize {
    let mut homes = vec![0i64; input as usize / 11];
    for elf in 1..homes.len() {
        homes
            .iter_mut()
            .skip(elf - 1)
            .step_by(elf)
            .take(50)
            .for_each(|home| {
                *home += elf as i64 * 11;
            })
    }
    // 1 based indexing... duh.
    homes
        .iter()
        .position(|&h| h >= input as i64)
        .expect("at least one home to have {input} amount of presents")
        + 1
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let input: i64 = atoi::<i64, 10>(&input);
    let primes = primes_to(input);

    // let part1 = solve_p1(input);
    // let part1 = find_lowest_sigma(input / 10, &primes);
    // assert_eq!(sigma(part1, &primes), input / 10);
    let part1 = find_lowest_that_exceeds(input / 10, &primes);
    print!("Part1 {part1}, ");
    let part2 = find_lowest_given_new_cond(input);
    println!("Part2: {part2}");
    Ok(())
}
