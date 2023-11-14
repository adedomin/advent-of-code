use aoc_shared::{array_windows, debug, read_input};
use std::{collections::HashSet, io};

const VOWELS: &[u8] = b"aeiou";

fn bad_combo(a: u8, b: u8) -> bool {
    match (a, b) {
        (b'a', b'b') => true,
        (b'c', b'd') => true,
        (b'p', b'q') => true,
        (b'x', b'y') => true,
        _ => false,
    }
}

#[derive(Debug)]
enum Why {
    VowelLw,
    VowelOk,
    DoublesOk,
    NoDoubles,
    BadPattrn,
}

fn good_word(word: &[u8]) -> bool {
    use Why::*;
    if word.len() < 3 {
        return false;
    }

    let vowel = if word.iter().filter(|&chr| VOWELS.contains(chr)).count() > 2 {
        VowelOk
    } else {
        VowelLw
    };
    let double_letters = array_windows(word)
        .try_fold(NoDoubles, |acc, &[a, b]| {
            if bad_combo(a, b) {
                None // same as BadPattrn
            } else if a == b {
                Some(DoublesOk)
            } else {
                Some(acc)
            }
        })
        .unwrap_or(BadPattrn);

    debug!(
        "Vowel State: {vowel:?}, Other Prop: {double_letters:?} -> Word {}",
        std::str::from_utf8(word).unwrap(),
    );

    matches!((vowel, double_letters), (VowelOk, DoublesOk))
}

fn good_word_p2(word: &[u8]) -> bool {
    if word.len() < 3 {
        return false;
    }

    let (_, duplicates, _) = array_windows(word).fold(
        (HashSet::new(), HashSet::new(), None),
        |(mut uniq, mut dups, last), &[a, b]| {
            // we have to reject overlapping pair
            // e.g. aaa generates 2 pairs, aa, aa.
            // aaaa however is ok, which is why after finding an overlap we set
            // last = None so aa, -aa- (filtered), aa from aaaa
            if let Some((alast, blast)) = last {
                if alast == a && blast == b {
                    return (uniq, dups, None);
                }
            }

            if uniq.contains(&(a, b)) {
                dups.insert((a, b));
            } else {
                uniq.insert((a, b));
            }

            (uniq, dups, Some((a, b)))
        },
    );
    let duplicates = duplicates.len();

    // detects xyx where *x* is separated by one char. this applies to aaa as well, where the separator
    // is identical.
    let split_pairs = array_windows(word).filter(|&[a, _, c]| a == c).count();

    debug!(
        "duplicates: {duplicates} > 0 && split_pairs: {split_pairs} > 0 ; word: {}",
        std::str::from_utf8(word).unwrap()
    );
    duplicates > 0 && split_pairs > 0
}

fn solve(input: &[u8]) -> usize {
    input
        .split(|&chr| chr == b'\n')
        .filter(|&word| good_word(word))
        .count()
}

fn solve_p2(input: &[u8]) -> usize {
    input
        .split(|&chr| chr == b'\n')
        .filter(|&word| good_word_p2(word))
        .count()
}
fn main() -> io::Result<()> {
    let input = read_input()?;

    let p1 = solve(&input);
    let p2 = solve_p2(&input);
    println!("Part1 {p1}, Part2 {p2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{good_word, good_word_p2};

    #[test]
    fn part1_examples() {
        assert!(good_word(b"ugknbfddgicrmopn"));
        assert!(good_word(b"aaa"));
        assert!(!good_word(b"jchzalrnumimnmhp"));
        assert!(!good_word(b"haegwjzuvuyypxyu"));
        assert!(!good_word(b"dvszwmarrgswjxmb"));
    }

    #[test]
    fn part2_examples() {
        assert!(good_word_p2(b"qjhvhtzxzqqjkmpb"));
        assert!(good_word_p2(b"xxyxx"));
        assert!(!good_word_p2(b"uurcxstgmygtbstg"));
        assert!(!good_word_p2(b"ieodomkazucvgmuy"));
        assert!(!good_word_p2(b"aaa"));
        assert!(good_word_p2(b"aaaa"));
    }
}
