use aoc_shared::{read_input, ts_to_u64, u64_to_ts};
use std::io;

// TS == Twenty-Six

/// max num given its limited to 8 positions.
const TS_MAX: u64 = ts_to_u64(b"zzzz_zzzz");
const I: u64 = ts_to_u64(b"i");
const O: u64 = ts_to_u64(b"o");
const L: u64 = ts_to_u64(b"l");

/// function determins if a given number has a pattern like, abc, bcd, cde, ..., xyz.
/// the pattern ascends as you go down the number.
fn ts_contains_req(n: u64) -> bool {
    let mut last = u64::MAX - 1;
    let mut pairs = 0usize;
    let mut overlaps = false;
    let mut found_asc = false;
    // number is 8 pos, need 7 iters for each double
    // we check doubles because it's easier to do all the checks in doubles
    // than triples
    // e.g. ABCDEFGH -> AB, BC, CD, DE, EF, FG, GH.
    for i in (0..7).rev() {
        let t1 = n / 26u64.pow(i + 1) % 26;
        let t2 = n / 26u64.pow(i + 0) % 26;

        if (t1 == I || t1 == O || t1 == L) && (t2 == I || t2 == O || t2 == L) {
            return false;
        }

        // we count up only if non overlap
        if t1 == t2 && !overlaps {
            pairs += 1;
            overlaps = true;
        } else {
            overlaps = false;
        }

        if last + 1 == t1 && t1 + 1 == t2 {
            found_asc = true;
        }

        last = t1;
    }
    found_asc && pairs > 1
}

fn itoa_8_ts(n: u64) -> String {
    let n = u64_to_ts::<8>(n);
    unsafe { std::str::from_utf8_unchecked(&n).to_owned() }
}

/// find the lowest new "password" that matches.
fn solve_p1(n: u64) -> u64 {
    let mut nm = n;
    while nm != TS_MAX {
        if ts_contains_req(nm) {
            return nm;
        }
        nm += 1;
    }
    panic!("NO SOLUTION???");
}

fn main() -> io::Result<()> {
    let input = ts_to_u64(&read_input()?);

    let p1 = solve_p1(input);
    let part1 = itoa_8_ts(p1);

    let p2 = solve_p1(p1 + 1);
    let part2 = itoa_8_ts(p2);

    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}

#[cfg(test)]
mod test {
    use aoc_shared::ts_to_u64;

    use crate::ts_contains_req;

    #[test]
    fn rule_tests() {
        assert!(!ts_contains_req(ts_to_u64(b"hijklmmn")));
        assert!(!ts_contains_req(ts_to_u64(b"abbceffg")));
        assert!(!ts_contains_req(ts_to_u64(b"abbcegjk")));
        assert!(ts_contains_req(ts_to_u64(b"abcdfaaa")));
        assert!(ts_contains_req(ts_to_u64(b"abcdffaa")));
        assert!(ts_contains_req(ts_to_u64(b"ghjaabcc")));
    }
}
