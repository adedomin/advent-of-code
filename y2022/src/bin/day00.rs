use std::io;
use y2021::{read_input, AoCTokenizer};

fn main() -> io::Result<()> {
    let input = read_input()?;
    let tokenizer = AoCTokenizer::new(&input);
    println!(
        "{}",
        tokenizer.fold(String::new(), |mut acc, token| {
            match token {
                y2021::Token::Something(word) if word != b"world" => {
                    acc.push_str(String::from_utf8(word.into()).unwrap().as_str());
                    acc
                }
                y2021::Token::Something(_) => {
                    acc.push_str(env!("USER"));
                    acc
                }
                y2021::Token::Delimiter(symbol) => {
                    acc.push(symbol.try_into().unwrap());
                    acc
                }
                y2021::Token::Newline => acc,
                y2021::Token::Space => acc,
                y2021::Token::End => acc,
            }
        })
    );
    Ok(())
}
