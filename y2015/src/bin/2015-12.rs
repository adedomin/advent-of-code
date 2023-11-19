use aoc_shared::{atoi, read_input, AoCTokenizer, Token};
use std::{fmt::Debug, io, ops::Add};

fn parse_and_solve_p1(input: &[u8]) -> i64 {
    let (ans, _, _) =
        AoCTokenizer::new(input).fold((0i64, 1, false), |(acc, sign, in_str), token| match token {
            Token::Something(word) if !in_str => {
                let parsed = sign * atoi::<i64, 10>(word);
                let new_acc = acc + parsed;
                (new_acc, 1, in_str)
            }
            Token::Delimiter(delim) if delim == b'-' && !in_str => (acc, -1, in_str),
            Token::Delimiter(delim) if delim == b'"' && in_str => (acc, sign, false),
            Token::Delimiter(delim) if delim == b'"' && !in_str => (acc, sign, true),
            _ => (acc, sign, in_str),
        });
    ans
}

enum Obj {
    Red,
    Sum(i64),
}

impl Debug for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Obj::Red => f.write_str("Red"),
            Obj::Sum(val) => f.write_fmt(format_args!("{val}")),
        }
    }
}

impl Default for Obj {
    fn default() -> Self {
        Self::Sum(0)
    }
}

impl Add<Obj> for Obj {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Obj::Red, _) => Obj::Red,
            (_, Obj::Red) => Obj::Red,
            (Obj::Sum(lhs), Obj::Sum(rhs)) => Obj::Sum(lhs + rhs),
        }
    }
}

impl<'a> Add<&'a Obj> for Obj {
    type Output = Obj;

    fn add(self, rhs: &'a Obj) -> Self::Output {
        match (self, rhs) {
            (Obj::Red, _) => Obj::Red,
            (_, Obj::Red) => Obj::Red,
            (Obj::Sum(lhs), Obj::Sum(rhs)) => Obj::Sum(lhs + rhs),
        }
    }
}

// same as first, but with more annoyances.
fn parse_and_solve_p2(input: &[u8]) -> i64 {
    let objsum = AoCTokenizer::new(input)
        .fold(
            (Vec::new(), 1, false, false, Obj::default()),
            |(mut stack, sign, in_str, in_prop, objsum), token| match token {
                Token::Something(word) if !in_str => {
                    let parsed = Obj::Sum(sign * atoi::<i64, 10>(word));
                    (stack, 1, in_str, false, objsum + parsed)
                }
                Token::Something(word) if in_prop => {
                    let new_obj = if word == b"red" { Obj::Red } else { objsum };
                    (stack, sign, in_str, false, new_obj)
                }
                Token::Delimiter(delim) if delim == b'-' && !in_str => {
                    (stack, -1, in_str, false, objsum)
                }
                Token::Delimiter(delim) if delim == b'"' && in_str => {
                    (stack, sign, false, false, objsum)
                }
                // This is the only case, other than b':", where we preserve
                // if it's an object property.
                Token::Delimiter(delim) if delim == b'"' && !in_str => {
                    (stack, sign, true, in_prop, objsum)
                }
                // Who cares about arrays? "red" the value only matters
                // if it's an object property. so ya.
                Token::Delimiter(delim) if delim == b':' && !in_str => {
                    (stack, sign, in_str, true, objsum)
                }
                Token::Delimiter(delim) if delim == b'{' && !in_str => {
                    stack.push(objsum);
                    (stack, sign, in_str, false, Obj::default())
                }
                Token::Delimiter(delim) if delim == b'}' && !in_str => {
                    let old_obj = stack.pop().expect("expected at least one object on stack");
                    let objsum = match objsum {
                        Obj::Red => Obj::default(),
                        Obj::Sum(_) => objsum,
                    } + old_obj;
                    (stack, sign, in_str, false, objsum)
                }
                Token::End => {
                    if !stack.is_empty() {
                        panic!("Invalid json. Not enough closing brackets.")
                    }
                    (stack, sign, in_str, false, objsum)
                }
                _ => (stack, sign, in_str, false, objsum),
            },
        )
        .4;
    match objsum {
        Obj::Red => 0,
        Obj::Sum(val) => val,
    }
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let part1 = parse_and_solve_p1(&input);
    let part2 = parse_and_solve_p2(&input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
