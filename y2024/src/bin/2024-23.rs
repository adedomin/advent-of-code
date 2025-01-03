use aoc_shared::read_input_to_string;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::io;

type Output<'a> = FxHashMap<&'a str, FxHashSet<&'a str>>;

// Generates a pair in lexicographic order.
fn gen_lexographic_pair(mut a: [&str; 3]) -> [&str; 3] {
    a.sort_unstable();
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

fn cliques3<'a>(edges: &'a Output<'a>) -> impl Iterator<Item = [&'a str; 3]> {
    edges
        .iter()
        .flat_map(|(v, e)| {
            e.iter()
                .filter_map(|v2| edges.get(v2).map(|e2| (v2, e2)))
                .flat_map(|(v2, e2)| {
                    e2.iter()
                        .filter_map(|v3| edges.get(v3)?.contains(v).then_some(v3))
                        .map(|v3| gen_lexographic_pair([v, v2, v3]))
                })
        })
        .unique()
}

fn part1_sol(edges: &Output) -> usize {
    cliques3(edges)
        .filter(|clique| clique.iter().any(|vert| vert.starts_with("t")))
        .count()
}

fn part2_sol(edges: &Output) -> String {
    let mut counts: FxHashMap<&str, i32> = FxHashMap::default();
    // get all 3-rings and count vertex representation in these 3-rings.
    cliques3(edges).for_each(|ring| {
        ring.iter().for_each(|e| {
            *counts.entry(e).or_default() += 1;
        });
    });
    // find the vertices with the most representation in the 3 ring subgraphs...
    let max_verts = counts
        .drain()
        .max_set_by_key(|(_, v)| *v)
        .into_iter()
        .map(|(e, _)| e)
        .collect::<FxHashSet<_>>();
    // create new edges with only the vertices that are most represented in the 3-ring subgraphs.
    // when trying to find 3-rings, the members that aren't in the max-clique will be self filtered.
    let max_edges = max_verts
        .iter()
        .map(|&v| (v, edges.get(v).unwrap().clone()))
        .collect::<Output>();
    // get 3-cliques in this new subgraph of max connected.
    // even if other vertices have a high representation in other 3-cliques, this should
    // only give us a list of the max-clique we're looking for.
    // this works because the problem would be unsolvable if there were multiple,
    // equally large, cliques in the graph.
    let mut password = cliques3(&max_edges)
        .flat_map(|ring| ring.into_iter())
        .unique()
        .collect_vec();
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
