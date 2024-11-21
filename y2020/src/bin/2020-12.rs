use aoc_shared::{fold_decimal_from, read_input};
use std::io;

type Output = Vec<Direction>;
type Solved = isize;

const EAST: usize = 1;
const HEADINGS: [(isize, isize); 4] = [
    (0, -1), // N
    (1, 0),  // E
    (0, 1),  // S
    (-1, 0), // W
];

const TWO_SEVENTY_SIN: isize = -1;
const TWO_SEVENTY_COS: isize = 0;
const ONE_EIGHTY_SIN: isize = 0;
const ONE_EIGHTY_COS: isize = -1;
const NINTY_SIN: isize = 1;
const NINTY_COS: isize = 0;

#[derive(Clone, Copy)]
struct QRot(i8);

impl From<isize> for QRot {
    fn from(value: isize) -> Self {
        QRot((value / 90 % HEADINGS.len() as isize) as i8)
    }
}

impl QRot {
    fn new_heading(&self, heading: usize) -> usize {
        assert!(heading < HEADINGS.len());
        let QRot(qrot) = self;
        let curr = heading as i8;
        let sum = curr + *qrot;
        if sum < 0 {
            (4 + sum) as usize
        } else {
            (sum % 4) as usize
        }
    }

    fn rot2d_around(&self, x: isize, y: isize) -> (isize, isize) {
        let QRot(quartrot) = self;
        if *quartrot == 0 {
            return (x, y);
        }
        // lookup sin() and cos() from table based on "90deg turns."
        let (sin, cos) = if *quartrot < 0 {
            if *quartrot == -1 {
                (TWO_SEVENTY_SIN, TWO_SEVENTY_COS)
            } else if *quartrot == -3 {
                (NINTY_SIN, NINTY_COS)
            } else {
                (ONE_EIGHTY_SIN, ONE_EIGHTY_COS)
            }
        } else if *quartrot == 1 {
            (NINTY_SIN, NINTY_COS)
        } else if *quartrot == 2 {
            (ONE_EIGHTY_SIN, ONE_EIGHTY_COS)
        } else {
            (TWO_SEVENTY_SIN, TWO_SEVENTY_COS)
        };

        // basic 2D rotation matmul.
        (x * cos - y * sin, x * sin + y * cos)
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Fixed(isize, isize),
    Rot(QRot),
    Straight(isize),
}

impl From<&[u8]> for Direction {
    fn from(value: &[u8]) -> Self {
        assert!(value.len() > 1);
        let (dir, num) = value.split_at(1);
        let speed: isize = fold_decimal_from(num);
        match dir[0] {
            b'N' => Self::Fixed(0, -speed),
            b'E' => Self::Fixed(speed, 0),
            b'S' => Self::Fixed(0, speed),
            b'W' => Self::Fixed(-speed, 0),
            b'L' => Self::Rot((-speed).into()),
            b'R' => Self::Rot(speed.into()),
            _ => Self::Straight(speed),
        }
    }
}

fn parse_input(input: &[u8]) -> Output {
    input
        .split(|&chr| chr == b'\n')
        .filter_map(|word| {
            if word.is_empty() {
                None
            } else {
                Some(word.into())
            }
        })
        .collect::<Output>()
}

fn part1_sol(input: &Output) -> Solved {
    let mut x = 0;
    let mut y = 0;
    let mut heading = EAST;

    input.iter().for_each(|dir| match dir {
        Direction::Fixed(dx, dy) => {
            x += dx;
            y += dy;
        }
        Direction::Rot(qrot) => {
            heading = qrot.new_heading(heading);
        }
        Direction::Straight(speed) => {
            let (dx, dy) = HEADINGS[heading];
            x += dx * (*speed);
            y += dy * (*speed);
        }
    });
    x.abs() + y.abs()
}

fn part2_sol(input: &Output) -> Solved {
    let mut xway = 10isize;
    let mut yway = -1isize;
    let mut x = 0isize;
    let mut y = 0isize;

    input.iter().for_each(|dir| match dir {
        Direction::Fixed(dx, dy) => {
            xway += dx;
            yway += dy;
        }
        Direction::Rot(qrot) => {
            (xway, yway) = qrot.rot2d_around(xway, yway);
        }
        Direction::Straight(mult) => {
            x += xway * mult;
            y += yway * mult;
        }
    });
    x.abs() + y.abs()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
