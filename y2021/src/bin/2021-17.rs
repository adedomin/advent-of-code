use std::{collections::HashSet, io};

use aoc_shared::{fold_decimal, read_input, AoCTokenizer, Token};

#[derive(Debug)]
struct Points {
    xmax: i32,
    ymax: i32,
    xmin: i32,
    ymin: i32,
}

enum Parse {
    Nothing,
    X,
    Y,
}

fn parse(input: Vec<u8>) -> Points {
    let (point, _, _) = AoCTokenizer::new(&input).fold(
        (
            Points {
                xmax: i32::MIN,
                ymax: i32::MIN,
                xmin: i32::MAX,
                ymin: i32::MAX,
            },
            Parse::Nothing,
            1,
        ),
        |(mut acc, state, neg), tok| match tok {
            Token::Something(str) if str == b"x" => (acc, Parse::X, 1),
            Token::Something(str) if str == b"y" => (acc, Parse::Y, 1),
            Token::Delimiter(del) if del == b'-' => (acc, state, -1),
            Token::Something(num) if num != b"target" && num != b"area" => {
                let n = if neg == -1 {
                    neg * num.iter().fold(0i32, fold_decimal)
                } else {
                    num.iter().fold(0i32, fold_decimal)
                };
                if let Parse::X = state {
                    if acc.xmax < n {
                        acc.xmax = n;
                    }
                    if acc.xmin > n {
                        acc.xmin = n
                    }
                } else if let Parse::Y = state {
                    if acc.ymax < n {
                        acc.ymax = n;
                    }
                    if acc.ymin > n {
                        acc.ymin = n
                    }
                }
                (acc, state, 1)
            }
            _ => (acc, state, 1),
        },
    );
    point
}

// int math truncates too soon. also no sqrt()
fn quadratic(a: f32, b: f32, c: f32) -> f32 {
    [
        (-b + (b * b - (4.0 * a * c)).sqrt()) / (2.0 * a),
        (-b - (b * b - (4.0 * a * c)).sqrt()) / (2.0 * a),
    ]
    .iter()
    .fold(0.0, |acc, &ans| if ans > 0.0 { ans } else { acc })
}

fn solve_possible(
    Points {
        xmax,
        ymax,
        xmin,
        ymin,
    }: Points,
) -> i32 {
    let ybound = ymin.abs().max(ymax.abs()) - 1;

    // min/max distance til x vector is 0.
    let min_velx_rest = quadratic(1.0, 1.0, -(2 * xmin) as f32).ceil() as i32;
    let max_velx_rest = quadratic(1.0, 1.0, -(2 * xmax) as f32).floor() as i32;

    let mut set = HashSet::<(i32, i32)>::new();
    (ymin..=ybound).for_each(|vy| {
        let (off, fvy) = if vy > 0 {
            (2 * vy + 1, -vy as f32 - 1.0)
        } else if vy == 0 {
            (1, -1.0)
        } else {
            (0, vy as f32)
        };
        let yt_min = quadratic(-1.0, 2.0 * fvy + 1.0, -2.0 * ymax as f32).ceil() as i32 + off;
        let yt_max = quadratic(-1.0, 2.0 * fvy + 1.0, -2.0 * ymin as f32).floor() as i32 + off;
        if yt_min > yt_max {
            return;
        }
        (yt_min..=yt_max).for_each(|t| {
            // Finding vx for given vy and t (time) as followed
            // x = (1/2) * (t) * (2*vx - (t - 1))
            // 2 * x = t * (2vx - (t - 1))
            // (2 * x) / t = 2vx - t + 1
            // ((2 * x) / t) + t - 1 = 2vx
            // (((2 * x) / t) + t - 1) / 2 = vx
            // int math truncates too soon. also no sqrt()
            let min_vx = ((((2.0 * xmin as f32) / t as f32) + t as f32 - 1.0) / 2.0).ceil() as i32;
            let max_vx = ((((2.0 * xmax as f32) / t as f32) + t as f32 - 1.0) / 2.0).floor() as i32;
            let xrange = (min_vx..=max_vx).filter(|&vx| vx >= t);
            if min_velx_rest < t {
                xrange
                    .chain(min_velx_rest..=max_velx_rest.min(t))
                    .for_each(|vx| {
                        set.insert((vx, vy));
                    });
            } else {
                xrange.for_each(|vx| {
                    set.insert((vx, vy));
                })
            };
        });
    });
    set.len() as i32
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let area_points = parse(input);
    // Take the lowest area and knowing that y velocity decreases by
    // -1, we're basically just looking for the highest y that will fall into this point.
    let p1 = (area_points.ymin.abs() * (area_points.ymin.abs() - 1)) / 2;
    let p2 = solve_possible(area_points);
    println!("Part1 {:?}, Part2 {}", p1, p2);
    Ok(())
}
