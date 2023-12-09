use aoc_shared::{array_windows, atoi, read_input, Token, Tokenize};
use itertools::Itertools;
use std::io;

type Output = Vec<Vec<i64>>;

fn parse_input(input: &[u8]) -> Output {
    input
        .tokenize()
        .fold(
            (vec![], vec![], 1),
            |(mut acc, mut line, is_neg), tok| match tok {
                Token::Something(num) => {
                    line.push(is_neg * atoi::<i64, 10>(num));
                    (acc, line, 1)
                }
                Token::Delimiter(b'-') => (acc, line, -1),
                Token::Newline | Token::DoubleNewline | Token::End if !line.is_empty() => {
                    acc.push(line);
                    (acc, vec![], 1)
                }
                _ => (acc, line, 1),
            },
        )
        .0
}

fn binomial(n: i64, k: i64) -> i64 {
    fn real_binom(acc: i64, acc2: i64, n: i64, k: i64) -> i64 {
        match (acc, acc2, n, k) {
            (a1, a2, _, 0) => a1 / a2,
            (_, _, 0, _) => 0,
            (a1, a2, n, k) => real_binom(a1 * n, a2 * k, n - 1, k - 1),
        }
    }
    real_binom(1, 1, n, if k > n / 2 { n - k } else { k })
}

struct Triangles<T: num::Integer + Clone> {
    last: Vec<T>,
    degree: i64,
    first: bool,
}

impl<T: num::Integer + Clone + Copy> From<&[T]> for Triangles<T> {
    fn from(value: &[T]) -> Self {
        Triangles {
            last: value.to_owned(),
            degree: 0,
            first: true,
        }
    }
}

impl<T: num::Integer + Clone + Copy> Iterator for Triangles<T> {
    type Item = (i64, Vec<T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.last.iter().all_equal() {
            None
        } else if self.first {
            self.first = false;
            Some((self.degree, self.last.clone()))
        } else {
            self.last = array_windows(&self.last)
                .map(|&[l, r]| r - l)
                .collect::<Vec<_>>();
            self.degree += 1;
            Some((self.degree, self.last.clone()))
        }
    }
}

fn part1_sol(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|nums| {
            Triangles::from(&nums[..])
                .map(|(degree, tri)| tri[0] * binomial(nums.len() as i64, degree))
                .sum::<i64>()
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part1_sol(
        &parsed_input
            .into_iter()
            .map(|line| line.into_iter().rev().collect_vec())
            .collect_vec()[..],
    );
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
