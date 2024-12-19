use aoc_shared::read_input_to_string;
use std::io;

type Int = usize;
type Output<'a> = (Vec<&'a str>, Vec<&'a str>);

fn parse_input(input: &str) -> Output<'_> {
    let (subtowels, towels) = input
        .split_once("\n\n")
        .expect("Towels and subtowels to be split by two newlines.");
    let st = subtowels
        .split(|char: char| !char.is_ascii_alphabetic())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();
    let t = towels
        .split(|char: char| !char.is_ascii_alphabetic())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();
    (st, t)
}

fn solve<'a>(subtowel: &[&'a str], towels: &[&'a str]) -> (Int, Int) {
    towels.iter().fold((0, 0), |(p1, p2), towel| {
        let mut residue = vec![0; towel.len() + 1];
        // if no pattern matches the front, then the rest of the matchers are moot.
        residue[0] = 1;
        // window over every postfix substring
        (0..towel.len()).for_each(|start| {
            subtowel
                .iter()
                .filter(|&st| towel[start..].starts_with(st)) // filter matching prefix
                .for_each(|st| residue[start + st.len()] += residue[start]); // move value up from last matching.
        });
        (
            p1 + (residue[towel.len()] != 0) as Int,
            p2 + residue[towel.len()],
        )
    })
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let (sub, towels) = parse_input(&input);
    let (part1, part2) = solve(&sub, &towels);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
