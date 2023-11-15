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

mod array_window;
pub use array_window::*;

use std::{
    env, fs,
    io::{self, Read},
    ops::{Add, Index, IndexMut, Mul},
};

/// Helper to destructure enums like Token::Something
/// unfortunately Rust won't allow ( ) after a path capture
///
///  # Example
///
/// `destructure_or_none!(x::y::z|your, list, of, patterns| = val)`
#[macro_export]
macro_rules! destructure_or_none {
    ($name:path |$($parts:ident),* $(,)?| = $value:expr) => {
        if let $name($($parts)*) = $value {
            Some($($parts)*)
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($args:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!($($args)+)
        }
    };
}

pub struct FlatVec2D<T>(pub Vec<T>, pub usize, pub usize);

impl<T> FlatVec2D<T> {
    pub fn new(xdim: usize, ydim: usize) -> Self
    where
        T: Default + Clone,
    {
        FlatVec2D(vec![T::default(); xdim * ydim], xdim, ydim)
    }
}

impl<T> Index<(usize, usize)> for FlatVec2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.0[flat_coord(x, y, self.1)]
    }
}

impl<T> IndexMut<(usize, usize)> for FlatVec2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.0[flat_coord(x, y, self.1)]
    }
}

#[derive(Clone, Copy)]
pub enum Rot2D {
    None,
    Clock90,
    Clock180,
    Clock270,
}

pub fn flat_coord(x: usize, y: usize, dim: usize) -> usize {
    x + y * dim
}

pub fn flat_coord_rot(x: usize, y: usize, xdim: usize, ydim: usize, rot: Rot2D) -> usize {
    match rot {
        Rot2D::None => flat_coord(x, y, xdim),
        Rot2D::Clock90 => flat_coord((ydim - 1) - y, x, ydim),
        Rot2D::Clock180 => flat_coord((xdim - 1) - x, (ydim - 1) - y, xdim),
        Rot2D::Clock270 => flat_coord(y, (xdim - 1) - x, ydim),
    }
}

impl<T> Index<(usize, usize, Rot2D)> for FlatVec2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize, Rot2D)) -> &Self::Output {
        let (x, y, rot) = index;
        &self.0[flat_coord_rot(x, y, self.1, self.2, rot)]
    }
}

impl<T> IndexMut<(usize, usize, Rot2D)> for FlatVec2D<T> {
    fn index_mut(&mut self, index: (usize, usize, Rot2D)) -> &mut Self::Output {
        let (x, y, rot) = index;
        &mut self.0[flat_coord_rot(x, y, self.1, self.2, rot)]
    }
}

pub fn parse_to_flat2d<T>(input: &[u8]) -> FlatVec2D<T>
where
    T: Default + Clone + From<u8>,
{
    let row_width = input.iter().position(|&chr| chr == b'\n').unwrap();
    let col_len = ((input.len() - 1) / (row_width + 1)) + 1;

    let mut ret = FlatVec2D(vec![T::default(); row_width * col_len], row_width, col_len);

    let mut i = 0;
    let mut j = 0;
    input.iter().for_each(|&el| {
        if el == b'\n' {
            i = 0;
            j += 1;
        } else if el != b'\n' {
            ret[(i, j)] = el.into();
            i += 1;
        }
    });

    ret
}

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

/// simple type for mapping over or getting the default
pub enum Sentinel<T> {
    Unset(T),
    Value(T),
}

impl<T> Sentinel<T> {
    pub fn map<F: FnOnce(&T) -> T>(&self, fun: F) -> Sentinel<T> {
        match self {
            Sentinel::Unset(v) => Sentinel::Value(fun(v)),
            Sentinel::Value(v) => Sentinel::Value(fun(v)),
        }
    }

    pub fn map_mv<F: FnOnce(T)>(self, fun: F) {
        match self {
            Sentinel::Unset(v) => fun(v),
            Sentinel::Value(v) => fun(v),
        }
    }

    pub fn is_unset(&self) -> bool {
        matches!(self, Sentinel::Unset(_))
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

/// Intended to be used with: .iter().fold(num, fold_decimal)
pub fn fold_decimal_from<T>(number: &[u8]) -> T
where
    T: Copy + 'static,
    T: Add<Output = T>,
    T: Mul<Output = T>,
    u8: num::traits::AsPrimitive<T>,
{
    number.iter().fold(0.as_(), fold_decimal)
}

pub struct AtoiErr {
    radix: u8,
    value: u8,
    idx: usize,
}

impl std::fmt::Debug for AtoiErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for AtoiErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "value {}, parsed at idx {}, is too large for base {}.",
            self.value, self.idx, self.radix
        ))
    }
}

/// Function to convert a byte array into a integer of type T using a base of 2 through 36.
/// RADIX has to be between 2 and 36, or it will runtime panic.
/// Function returns None, if the bytes cannot be parsed into a correct number.
pub fn atoi<T, const RADIX: u8>(number: &[u8]) -> Result<T, AtoiErr>
where
    T: Copy + 'static,
    T: Add<Output = T>,
    T: Mul<Output = T>,
    u8: num::traits::AsPrimitive<T>,
{
    assert!(RADIX > 1 && RADIX < 37);
    number
        .iter()
        .enumerate()
        .try_fold(0.as_(), |acc, (pos, &chr)| {
            let val = (match chr {
                b'0'..=b'9' => chr,
                b'A'..=b'Z' => chr - 7, // b'9' to b'A' has 7 other chars between
                b'a'..=b'z' => chr - 39, // b'9' to b'a' has 39 other chars between it.
                _ => 255,               // junk.
            } - b'0');
            if RADIX <= val {
                Err(AtoiErr {
                    radix: RADIX,
                    value: val,
                    idx: pos,
                })
            } else {
                Ok(acc * RADIX.as_() + val.as_())
            }
        })
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token<'a> {
    Something(&'a [u8]),
    Delimiter(u8),
    Newline,
    DoubleNewline,
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

pub struct RecordGrouper<'a> {
    token_tmp: Vec<Token<'a>>,
    tokenizer: AoCTokenizer<'a>,
    record_sep: Token<'a>,
}

impl<'a> RecordGrouper<'a> {
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

impl<'a> Iterator for RecordGrouper<'a> {
    type Item = Vec<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        for token in self.tokenizer.by_ref() {
            if (token == self.record_sep || token == Token::End) && !self.token_tmp.is_empty() {
                return Some(std::mem::take(&mut self.token_tmp));
            }

            self.token_tmp.push(token);
        }

        None
    }
}
