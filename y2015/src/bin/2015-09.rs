use aoc_shared::{fold_decimal, read_input};
use itertools::Itertools;
use regex::bytes::Regex;
use std::{
    collections::{HashMap, HashSet},
    io,
};

type Output<'a> = (Vec<&'a [u8]>, HashMap<(&'a [u8], &'a [u8]), u64>);
type Solved = u64;

// Generates a pair in lexicographic order, for uniqueness.
// City paths are bidirectional.
fn gen_city_pair<'a>(city_a: &'a [u8], city_b: &'a [u8]) -> (&'a [u8], &'a [u8]) {
    match city_a.cmp(city_b) {
        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => (city_a, city_b),
        std::cmp::Ordering::Greater => (city_b, city_a),
    }
}

fn parse_input(input: &'_ [u8]) -> Output<'_> {
    let re =
        Regex::new(r##"(?m)^(?<city1>[A-Za-z]+) to (?<city2>[A-Za-z]+) = (?<cost>[[:digit:]]+)$"##)
            .unwrap();

    let (clist, ccost) = re.captures_iter(input).fold(
        (HashSet::new(), HashMap::new()),
        |(mut cities, mut city_cost), rematch| {
            let start = rematch
                .name("city1")
                .expect("Should have matched: city1")
                .as_bytes();
            let end = rematch
                .name("city2")
                .expect("Should have matched: city2")
                .as_bytes();
            let cost = rematch
                .name("cost")
                .expect("Should have matched: cost")
                .as_bytes()
                .iter()
                .fold(0u64, fold_decimal);
            city_cost.insert(gen_city_pair(start, end), cost);
            cities.extend([start, end]);

            (cities, city_cost)
        },
    );
    let clist = clist.iter().copied().collect::<Vec<&[u8]>>();
    (clist, ccost)
}

fn part1_2_sol<'a>(
    cities: &[&'a [u8]],
    costs: &HashMap<(&'a [u8], &'a [u8]), u64>,
) -> (Solved, Solved) {
    cities
        .iter()
        .copied()
        .permutations(cities.len())
        .flat_map(|perm| {
            perm.iter().tuple_windows().try_fold(0u64, |acc, (c1, c2)| {
                costs.get(&gen_city_pair(c1, c2)).map(|cost| acc + cost)
            })
        })
        .fold((u64::MAX, u64::MIN), |(min, max), costs| {
            if costs < min {
                (costs, max)
            } else if costs > max {
                (min, costs)
            } else {
                (min, max)
            }
        })
}

// fn part2_sol(input: Output) -> Solved {}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (cities, costs) = parse_input(&input);
    let (min, max) = part1_2_sol(&cities, &costs);
    println!("Part1: {min}, Part2: {max}");
    Ok(())
}
