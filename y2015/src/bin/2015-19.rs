use aoc_shared::{read_input, Token, Tokenize};
use itertools::Itertools;
use regex::bytes::Regex;
use std::{
    collections::{HashMap, HashSet},
    io,
};

#[derive(Debug, Clone)]
struct Replacer<'a>(&'a [u8], &'a [u8]);

fn parse_input<'a>(input: &'a [u8]) -> (Vec<Replacer>, &'a [u8]) {
    let (replacements, _, mol, _) = input.tokenize().fold(
        (Vec::new(), None, None, false),
        |(mut acc, mol1, mol2, is_final), tok| match tok {
            Token::Something(word) if !is_final => {
                if mol1.is_some() {
                    (acc, mol1, Some(word), is_final)
                } else {
                    (acc, Some(word), mol2, is_final)
                }
            }
            Token::Something(word) => (acc, mol1, Some(word), is_final),
            Token::Newline if !is_final => {
                if let Some(m1) = mol1 {
                    if let Some(m2) = mol2 {
                        acc.push(Replacer(m1, m2));
                    }
                }
                (acc, None, None, is_final)
            }
            Token::DoubleNewline => {
                if let Some(m1) = mol1 {
                    if let Some(m2) = mol2 {
                        acc.push(Replacer(m1, m2));
                    }
                }
                (acc, None, None, true)
            }
            _ => (acc, mol1, mol2, is_final),
        },
    );
    (
        replacements,
        mol.expect("We should have found the last string."),
    )
}

// unique tokens in my input
// I don't know how else I can compute these weird properties at runtime.
const Y: u8 = 255; // this is interspered in the values that have production rules.
const RN: u8 = Y - 1; // this seems to group with a terminating Ar
const AR: u8 = RN - 1; // always in tail position, assumed related to Rh, X Rh ... Ar
const C: u8 = AR - 1; // always at start of a replacement with an Rh as 2nd? no mapped production, C RH ... Ar

fn parse_input2<'a>(replace: &[Replacer], molecule: &'a [u8]) -> Vec<u8> {
    let mut last = 0u8;
    let mut tok_list: HashMap<&[u8], u8> =
        HashMap::from_iter(replace.iter().cloned().map(|Replacer(find, _)| {
            last += 1;
            (find, last)
        }));

    tok_list.insert(b"Y", Y);
    tok_list.insert(b"Rn", RN);
    tok_list.insert(b"Ar", AR);
    tok_list.insert(b"C", C);

    // I don't feel like building my own.
    let re = Regex::new(r#"([A-Z][a-z]*)"#).unwrap();

    // create new molecule using token numbers
    let new_mol = re
        .captures_iter(molecule)
        .map(|m| {
            let c = m.get(0).unwrap().as_bytes();
            println!("{}", std::str::from_utf8(c).unwrap());
            *tok_list.get(c).unwrap_or(&0)
        })
        .collect::<Vec<u8>>();
    new_mol
}

fn find_all<'a>(Replacer(find, _): &Replacer<'a>, molecule: &[u8]) -> Vec<usize> {
    molecule.windows(find.len()).enumerate().fold(
        Vec::with_capacity(molecule.len()),
        |mut acc, (i, el)| {
            if el == *find {
                acc.push(i);
            }
            acc
        },
    )
}

fn replace_all<'a>(
    Replacer(find, replace): &Replacer<'a>,
    molecule: &'a [u8],
    pos: &'a [usize],
) -> impl Iterator<Item = Vec<u8>> + 'a {
    pos.iter().map(|&i| {
        let (mc1, mc2) = molecule.split_at(i);
        let mc2 = &mc2[find.len()..];
        let mut new = Vec::with_capacity(mc1.len() + mc2.len() + replace.len());
        new.extend_from_slice(mc1);
        new.extend_from_slice(replace);
        new.extend_from_slice(mc2);
        new
    })
}

fn part1_sol(replace: &[Replacer], molecule: &[u8]) -> HashSet<Vec<u8>> {
    let mut replacements = HashSet::with_capacity(replace.len());
    replace.iter().for_each(|repl| {
        let pos = find_all(repl, molecule);
        replacements.extend(replace_all(repl, molecule, &pos));
    });
    replacements
}

fn part2_sol<'a>(molecule: Vec<u8>) -> i32 {
    let mlen = molecule.len() as i32;
    let delete_term = molecule.iter().fold(0i32, |acc, &tok| {
        // These are found in patterns and basically give us 2 kills for one replace
        // e.g. {el} => ..Y{el}..
        if tok == Y {
            acc + 2
        // these are always either in the RHS position as pairs
        // or in the exotic case where the terminal C is in LHS
        // This is basically hardcoded from studying my input, very disgusting.
        // computing these otherwise isn't practical.
        } else if tok == RN || tok == AR || tok == C {
            acc + 1
        } else {
            acc
        }
    });
    mlen - delete_term
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (replacements, mol) = parse_input(&input);
    let part1 = part1_sol(&replacements, &mol).len();
    print!("Part1: {part1}, ");
    let mol = parse_input2(&replacements, mol);
    let part2 = part2_sol(mol);
    println!("Part2: {part2}");
    Ok(())
}
