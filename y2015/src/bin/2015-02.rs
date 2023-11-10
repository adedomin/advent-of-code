use std::io;

use y2015::{fold_decimal_from, read_input};

#[derive(Debug)]
struct Present {
    pub l: i64,
    pub w: i64,
    pub h: i64,
}

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr),+) => (std::cmp::min($x, min!($($y),*)));
}

impl Present {
    fn surface_area(&self) -> i64 {
        (2 * self.l * self.w) + (2 * self.w * self.h) + (2 * self.h * self.l)
    }

    fn smallest_area(&self) -> i64 {
        min!(self.l * self.w, self.w * self.h, self.h * self.l)
    }

    fn volume(&self) -> i64 {
        self.l * self.w * self.h
    }

    fn smallest_perimeter(&self) -> i64 {
        let (s1, s2) =
            [self.l, self.w, self.h]
                .iter()
                .fold((i64::MAX, i64::MAX), |(s1, s2), &v| {
                    if v < s1 {
                        (v, s1)
                    } else if v < s2 {
                        (s1, v)
                    } else {
                        (s1, s2)
                    }
                });
        2 * s1 + 2 * s2
    }
}

fn parse_input(input: Vec<u8>) -> Vec<Present> {
    let re = regex::bytes::Regex::new(
        r#"(?m)^(?P<l>[[:digit:]]+)x(?P<w>[[:digit:]]+)x(?P<h>[[:digit:]]+)"#,
    )
    .unwrap();
    re.captures_iter(&input)
        .map(|c| {
            let (_, [l, w, h]) = c.extract();
            let l = fold_decimal_from(l);
            let w = fold_decimal_from(w);
            let h = fold_decimal_from(h);
            Present { l, w, h }
        })
        .collect::<Vec<Present>>()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (tot_paper, tot_ribbon) =
        parse_input(input)
            .iter()
            .fold((0, 0), |(paper, ribbon), present| {
                (
                    paper + present.surface_area() + present.smallest_area(),
                    ribbon + present.volume() + present.smallest_perimeter(),
                )
            });
    println!("Part1: {tot_paper}, Part2: {tot_ribbon}");
    Ok(())
}
