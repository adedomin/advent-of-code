use aoc_shared::{destructure_or_none, read_input, GroupTokenize, Token};
use std::{
    collections::{HashMap, HashSet},
    io,
};

type Output = Vec<Vec<usize>>;

fn parse_input(input: &[u8]) -> Output {
    let mut ident = 0;
    let mut ident_map = HashMap::new();
    let v_e = input
        .group_tokens(Token::Newline)
        .map(|tok| {
            tok.into_iter()
                .flat_map(|word| destructure_or_none!(Token::Something|word| = word))
                .map(|word| {
                    if let Some(id) = ident_map.get(word) {
                        *id
                    } else {
                        let new_id = ident;
                        ident_map.insert(word, new_id);
                        ident += 1;
                        new_id
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    // SEE: y2023/input/25.todot.bash | dot -Tsvg -Kneato > 25.svg && firefox 25.svg
    let mut graph = vec![HashSet::new(); ident];
    // filters 3-node "bridge"
    v_e.into_iter().for_each(|vert| {
        if vert.is_empty() {
            return;
        }
        let (root, conns) = vert.split_at(1);
        let root = root[0];
        graph[root].extend(conns.iter());
        conns.into_iter().for_each(|&conn| {
            graph[conn].insert(root);
        });
    });
    let mut graph = graph
        .into_iter()
        .map(|hset| hset.into_iter().collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();
    [
        (
            *ident_map.get(&b"cvx"[..]).unwrap(),
            *ident_map.get(&b"tvj"[..]).unwrap(),
        ),
        (
            *ident_map.get(&b"spx"[..]).unwrap(),
            *ident_map.get(&b"fsv"[..]).unwrap(),
        ),
        (
            *ident_map.get(&b"kdk"[..]).unwrap(),
            *ident_map.get(&b"nct"[..]).unwrap(),
        ),
    ]
    .into_iter()
    .for_each(|(v1, v2)| {
        let i1 = graph[v1].iter().position(|v| *v == v2).unwrap();
        graph[v1].swap_remove(i1);
        let i2 = graph[v2].iter().position(|v| *v == v1).unwrap();
        graph[v2].swap_remove(i2);
    });
    graph
}

fn solve(i: Vec<Vec<usize>>) -> usize {
    let mut seen = vec![false; i.len()];
    let mut stack = vec![0];
    while let Some(v) = stack.pop() {
        if seen[v] {
            continue;
        }

        seen[v] = true;
        stack.extend_from_slice(&i[v]);
    }
    println!("{seen:?}");
    let vis = seen.into_iter().filter(|s| *s).count();
    i.len().abs_diff(vis) * vis
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed_input = parse_input(&input);
    let part1 = solve(parsed_input);
    print!("Part1: {part1}, ");
    // print!("Part2: {part2}");
    println!();
    Ok(())
}
