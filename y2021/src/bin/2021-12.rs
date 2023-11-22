use std::{fmt::Debug, io, vec};

use aoc_shared::{read_input, AoCTokenizer, Token};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CaveSize {
    Small,
    Large,
}

const START: u64 = u64::MAX - 1;
const END: u64 = u64::MAX;

#[derive(Clone)]
struct GraphNode {
    ident: u64,
    cave: CaveSize,
    nodes: Vec<usize>,
}

impl Default for GraphNode {
    fn default() -> Self {
        GraphNode {
            ident: u64::MAX - 1,
            cave: CaveSize::Small,
            nodes: vec![],
        }
    }
}

impl Debug for GraphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ident == START {
            f.write_str("start")
        } else if self.ident == END {
            f.write_str("end")
        } else {
            let id1 = (self.ident >> 8) as u8;
            let id2 = (self.ident & 0b1111_1111) as u8;
            let node_name = String::from_utf8_lossy(&[id2, id1]).to_string();
            f.write_str(&node_name)
        }
    }
}

fn calc_map_ident(id: &[u8]) -> (CaveSize, u64) {
    let ident = if id.eq(b"start") {
        u64::MAX - 1
    } else if id.eq(b"end") {
        u64::MAX
    } else {
        id.iter()
            .take(8)
            .fold((0u64, 0u8), |(acc, idx), &digit| {
                (acc | ((digit as u64) << (idx as u64 * 8)), idx + 1)
            })
            .0
    };
    let cave_size = if id[0].is_ascii_lowercase() {
        CaveSize::Small
    } else {
        CaveSize::Large
    };
    (cave_size, ident)
}

fn parse(input: Vec<u8>) -> Vec<GraphNode> {
    let mut nodes = vec![
        GraphNode::default(),
        GraphNode {
            ident: u64::MAX,
            cave: CaveSize::Small,
            nodes: vec![],
        },
    ];
    let mut parent_idx = 0usize;
    let mut sep = false;
    for token in AoCTokenizer::new(&input) {
        match token {
            Token::Something(ident_str) if !sep => {
                let (cave, ident) = calc_map_ident(ident_str);
                parent_idx = if let Some(idx) = nodes.iter().position(|n| n.ident == ident) {
                    idx
                } else {
                    nodes.push(GraphNode {
                        ident,
                        cave,
                        nodes: vec![],
                    });
                    nodes.len() - 1
                };
            }
            Token::Something(ident_str) => {
                let (cave, ident) = calc_map_ident(ident_str);
                if let Some(idx) = nodes.iter().position(|n| n.ident == ident) {
                    if !nodes[parent_idx].nodes.contains(&idx) {
                        nodes[parent_idx].nodes.push(idx);
                    }
                    if parent_idx != 0 && !nodes[idx].nodes.contains(&parent_idx) {
                        nodes[idx].nodes.push(parent_idx);
                    }
                } else {
                    nodes.push(GraphNode {
                        ident,
                        cave,
                        nodes: vec![parent_idx],
                    });
                    let idx = nodes.len() - 1;
                    if !nodes[parent_idx].nodes.contains(&idx) {
                        nodes[parent_idx].nodes.push(idx);
                    }
                };
            }
            Token::Delimiter(_) => {
                sep = true;
            }
            Token::Newline | Token::DoubleNewline => {
                sep = false;
            }
            Token::Space => (),
            Token::End => (),
        }
    }
    nodes
}

fn solve(nodes: &[GraphNode], twice: Option<usize>) -> u64 {
    let mut visit_map = vec![0; nodes.len()];
    let mut stack = vec![(0usize, 0usize, &nodes[0])];
    let mut paths = 0;

    while let Some((idx, nidx, node)) = stack.pop() {
        if node.ident == END {
            //for (_, _, item) in &stack {
            //    print!("{:?},", item);
            //}
            //println!("end");
            paths += 1;
            continue;
        }

        let mut i = nidx;
        while i < node.nodes.len() {
            let neigh = node.nodes[i];
            if visit_map[neigh] < 1 {
                break;
            } else if let Some(t) = twice {
                if t == neigh && visit_map[neigh] < 2 {
                    break;
                }
            }
            i += 1;
        }

        if i < node.nodes.len() {
            if node.cave == CaveSize::Small && nidx == 0 {
                visit_map[idx] += 1;
            }
            stack.push((idx, i + 1, node));
            let neigh_node = node.nodes[i];
            stack.push((neigh_node, 0usize, &nodes[neigh_node]));
        } else if node.cave == CaveSize::Small {
            visit_map[idx] -= 1;
        }
    }
    paths
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let root = parse(input);
    let p1 = solve(&root, None);
    let p2 = root
        .iter()
        .enumerate()
        .filter(|(_, n)| n.cave == CaveSize::Small && n.ident != START && n.ident != END)
        .fold(0u64, |acc, (idx, _)| (acc + solve(&root, Some(idx))) - p1);
    println!("Part1 {}, Part2 {}", p1, p1 + p2);
    Ok(())
}
