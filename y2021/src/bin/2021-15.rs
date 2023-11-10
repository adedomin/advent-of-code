use std::{collections::BinaryHeap, io};

use aoc_shared::read_input;

#[derive(Clone, Copy, Default)]
struct Nodes {
    cost: u64,
    north: Option<usize>,
    east: Option<usize>,
    south: Option<usize>,
    west: Option<usize>,
}

fn xy(x: usize, y: usize, xmax: usize) -> usize {
    x + y * xmax
}

fn parse(input: &[u8]) -> Vec<Nodes> {
    let xmax = input
        .iter()
        .position(|&chr| chr == b'\n')
        .expect("Expected the input to have a newline.");
    let ymax = ((input.len() - 1) / (xmax + 1)) + 1;

    let mut y = 0usize;
    let mut x = 0usize;
    let mut nodes = vec![Nodes::default(); xmax * ymax];
    input.iter().for_each(|&chr| match chr {
        b'\n' => {
            y += 1;
            x = 0;
        }
        c => {
            let node = &mut nodes[xy(x, y, xmax)];

            node.cost = (c - b'0') as u64;
            if y > 0 {
                node.north = Some(xy(x, y - 1, xmax));
            }
            if x + 1 < xmax {
                node.east = Some(xy(x + 1, y, xmax));
            }
            if y + 1 < ymax {
                node.south = Some(xy(x, y + 1, xmax));
            }
            if x > 0 {
                node.west = Some(xy(x - 1, y, xmax));
            }
            x += 1
        }
    });
    nodes
}

fn parse_p2(input: &[u8]) -> Vec<Nodes> {
    let inp_xmax = input
        .iter()
        .position(|&chr| chr == b'\n')
        .expect("Expected the input to have a newline.");
    let inp_ymax = ((input.len() - 1) / (inp_xmax + 1)) + 1;
    let xmax = inp_xmax * 5;
    let ymax = inp_ymax * 5;

    let mut y = 0usize;
    let mut x = 0usize;
    let mut nodes = vec![Nodes::default(); xmax * ymax];
    input.iter().for_each(|&chr| match chr {
        b'\n' => {
            y += 1;
            x = 0;
        }
        c => {
            for ypow in 0..5 {
                for xpow in 0..5 {
                    let x = x + (xpow * inp_xmax);
                    let y = y + (ypow * inp_ymax);
                    let cost_mag = (ypow + xpow) as u64;
                    let cost = (c - b'0') as u64 + cost_mag;
                    let node = &mut nodes[xy(x, y, xmax)];

                    node.cost = (cost / 10) + (cost % 10);
                    if y > 0 {
                        node.north = Some(xy(x, y - 1, xmax));
                    }
                    if x + 1 < xmax {
                        node.east = Some(xy(x + 1, y, xmax));
                    }
                    if y + 1 < ymax {
                        node.south = Some(xy(x, y + 1, xmax));
                    }
                    if x > 0 {
                        node.west = Some(xy(x - 1, y, xmax));
                    }
                }
            }
            x += 1
        }
    });
    nodes
}

/// We define this struct only because BinaryHeap is a Max heap only;
/// so we need a custom Ord & PartialOrd to make BinaryHeap more like
/// a min heap.
#[derive(Clone, Copy, PartialEq, Eq)]
struct HeapState {
    node_idx: usize,
    cost: u64,
}

impl Ord for HeapState {
    /// Reverse of the u64 Ord impl.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for HeapState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(nodes: Vec<Nodes>) -> Option<u64> {
    let mut dist = vec![u64::MAX; nodes.len()];
    let mut heap = BinaryHeap::new();
    dist[0] = 0;

    heap.push(HeapState {
        node_idx: 0, // top-left
        cost: 0,     // we ignore cost of starting in this node
    });

    while let Some(HeapState { node_idx, cost }) = heap.pop() {
        // bottom-right
        if node_idx == nodes.len() - 1 {
            return Some(cost);
        } else if cost > dist[node_idx] {
            continue;
        }

        [
            nodes[node_idx].north,
            nodes[node_idx].east,
            nodes[node_idx].south,
            nodes[node_idx].west,
        ]
        .iter()
        .filter(|&&neigh| neigh.is_some())
        .map(|&neigh| neigh.unwrap())
        .for_each(|neigh| {
            let new_cost = cost + nodes[neigh].cost;
            if new_cost < dist[neigh] {
                dist[neigh] = new_cost;
                heap.push(HeapState {
                    node_idx: neigh,
                    cost: new_cost,
                });
            }
        });
    }
    None
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let nodes = parse(&input);
    let nodes_p2 = parse_p2(&input);
    let p1 = solve(nodes).expect("No path for P1 found.");
    let p2 = solve(nodes_p2).expect("No path for P2 found.");
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
