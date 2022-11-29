// Copyright (C) 2021  Anthony DeDominic <adedomin@gmail.com>

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

// #![feature(exclusive_range_pattern)]

use std::{
    env, fs,
    io::{self, Read},
    ops::{Add, Mul},
};

pub type Vec2D<T> = Vec<Vec<T>>;

use num::cast::AsPrimitive;

pub fn read_input() -> io::Result<Vec<u8>> {
    match env::args().nth(1) {
        Some(arg) => fs::read(arg),
        None => {
            let mut buf = vec![];
            io::stdin().lock().read_to_end(&mut buf)?;
            Ok(buf)
        }
    }
}

pub fn read_input_to_string() -> io::Result<String> {
    match env::args().nth(1) {
        Some(arg) => fs::read_to_string(arg),
        None => {
            let mut buf = String::new();
            io::stdin().lock().read_to_string(&mut buf)?;
            Ok(buf)
        }
    }
}

pub struct AoCTokenizer<'a> {
    head: usize,
    done: bool,
    buffer: &'a [u8],
}

/// Intended to be used with: .iter().fold(num, fold_decimal)
pub fn fold_decimal<T>(acc: T, chr: &u8) -> T
where
    T: Copy + 'static,
    T: Add<Output = T>,
    T: Mul<Output = T>,
    u8: num::traits::AsPrimitive<T>,
{
    acc * 10.as_() + (chr - b'0').as_()
}

#[derive(Debug)]
pub enum Token<'a> {
    Something(&'a [u8]),
    Delimiter(u8),
    Newline,
    Space,
    End,
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
                let mut i = start + 1;
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
            b'\n' => Some(Token::Newline),
            b' ' => Some(Token::Space),
            x => Some(Token::Delimiter(x)),
        }
    }
}
