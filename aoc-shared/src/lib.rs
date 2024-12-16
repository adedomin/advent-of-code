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
mod tokenizer;
pub use tokenizer::*;
mod flat2d;
pub use flat2d::*;
mod atoi;
pub use atoi::*;
mod pop_if;
pub use pop_if::pop_if;
mod dijkstra;
pub use dijkstra::*;

use std::{
    collections::HashMap,
    env, fs,
    io::{self, Read, Write},
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

pub fn advanced_cli() -> (Vec<u8>, Option<impl Write>, HashMap<String, String>) {
    let (r, w, o) = env::args().skip(1).fold(
        (None, None, HashMap::new()),
        |(read, write, mut opt), arg| {
            let eq_at = arg.find('=');
            if let Some(eq) = eq_at {
                let (k, v) = arg.split_at(eq);
                let v = &v[1..];
                match k {
                    "i" | "input" => (
                        Some(fs::read(v).expect("Expected to be able to open input.")),
                        write,
                        opt,
                    ),
                    "o" | "output" => (
                        read,
                        Some(
                            std::fs::File::create(v).expect("Expected to be able to open output."),
                        ),
                        opt,
                    ),
                    _ => {
                        opt.insert(k.to_owned(), v.to_owned());
                        (read, write, opt)
                    }
                }
            } else {
                (
                    Some(fs::read(arg).expect("Expected to be able to open input.")),
                    write,
                    opt,
                )
            }
        },
    );
    if let Some(read) = r {
        (read, w, o)
    } else {
        let mut buf = vec![];
        io::stdin()
            .lock()
            .read_to_end(&mut buf)
            .expect("Expected to read STDIN.");
        (buf, w, o)
    }
}

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
