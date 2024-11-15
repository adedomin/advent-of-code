use aoc_shared::{parse_to_flat2d, read_input, FlatVec2D};
use std::io;

#[derive(Default, Copy, Clone)]
enum X {
    #[default]
    Dot,
    Hash,
}

impl From<u8> for X {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Self::Hash,
            _ => Self::Dot,
        }
    }
}

type Output = FlatVec2D<X>;
type Solved = i64;

fn part1_sol(map: &Output) -> Solved {}

fn part2_sol(map: &Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_to_flat2d(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
