use aoc_shared::{array_windows, read_input_to_string, FlatVec2D};
use itertools::Itertools;
use std::io;

type Int = isize;
type Output = Vec<(Int, Int, Int, Int)>;

fn parse_input(input: &str) -> Output {
    input
        .split(|chr: char| !chr.is_ascii_digit() && chr != '-')
        .filter_map(|num| {
            // bool type does not have a "map" sadly.
            (!num.is_empty())
                .then(|| num.parse::<Int>().ok())
                .and_then(|x| x)
        })
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
// the distortion of the picture must be under this number????
const P1_TOTAL_GUESS: f64 = 1f64;

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

fn calc_disorder(picture: &FlatVec2D<bool>) -> f64 {
    let (mis, set) = array_windows(&picture.0).fold((0f64, 0f64), |(mismatch, set), [l, r]| {
        let mut mis = mismatch;
        let mut set = set;
        if l != r {
            mis += 1f64;
        }

        if *l {
            set += 1f64;
        }

        (mis, set)
    });
    // there has to be at least one set pixel (>0)
    mis / set
}

fn print_tree(picture: &FlatVec2D<bool>) {
    for y in picture.yrange() {
        for x in picture.xrange() {
            if picture[(x, y)] {
                print!("@")
            } else {
                print!(" ")
            }
        }
        println!();
    }
}

// at each step, check the disorder.
fn part2_sol(input: &Output) -> Int {
    let mut points = input.clone();
    let mut picture = FlatVec2D::<bool>::new(P1_BOUNDS_X as usize, P1_BOUNDS_Y as usize);
    for i in 1.. {
        picture.0.fill(false);
        points.iter_mut().for_each(|(x, y, vx, vy)| {
            *x = aoc_shared::wrap(*x + *vx, P1_BOUNDS_X) as isize;
            *y = aoc_shared::wrap(*y + *vy, P1_BOUNDS_Y) as isize;
            picture[(*x as usize, *y as usize)] = true;
        });

        let disorder = calc_disorder(&picture);
        // literal
        if disorder < P1_TOTAL_GUESS {
            #[cfg(debug_assertions)]
            print_tree(&picture);
            return i;
        }
    }
    unreachable!();
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
