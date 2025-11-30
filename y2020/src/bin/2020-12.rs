use aoc_shared::{
    fold_decimal_from, read_input,
    rot::{rot_right, CARDINALS, E},
};
use std::io;

type Output = Vec<Direction>;
type Solved = isize;

#[derive(Clone, Copy)]
enum Direction {
    Fixed(isize, isize),
    Rot(usize),
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
            b'L' => Self::Rot((-speed / 90).rem_euclid(CARDINALS.len() as isize) as usize),
            b'R' => Self::Rot((speed / 90).rem_euclid(CARDINALS.len() as isize) as usize),
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
    // east
    let mut heading_idx = E;

    input.iter().for_each(|dir| match dir {
        Direction::Fixed(dx, dy) => {
            x += dx;
            y += dy;
        }
        Direction::Rot(qrot) => {
            heading_idx = (heading_idx + qrot) % CARDINALS.len();
        }
        Direction::Straight(speed) => {
            let (dx, dy) = CARDINALS[heading_idx];
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
            (0..*qrot).for_each(|_| (xway, yway) = rot_right((xway, yway)));
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
