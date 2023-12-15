use aoc_shared::{debug, read_input, AoCTokenizer, Token};
use std::io;

type Output<'a> = Vec<NToken<'a>>;
type Solved = i64;

enum NToken<'a> {
    Quote,
    Escape(Escaped),
    UknkEsc,
    Char(u8),
    Chars(&'a [u8]),
    Delim,
}

enum Escaped {
    Quot,
    Esc,
    Hex(u8, u8),
}

trait CodeCharSz {
    fn code_size(&self) -> i64;
    fn mem_size(&self) -> i64;
    fn as_escaped_size(&self) -> i64;
}

impl<'a> CodeCharSz for NToken<'a> {
    fn code_size(&self) -> i64 {
        match self {
            NToken::Quote => 1,
            NToken::Escape(esc) => esc.code_size(),
            NToken::Char(_) => 1,
            NToken::Chars(chars) => chars.len() as i64,
            NToken::Delim => 0,
            NToken::UknkEsc => panic!("You shouldn't have this token"),
        }
    }

    fn mem_size(&self) -> i64 {
        match self {
            NToken::Quote => 0,
            NToken::Escape(esc) => esc.mem_size(),
            NToken::Char(_) => 1,
            NToken::Chars(chars) => chars.len() as i64,
            NToken::Delim => 0,
            NToken::UknkEsc => panic!("You shouldn't have this token"),
        }
    }

    fn as_escaped_size(&self) -> i64 {
        match self {
            NToken::Quote => 2,
            NToken::Escape(esc) => esc.as_escaped_size(),
            NToken::Char(_) => 1,
            NToken::Chars(chars) => chars.len() as i64,
            NToken::Delim => 2, // clever hack. since we are "requoting" the strings by escape, this represents the new wrapping "".
            NToken::UknkEsc => panic!("You shouldn't have this token,"),
        }
    }
}

impl CodeCharSz for Escaped {
    fn code_size(&self) -> i64 {
        match self {
            Escaped::Quot => 2,
            Escaped::Esc => 2,
            Escaped::Hex(_, _) => 4,
        }
    }

    fn mem_size(&self) -> i64 {
        match self {
            Escaped::Quot => 1,
            Escaped::Esc => 1,
            Escaped::Hex(_, _) => 1,
        }
    }

    fn as_escaped_size(&self) -> i64 {
        match self {
            Escaped::Quot => 4,      // \" -> \\\"
            Escaped::Esc => 4,       // \\ -> \\\\
            Escaped::Hex(_, _) => 5, // \xHH -> \\xHH
        }
    }
}

fn parse_input(input: &[u8]) -> Output {
    use Escaped::*;
    use NToken::*;
    let (tokens, _, _) = AoCTokenizer::new(input)
        .map(|token| match token {
            Token::Something(word) => Chars(word),
            Token::Delimiter(b'\\') => UknkEsc,
            Token::Delimiter(b'"') => Quote,
            Token::Delimiter(delim) => Char(delim),
            Token::Newline | Token::DoubleNewline | Token::Space | Token::End => Delim,
        })
        .fold(
            (Vec::new(), None, None),
            |(mut acc, esc, delim), tok| match tok {
                Quote if esc.is_some() => {
                    acc.push(Escape(Quot));
                    (acc, None, None)
                }
                Quote => {
                    acc.push(Quote);
                    (acc, None, None)
                }
                UknkEsc if esc.is_some() => {
                    acc.push(Escape(Esc));
                    (acc, None, None)
                }
                UknkEsc => (acc, Some(UknkEsc), None),
                Char(char) if esc.is_some() => {
                    // char is only for delimiters.
                    panic!("invalid escape {char:?}.")
                }
                Char(char) => {
                    acc.push(Char(char));
                    (acc, None, None)
                }
                Chars(chars) if esc.is_some() => {
                    if chars.len() < 3 {
                        panic!("Not enough characters for ascii escape.");
                    } else if chars[0] != b'x' {
                        panic!("Invalid ascii escape; must start with x.");
                    }
                    let (p1, p2) = chars.split_at(3);
                    acc.push(Escape(Hex(p1[1], p1[2])));
                    if !p2.is_empty() {
                        acc.push(Chars(p2));
                    }
                    (acc, None, None)
                }
                Chars(chars) => {
                    acc.push(Chars(chars));
                    (acc, None, None)
                }
                Delim if delim.is_none() => {
                    acc.push(Delim);
                    (acc, None, Some(Delim))
                }
                // join all repeating delims to one
                Delim => (acc, None, Some(Delim)),
                Escape(_) => unreachable!("Shouldn't have fully parsed escapes here."),
            },
        );
    tokens
}

fn part1_sol(input: &[NToken]) -> Solved {
    let code_size: i64 = input.iter().map(|tok| tok.code_size()).sum();
    let mem_size: i64 = input.iter().map(|tok| tok.mem_size()).sum();
    code_size - mem_size
}

fn part2_sol(input: &[NToken]) -> Solved {
    let double_esc_size: i64 = input.iter().map(|tok| tok.as_escaped_size()).sum();
    let code_size: i64 = input.iter().map(|tok| tok.code_size()).sum();
    debug!(
        "{double_esc_size} - {code_size} = {}",
        double_esc_size - code_size
    );
    double_esc_size - code_size
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);

    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input);
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
