use std::io;

use aoc_shared::read_input_to_string;
use regex::RegexSet;

type Output<'a> = Vec<Vec<&'a str>>;
type Solved = i64;

const P1_RE: &[&str] = &[
    r"^cid:", r"^byr:", r"^iyr:", r"^eyr:", r"^hcl:", r"^ecl:", r"^pid:", r"^hgt:",
];

const P2_RE: &[&str] = &[
    r"^cid:.+$",
    r"^byr:(19[2-9][0-9]|200[0-2])$",
    r"^iyr:(201[0-9]|2020)$",
    r"^eyr:(202[0-9]|2030)$",
    r"^hcl:#[0-9a-fA-F]{6}$",
    r"^ecl:(amb|blu|brn|gry|grn|hzl|oth)$",
    r"^pid:[0-9]{9}$",
    r"^hgt:((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)$",
];

fn parse_input(input: &'_ str) -> Output<'_> {
    input
        .split("\n\n")
        .map(|strings| strings.split_ascii_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<_>>()
}

fn solve(passports: &Output, re_set: &[&str]) -> Solved {
    let mut tot_valid = 0;
    let re = RegexSet::new(re_set).unwrap();
    'nextpass: for passport in passports {
        let mut valid_entries = vec![false; re_set.len()];
        for &line in passport {
            let matches = re.matches(line).into_iter().collect::<Vec<usize>>();
            if matches.is_empty() {
                continue 'nextpass;
            }
            assert_eq!(matches.len(), 1);
            valid_entries[matches[0]] = true;
        }

        let has_cid = valid_entries[0];
        let count_ents = valid_entries.into_iter().map(|b| b as i32).sum::<i32>();
        if (!has_cid && count_ents == 7) || count_ents == 8 {
            tot_valid += 1;
        }
    }
    tot_valid
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = solve(&parsed_input, P1_RE);
    let part2 = solve(&parsed_input, P2_RE);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
