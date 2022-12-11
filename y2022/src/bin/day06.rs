#![feature(array_windows)]
use std::io;
use y2022::read_input;

const START_OF_HEAD_OFF: usize = 4;
const START_OF_MSG_OFF: usize = 14;

macro_rules! ne_for_all_permut {
    ($($el:ident),* $(,)?) => {
        ne_for_all_permut!(@inner $($el)*; $($el)*)
    };
    (@inner $head:ident $($tail:ident)*; $($back:ident)*) => {
        $(
            if stringify!($head) == stringify!($back) { true } else {
                $head != $back
            } &&
        )*
        ne_for_all_permut!(@inner $($tail)*; $($back)*)
    };
    (@inner ; $($el:ident)*) => { true };
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let part1 = input
        .array_windows()
        .position(|[a, b, c, d]| ne_for_all_permut!(a, b, c, d))
        .expect("No start of packet found")
        + START_OF_HEAD_OFF;

    let part2 = input
        .array_windows()
        .position(|[a, b, c, d, e, f, g, h, i, j, k, l, m, n]| {
            ne_for_all_permut!(a, b, c, d, e, f, g, h, i, j, k, l, m, n)
        })
        .expect("No start of packet found")
        + START_OF_MSG_OFF;
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
