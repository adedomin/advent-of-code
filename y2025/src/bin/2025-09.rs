use std::io;

use aoc_shared::{array_windows, read_input_to_string};

type Int = i64;
type Point = (Int, Int);
type Line = (Point, Point);

fn parse_input(input: &str) -> Vec<Point> {
    input
        .split_ascii_whitespace()
        .map(|line| {
            line.split_once(',')
                .map(|(x, y)| {
                    (
                        x.parse::<Int>().expect("num"),
                        y.parse::<Int>().expect("num"),
                    )
                })
                .expect("cartesian coord pair.")
        })
        .collect::<Vec<_>>()
}

fn order_tup((x1, y1): Point, (x2, y2): Point) -> (Point, Point) {
    ((x1.min(x2), y1.min(y2)), (x1.max(x2), y1.max(y2)))
}

fn to_lines(input: &[Point]) -> Vec<(Point, Point)> {
    let mut iclone = input.to_vec();
    // add trailer to complete the poly
    iclone.push(iclone[0]);
    array_windows(&iclone)
        .map(|&[p1, p2]| order_tup(p1, p2))
        .collect::<Vec<_>>()
}

fn solve(input: &[Point], part2: Option<&[Line]>) -> Int {
    (0..input.len() - 1)
        .flat_map(|x| (x + 1..input.len()).map(move |y| order_tup(input[x], input[y])))
        .filter(|((x1, y1), (x2, y2))| {
            if let Some(poly) = part2 {
                poly.iter().all(|((px1, py1), (px2, py2))| {
                    x1 >= px2 || x2 <= px1 || y1 >= py2 || y2 <= py1
                })
            } else {
                true
            }
        })
        .map(|((x1, y1), (x2, y2))| (x2 - x1 + 1) * (y2 - y1 + 1))
        .max()
        .expect("one num")
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let input = parse_input(input.trim());
    let part1 = solve(&input, None);
    let input_2 = to_lines(&input);
    let part2 = solve(&input, Some(&input_2));
    println!("Part1 {part1}  Part2 {part2}");
    Ok(())
}
