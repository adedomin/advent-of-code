use aoc_shared::read_input_to_string;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::BinaryHeap, io};

type Output<'a> = FxHashMap<&'a str, FxHashSet<&'a str>>;

// Generates a pair in lexicographic order.
fn gen_lexographic_pair(mut a: [&str; 3]) -> [&str; 3] {
    a.sort();
    a
}
fn parse_input(input: &str) -> Output<'_> {
    let mut verts: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();
    input
        .split(|c: char| !c.is_ascii_alphanumeric())
        .tuples()
        .for_each(|(a, b)| {
            verts.entry(a).or_default().insert(b);
            verts.entry(b).or_default().insert(a);
        });
    verts
}

fn cliques3<'a>(verts: &'a Output<'a>) -> impl Iterator<Item = [&'a str; 3]> {
    verts
        .iter()
        .flat_map(|(e, v)| {
            v.iter().flat_map(|e2| {
                let v2 = verts.get(e2).expect("should have a reverse mapping.");
                v2.iter()
                    .filter_map(|e3| verts.get(e3)?.contains(e).then_some(e3))
                    .map(|e3| gen_lexographic_pair([e, e2, e3]))
            })
        })
        .unique()
}

fn part1_sol(verts: &Output) -> usize {
    cliques3(verts)
        .filter(|clique| clique.iter().any(|e| e.starts_with("t")))
        .count()
}

fn part2_sol(verts: &Output) -> String {
    let mut counts: FxHashMap<&str, i32> = FxHashMap::default();
    cliques3(verts).for_each(|ring| {
        ring.iter().for_each(|e| {
            *counts.entry(e).or_default() += 1;
        });
    });
    // find the edges with the most representation in the 3 ring subgraphs...
    let max_edges = counts
        .drain()
        .max_set_by_key(|(_, v)| *v)
        .into_iter()
        .map(|(e, _)| e)
        .collect::<FxHashSet<_>>();
    // recreate the verticies with only the max edges.
    let max_verts = max_edges
        .iter()
        .map(|&e| {
            let adj = verts
                .get(e)
                .map(|adj| max_edges.intersection(adj))
                .unwrap()
                .cloned()
                .collect::<FxHashSet<_>>();
            (e, adj)
        })
        .collect::<Output>();
    // get 3-cliques in this new subgraph of max connected.
    let mut password = cliques3(&max_verts)
        .flat_map(|ring| ring.into_iter())
        .unique()
        .collect_vec();
    // we should only have the dominating clique?
    password.sort();
    password.into_iter().join(",")
}

fn main() -> io::Result<()> {
    let input = read_input_to_string()?;
    let parsed_input = parse_input(&input);
    let part1 = part1_sol(&parsed_input);
    let part2 = part2_sol(&parsed_input);
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
