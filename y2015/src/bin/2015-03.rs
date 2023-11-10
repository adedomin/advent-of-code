use aoc_shared::read_input;

use std::{collections::HashSet, io};

fn move_coord((x, y): (isize, isize), direction: u8) -> (isize, isize) {
    match direction {
        b'^' => (x, y + 1),
        b'>' => (x + 1, y),
        b'v' => (x, y - 1),
        b'<' => (x - 1, y),
        _ => (x, y),
    }
}

/// Where N is number of Santa's
fn p1<const N: usize>(input: &[u8]) -> usize {
    let (visited, _) = input.chunks(N).fold(
        (HashSet::from([(0, 0)]), [(0, 0); N]),
        |(mut visited, mut santas), chunks| {
            santas.iter_mut().zip(chunks).for_each(|(coord, &dir)| {
                *coord = move_coord(coord.clone(), dir);
                visited.insert(coord.clone());
            });
            (visited, santas)
        },
    );
    visited.len()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let visit_p1 = p1::<1>(&input);
    let visit_p2 = p1::<2>(&input);
    println!("Part1: {visit_p1}, Part2: {visit_p2}");
    Ok(())
}
