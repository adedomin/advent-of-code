use aoc_shared::{fold_decimal, read_input};
use itertools::{chain, Itertools};
use regex::bytes::Regex;
use std::{
    collections::{HashMap, HashSet},
    io,
    iter::once,
};

type Output<'a> = (Vec<&'a [u8]>, HashMap<(&'a [u8], &'a [u8]), i64>);

fn parse_input(input: &'_ [u8]) -> Output<'_> {
    let re =
        Regex::new(r##"(?m)^(?<p1>[A-Za-z]+) would (?<verb>gain|lose) (?<value>[[:digit:]]+) happiness units by sitting next to (?<p2>[A-Za-z]+).$"##)
            .unwrap();

    let (plist, pcost) = re.captures_iter(input).fold(
        (HashSet::new(), HashMap::new()),
        |(mut people, mut people_map), rematch| {
            let person1 = rematch
                .name("p1")
                .expect("Should have matched: city1")
                .as_bytes();
            let person2 = rematch
                .name("p2")
                .expect("Should have matched: city2")
                .as_bytes();

            let sign = if rematch.name("verb").expect("should match").as_bytes() == b"gain" {
                1
            } else {
                -1
            };
            let value = sign
                * rematch
                    .name("value")
                    .expect("Should have matched: cost")
                    .as_bytes()
                    .iter()
                    .fold(0i64, fold_decimal);

            people_map.insert((person1, person2), value);
            people.insert(person1);
            people.insert(person2);
            (people, people_map)
        },
    );
    let plist = plist.iter().copied().collect::<Vec<&[u8]>>();
    (plist, pcost)
}

fn solve<'a>(people: &[&'a [u8]], costs: &HashMap<(&'a [u8], &'a [u8]), i64>) -> i64 {
    if people.len() < 3 {
        panic!("list too short to decide.");
    }
    people
        .iter()
        .copied()
        .permutations(people.len())
        .map(|perm| {
            let last = *perm.last().expect("list can't be empty");
            // we add the last to the list to make it appear "circular"
            // the table arrangement is inherently circular.
            chain!(once(last), perm,)
                .tuple_windows()
                .fold(0i64, |acc, (p1, p2)| {
                    // 0 is because if any two pair don't have a "happiness preference" we assume pure neutral.
                    let left_to_right = *costs.get(&(p1, p2)).unwrap_or(&0);
                    let right_to_left = *costs.get(&(p2, p1)).unwrap_or(&0);
                    let sum = left_to_right + right_to_left;
                    acc + sum
                })
        })
        .max()
        .unwrap()
}

// fn part2_sol(input: Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (mut people, costs) = parse_input(&input);
    let p1 = solve(&people, &costs);
    // part2 has a new person, ME, that has total neutrality, so we can just inject ourself into the list.
    // because we don't want to collide with real persons, we'll assume I am the string, ε (0 length) which
    // our input cannot have as it wouldn't match the tokenizer.
    people.push(b"");
    let p2 = solve(&people, &costs);
    println!("Part1: {p1}, Part2: {p2}");
    Ok(())
}
