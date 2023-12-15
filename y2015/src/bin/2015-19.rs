use aoc_shared::{read_input, Token, Tokenize};
use regex::bytes::Regex;
use std::{
    collections::{HashMap, HashSet},
    io,
};

#[derive(Debug, Clone)]
struct Replacer<'a>(&'a [u8], &'a [u8]);

fn parse_input(input: &[u8]) -> (Vec<Replacer>, &[u8]) {
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

const LHS_TERM: u8 = 255;

fn parse_input2<'a>(replace: &[Replacer<'a>], molecule: &'a [u8]) -> (Vec<u8>, Vec<u8>) {
    let mut last = 0u8;
    let mut tok_list: HashMap<&[u8], u8> =
        HashMap::from_iter(replace.iter().cloned().map(|Replacer(find, _)| {
            last += 1;
            (find, last)
        }));

    // I don't feel like building my own.
    let re = Regex::new(r#"([A-Z][a-z]*)"#).unwrap();

    // find all unique terminating tokens
    let terminals = replace
        .iter()
        .flat_map(|Replacer(_, replacement)| {
            // we have to collect matches first to properly filter tokens in tail position.
            let matchers = re
                .captures_iter(replacement)
                .map(|m| m.get(0).unwrap().as_bytes())
                .collect::<Vec<&'a [u8]>>();
            matchers
                .iter()
                .enumerate()
                .flat_map(|(i, m)| {
                    if tok_list.contains_key(m) || i == matchers.len() - 1 {
                        // this filters out tokens in tail or front, since their purpose appears to be surrounding.
                        None
                    } else if i == 0 {
                        tok_list.insert(m, LHS_TERM);
                        Some(LHS_TERM) // this is a special terminator... it's on LHS, it's not like the interspersed ones on right.
                                       // it only deletes 1 instead of 2
                    } else {
                        last += 1;
                        tok_list.insert(m, last);
                        Some(last)
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    // create new molecule using token numbers
    let new_mol = re
        .captures_iter(molecule)
        .map(|m| {
            let c = m.get(0).unwrap().as_bytes();
            *tok_list.get(c).unwrap_or(&0)
        })
        .collect::<Vec<u8>>();
    (terminals, new_mol)
}

fn find_all(Replacer(find, _): &Replacer<'_>, molecule: &[u8]) -> Vec<usize> {
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

fn part2_sol(term: Vec<u8>, molecule: Vec<u8>) -> i32 {
    let m = molecule.len() as i32;
    let del = molecule.iter().fold(0i32, |acc, tok| {
        if term.contains(tok) && *tok != LHS_TERM {
            acc + 2
        } else if *tok == LHS_TERM {
            acc + 1
        } else {
            acc
        }
    });
    m - del
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (replacements, mol) = parse_input(&input);
    let part1 = part1_sol(&replacements, mol).len();
    print!("Part1: {part1}, ");
    let (terminals, mol) = parse_input2(&replacements, mol);
    let part2 = part2_sol(terminals, mol);
    println!("Part2: {part2}");
    Ok(())
}
