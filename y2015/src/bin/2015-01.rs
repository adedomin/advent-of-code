use std::io;

use aoc_shared::read_input;

fn main() -> io::Result<()> {
    let input = read_input()?;

    let (sum_p1, pos_p2) =
        input
            .iter()
            .enumerate()
            .fold((0, None), |(sum, found_negsum), (idx, val)| {
                let sum = match val {
                    b'(' => 1,
                    b')' => -1,
                    _ => 0,
                } + sum;
                if found_negsum.is_none() && sum == -1 {
                    (sum, Some(idx + 1))
                } else {
                    (sum, found_negsum)
                }
            });
    print!("Part1: {sum_p1}, Part2: ");
    if let Some(pos) = pos_p2 {
        println!("{pos}");
    } else {
        println!("No answer.");
    }
    Ok(())
}
