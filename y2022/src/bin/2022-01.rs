use aoc_shared::{fold_decimal, read_input, AoCTokenizer, Token};
use std::io;

fn main() -> io::Result<()> {
    let input = read_input()?;
    let tokenizer = AoCTokenizer::new(&input);

    let (top3, _) = tokenizer.fold(([0; 3], None), |(top3, total), token| match token {
        Token::Something(word) => {
            let kcal = word.iter().fold(0, fold_decimal);
            (top3, total.map_or(Some(kcal), |tot| Some(tot + kcal)))
        }
        Token::DoubleNewline | Token::End if let Some(total) = total => {
            if top3[0] < total {
                ([total, top3[0], top3[1]], None)
            } else if top3[1] < total {
                ([top3[0], total, top3[1]], None)
            } else if top3[2] < total {
                ([top3[0], top3[1], total], None)
            } else {
                (top3, None)
            }
        }
        _ => (top3, total),
    });

    let top1 = top3[0];
    let top3_total = top3.iter().sum::<i32>();
    println!("Part 1: {top1}, Part 2: {top3_total}");
    Ok(())
}
