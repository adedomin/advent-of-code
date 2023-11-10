use std::{
    collections::HashSet,
    fmt::{Debug, Write},
    io,
};

use aoc_shared::read_input;

struct Picture {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
    iter: i64,
    pic: HashSet<(i64, i64)>,
    enhance: [bool; 512],
}

impl Debug for Picture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.ymin - 3..self.ymax + 3 {
            for x in self.xmin - 3..self.xmax + 3 {
                if self.contains(x, y) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Picture {
    fn increment_iter(&mut self) {
        self.iter += 1;
        self.xmin -= 1;
        self.ymin -= 1;
        self.xmax += 1;
        self.ymax += 1;
    }

    fn out_plot(&self, x: i64, y: i64) -> bool {
        self.xmin > x || self.xmax <= x || self.ymin > y || self.ymax <= y
    }

    fn has_infinite_on(&self) -> bool {
        if self.iter == 0 {
            false
        } else if self.iter % 2 == 0 {
            if self.enhance[0] {
                self.enhance[self.enhance.len() - 1]
            } else {
                false
            }
        } else {
            self.enhance[0]
        }
    }

    fn contains(&self, x: i64, y: i64) -> bool {
        if self.out_plot(x, y) {
            self.has_infinite_on()
        } else {
            self.pic.contains(&(x, y))
        }
    }

    fn contains_as_bit(&self, x: i64, y: i64) -> usize {
        self.contains(x, y) as usize
    }

    fn to_enhance_num(&self, x: i64, y: i64) -> usize {
        self.contains_as_bit(x - 1, y - 1) << 8
            | self.contains_as_bit(x, y - 1) << 7
            | self.contains_as_bit(x + 1, y - 1) << 6
            | self.contains_as_bit(x - 1, y) << 5
            | self.contains_as_bit(x, y) << 4
            | self.contains_as_bit(x + 1, y) << 3
            | self.contains_as_bit(x - 1, y + 1) << 2
            | self.contains_as_bit(x, y + 1) << 1
            | self.contains_as_bit(x + 1, y + 1)
    }

    fn iterate(&mut self) {
        let mut new_pic = HashSet::<(i64, i64)>::new();
        for y in self.ymin - 1..self.ymax + 1 {
            for x in self.xmin - 1..self.xmax + 1 {
                let at = self.to_enhance_num(x, y);
                let is_set = self.enhance[at];
                if is_set {
                    new_pic.insert((x, y));
                }
            }
        }
        self.pic = new_pic;
        self.increment_iter();
    }
}

fn parse(input: Vec<u8>) -> Picture {
    let (enhancer_rules, picture_start) =
        input.split_at(input.iter().position(|&chr| chr == b'\n').unwrap());

    let enhance = enhancer_rules
        .iter()
        .enumerate()
        .fold([false; 512], |mut acc, (idx, &chr)| {
            if chr == b'#' {
                acc[idx] = true;
                acc
            } else {
                acc
            }
        });

    let mut max_x = i64::MIN;
    let mut cur_x = 0i64;
    let mut cur_y = 0i64;
    let mut pic = HashSet::<(i64, i64)>::new();
    let tokenizer = picture_start.iter().skip_while(|&&chr| chr == b'\n');
    for &token in tokenizer {
        match token {
            b'\n' => {
                if max_x < cur_x {
                    max_x = cur_x;
                }
                cur_x = 0;
                cur_y += 1;
            }
            b'.' => cur_x += 1,
            b'#' => {
                pic.insert((cur_x, cur_y));
                cur_x += 1;
            }
            _ => (),
        }
    }

    Picture {
        xmin: 0,
        xmax: max_x,
        ymin: 0,
        ymax: cur_y,
        iter: 0,
        pic,
        enhance,
    }
}

fn solve(mut picture: Picture) -> (i64, i64) {
    let mut i = 0;
    let p1 = loop {
        if i < 2 {
            picture.iterate();
            // println!("{:?}", picture);
            i += 1;
        } else {
            break picture.pic.len() as i64;
        }
    };
    let p2 = loop {
        if i < 50 {
            picture.iterate();
            // println!("{:?}", picture);
            i += 1;
        } else {
            break picture.pic.len() as i64;
        }
    };
    // println!("{:?}", picture);
    (p1, p2)
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let picture = parse(input);
    // println!("{:?}", picture);
    let (p1, p2) = solve(picture);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
