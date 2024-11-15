use aoc_shared::read_input_to_string;
use std::io;

type Output = Vec<Vec<QAndA>>;
type Solved = u32;

struct QAndA(pub u8);

impl From<u8> for QAndA {
    fn from(value: u8) -> Self {
        QAndA(match value {
            b'a'..=b'z' => value - 97u8,
            _ => 27u8,
        })
    }
}

impl QAndA {
    fn is_delim(&self) -> bool {
        self.0 == 27
    }
}

fn parse_input(input: &str) -> Output {
    input
        .split("\n\n")
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.as_bytes().iter().map(|&c| c.into()).collect::<Vec<_>>())
            }
        })
        .collect::<Vec<_>>()
}

fn part1_sol(input: &Output) -> Solved {
    input.iter().fold(0u32, |acc, group| {
        acc + group
            .iter()
            .filter(|a| !a.is_delim())
            .fold(0u32, |acc, a| acc | (1u32 << a.0))
            .count_ones()
    })
}

fn part2_sol(input: &Output) -> Solved {
    input.iter().fold(0u32, |acc, group| {
        let (gtot, yes) = group
            .iter()
            .fold((2u32.pow(26) - 1, 0u32), |(gtot, yes), a| {
                if a.is_delim() {
                    (gtot & yes, 0)
                } else {
                    (gtot, yes | (1 << a.0))
                }
            });
        // handle residue
        let gtot = if yes != 0 {
            gtot & yes
        } else {
            gtot // trailing newline in input, no data to handle.
        };
        acc + gtot.count_ones()
    })
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
