use aoc_shared::{destructure_or_none, fold_decimal_from, read_input, AoCTokenizer, Token};
use itertools::Itertools;
use std::{
    fmt::{Display, Write},
    io,
};

#[derive(Clone, Copy)]
struct Instructions(usize, usize, usize);
#[derive(Debug, Clone)]
struct Crates(pub Vec<Vec<u8>>);

impl Display for Crates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_stack = self.0.iter().max_by(|a, b| a.len().cmp(&b.len()));
        if let Some(max_stack) = max_stack {
            let max_stack = max_stack.len() - 1;
            let mut i = max_stack as isize;
            while i > -1 {
                for stack in &self.0 {
                    if let Some(val) = stack.get(i as usize) {
                        f.write_fmt(format_args!("[{}] ", char::from(*val)))?
                    } else {
                        f.write_str("    ")?
                    }
                }
                f.write_char('\n')?;
                i -= 1;
            }
        }
        for (i, _) in self.0.iter().enumerate() {
            let i = i + 1;
            f.write_fmt(format_args!(" {i}  "))?
        }
        f.write_char('\n')?;
        Ok(())
    }
}

type Parsed = (Crates, Vec<Instructions>);

const ALPHA_START: u8 = b'A' - 1;

fn parse_input(input: Vec<u8>) -> Parsed {
    let mut tokenizer = AoCTokenizer::new(&input);
    let (mut crates, _) = tokenizer
        .by_ref()
        .take_while(|token| !matches!(token, Token::DoubleNewline))
        .fold(
            (Vec::new(), 0usize),
            |(mut acc, padding), token| match token {
                Token::Something(alphanum) if ALPHA_START < alphanum[0] => {
                    let idx = padding / 4;
                    while acc.len() < idx + 1 {
                        acc.push(Vec::new())
                    }
                    let crate_stack = acc.get_mut(idx).unwrap();
                    crate_stack.push(alphanum[0]);
                    (acc, padding + 1)
                }
                Token::Something(_) | Token::Delimiter(_) | Token::Space => (acc, padding + 1),
                Token::Newline => (acc, 0),
                Token::End => panic!("End of the input came before fully parsing crate positions."),
                Token::DoubleNewline => unreachable!(),
            },
        );

    crates.iter_mut().for_each(|stack| stack.reverse());

    let instructions = tokenizer
        .flat_map(|token| destructure_or_none!(Token::Something|word| = token))
        .filter(|token| !matches!(token, &b"move" | &b"from" | &b"to"))
        .tuples()
        .map(|(count, from, to)| {
            Instructions(
                fold_decimal_from(count),
                fold_decimal_from::<usize>(from) - 1usize,
                fold_decimal_from::<usize>(to) - 1usize,
            )
        })
        .collect();
    (Crates(crates), instructions)
}

fn move_crates(crates: &mut Crates, instructions: &[Instructions]) {
    for Instructions(count, from, to) in instructions.iter().copied() {
        for _ in 0..count {
            let tmp = {
                let from_stack = &mut crates.0[from];
                from_stack
                    .pop()
                    .unwrap_or_else(|| panic!("not enough crates to pop for stack {from}!"))
            };
            crates.0[to].push(tmp);
        }
    }
}

fn move_crates_9001(crates: &mut Crates, instructions: &[Instructions]) {
    for Instructions(count, from, to) in instructions.iter().copied() {
        let mut movable = Vec::new();
        for _ in 0..count {
            let from_stack = &mut crates.0[from];
            let tmp = from_stack
                .pop()
                .unwrap_or_else(|| panic!("not enough crates to pop for stack {from}!"));
            movable.push(tmp);
        }
        crates.0[to].extend(movable.iter().rev());
    }
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (mut crates, instructions) = parse_input(input);
    let mut crates_p2 = crates.clone();
    move_crates(&mut crates, &instructions);
    move_crates_9001(&mut crates_p2, &instructions);
    println!("Part1:\n\n{crates}\nPart2:\n\n{crates_p2}");
    Ok(())
}
