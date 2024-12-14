use aoc_shared::read_input_to_string;
use itertools::Itertools;
use std::io;

type Int = isize;
type Output = Vec<(Int, Int, Int, Int)>;

fn parse_input(input: &str) -> Output {
    input
        .split(|chr: char| !chr.is_ascii_digit() && chr != '-')
        .filter(|word| !word.is_empty())
        .filter_map(|num| num.parse::<Int>().ok())
        .tuples()
        .collect::<Output>()
}

const P1_BOUNDS_X: Int = 101;
const P1_BOUNDS_Y: Int = 103;

const P1_QUADS: [(Int, Int, Int, Int); 4] = [
    (0, P1_BOUNDS_X / 2, 0, P1_BOUNDS_Y / 2),
    (P1_BOUNDS_X / 2 + 1, P1_BOUNDS_X, 0, P1_BOUNDS_Y / 2),
    (0, P1_BOUNDS_X / 2, P1_BOUNDS_Y / 2 + 1, P1_BOUNDS_Y),
    (
        P1_BOUNDS_X / 2 + 1,
        P1_BOUNDS_X,
        P1_BOUNDS_Y / 2 + 1,
        P1_BOUNDS_Y,
    ),
];

const P1_MOVE: Int = 100;

fn part1_sol(input: &Output) -> Int {
    let quads = P1_QUADS.map(|(xs, xe, ys, ye)| (xs..xe, ys..ye));
    input
        .iter()
        .map(|(sx, sy, vx, vy)| {
            (
                aoc_shared::wrap(sx + vx * P1_MOVE, P1_BOUNDS_X) as isize,
                aoc_shared::wrap(sy + vy * P1_MOVE, P1_BOUNDS_Y) as isize,
            )
        })
        .fold([0, 0, 0, 0], |mut acc, (x, y)| {
            let Some(i) = quads
                .iter()
                .position(|(rx, ry)| rx.contains(&x) && ry.contains(&y))
            else {
                return acc;
            };
            acc[i] += 1;
            acc
        })
        .into_iter()
        .product()
}

// fn part2_sol(input: &Output) -> Int {}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    // let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    // print!("Part2: {part2}");
    println!();
    Ok(())
}
