use std::iter::FusedIterator;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token<'a> {
    Something(&'a [u8]),
    Delimiter(u8),
    Newline,
    DoubleNewline,
    Space,
    End,
}

pub struct AoCTokenizer<'a> {
    head: usize,
    done: bool,
    buffer: &'a [u8],
}
impl<'a> AoCTokenizer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        AoCTokenizer {
            head: 0,
            done: false,
            buffer: input,
        }
    }
}

impl<'a> From<&'a [u8]> for AoCTokenizer<'a> {
    fn from(data: &'a [u8]) -> Self {
        AoCTokenizer::new(data)
    }
}

impl<'a> Iterator for AoCTokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        } else if self.head >= self.buffer.len() {
            self.done = true;
            return Some(Token::End);
        }

        let start = self.head;
        self.head += 1;
        match self.buffer[start] {
            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => {
                let mut i = self.head;
                self.head = loop {
                    if i < self.buffer.len() {
                        match self.buffer[i] {
                            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' => (),
                            _ => break i,
                        }
                        i += 1;
                    } else {
                        break i;
                    }
                };
                Some(Token::Something(&self.buffer[start..self.head]))
            }
            b'\n' => {
                if self.head < self.buffer.len() && self.buffer[self.head] == b'\n' {
                    self.head += 1;
                    Some(Token::DoubleNewline)
                } else {
                    Some(Token::Newline)
                }
            }
            b' ' => Some(Token::Space),
            x => Some(Token::Delimiter(x)),
        }
    }
}

impl<'a> FusedIterator for AoCTokenizer<'a> {}

pub trait Tokenize<'a> {
    fn tokenize(self) -> AoCTokenizer<'a>;
}

impl<'a> Tokenize<'a> for &'a [u8] {
    fn tokenize(self) -> AoCTokenizer<'a> {
        AoCTokenizer::new(self)
    }
}

pub struct RecordGrouper<'a, T: Iterator<Item = Token<'a>>> {
    token_tmp: Vec<Token<'a>>,
    tokenizer: T,
    record_sep: Token<'a>,
}

impl<'a> RecordGrouper<'a, AoCTokenizer<'a>> {
    pub fn new(input: &'a [u8]) -> Self {
        RecordGrouper {
            token_tmp: vec![],
            tokenizer: AoCTokenizer::new(input),
            record_sep: Token::DoubleNewline,
        }
    }

    pub fn new_with_rs(input: &'a [u8], record_sep: Token<'a>) -> Self {
        RecordGrouper {
            token_tmp: vec![],
            tokenizer: AoCTokenizer::new(input),
            record_sep,
        }
    }
}

impl<'a, T: Iterator<Item = Token<'a>>> RecordGrouper<'a, T> {
    pub fn new_from_tokens_with_rs(tokens: T, record_sep: Token<'a>) -> Self {
        RecordGrouper {
            token_tmp: vec![],
            tokenizer: tokens,
            record_sep,
        }
    }
}

impl<'a, T: Iterator<Item = Token<'a>>> Iterator for RecordGrouper<'a, T> {
    type Item = Vec<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        for token in self.tokenizer.by_ref() {
            match (
                token == self.record_sep || token == Token::End,
                self.token_tmp.is_empty(),
            ) {
                (true, false) => return Some(std::mem::take(&mut self.token_tmp)),
                (false, _) => self.token_tmp.push(token),
                (true, true) => (), // skip empty records
            };
        }

        if !self.token_tmp.is_empty() {
            Some(std::mem::take(&mut self.token_tmp))
        } else {
            None
        }
    }
}

impl<'a, T: Iterator<Item = Token<'a>>> FusedIterator for RecordGrouper<'a, T> {}

pub trait GroupTokenize<'a> {
    fn group_tokens(self, separator: Token<'a>) -> RecordGrouper<'a, AoCTokenizer<'a>>;
}

impl<'a> GroupTokenize<'a> for &'a [u8] {
    fn group_tokens(self, separator: Token<'a>) -> RecordGrouper<'a, AoCTokenizer<'a>> {
        RecordGrouper::new_with_rs(self, separator)
    }
}

pub trait GroupTokens<'a, T: Iterator<Item = Token<'a>>> {
    fn group_tokens(self, separator: Token<'a>) -> RecordGrouper<'a, T>;
}

impl<'a, T: Iterator<Item = Token<'a>>> GroupTokens<'a, T> for T {
    fn group_tokens(self, separator: Token<'a>) -> RecordGrouper<'a, T> {
        RecordGrouper::new_from_tokens_with_rs(self, separator)
    }
}
