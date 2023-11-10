use aoc_shared::read_input_to_string;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::io;

fn solve<const LEADING: usize>(key: &str) -> Option<u64> {
    let leading = [b'0'; LEADING];
    (0..u64::MAX).into_par_iter().find_first(|&num| {
        let mut buffer = itoa::Buffer::new();
        let postfix = buffer.format(num);
        let mut message = key.to_owned();
        message.extend(postfix.chars());
        let res = format!("{:?}", md5::compute(&message));

        &res.as_bytes()[..LEADING] == &leading
    })
}

fn main() -> io::Result<()> {
    let key = read_input_to_string()?.trim_end().to_string();

    print!("Part1: ");
    if let Some(p1) = solve::<5>(&key) {
        print!("{p1}");
    } else {
        print!("No answer");
    }
    print!(", Part2: ");
    if let Some(p2) = solve::<6>(&key) {
        print!("{p2}");
    } else {
        print!("No answer");
    }
    println!("");
    Ok(())
}
