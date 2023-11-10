use std::{collections::BinaryHeap, io};
use aoc_shared::{read_input, FlatVec2D};

const HEIGHT_START: u8 = b'a';
const HEIGHT_END: u8 = b'z';

fn parse_input(input: Vec<u8>) -> ((usize, usize), (usize, usize), FlatVec2D<u8>) {
    let row_width = input.iter().position(|&chr| chr == b'\n').unwrap();
    let col_len = ((input.len() - 1) / (row_width + 1)) + 1;

    let mut ret = FlatVec2D(vec![255; row_width * col_len], row_width, col_len);

    let mut i = 0;
    let mut j = 0;
    let mut start = (0, 0);
    let mut end = (0, 0);
    input.iter().for_each(|&el| {
        if el == b'\n' {
            i = 0;
            j += 1;
        } else {
            if el == b'S' {
                start = (i, j);
                ret[start] = 0;
            } else if el == b'E' {
                end = (i, j);
                ret[end] = HEIGHT_END - HEIGHT_START;
            } else {
                ret[(i, j)] = el - HEIGHT_START;
            }
            i += 1;
        }
    });

    (start, end, ret)
}

fn get_neigh(x: usize, y: usize, xmax: usize, ymax: usize) -> Vec<(usize, usize)> {
    let mut ret = Vec::with_capacity(4);
    if y != 0 {
        ret.push((x, y - 1));
    }

    if x != 0 {
        ret.push((x - 1, y));
    }

    if y + 1 != ymax {
        ret.push((x, y + 1));
    }

    if x + 1 != xmax {
        ret.push((x + 1, y));
    }

    ret
}

/// We define this struct only because BinaryHeap is a Max heap only;
/// so we need a custom Ord & PartialOrd to make BinaryHeap more like
/// a min heap.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct HeapState {
    node_idx: (usize, usize),
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

fn solve(start: (usize, usize), end: (usize, usize), nodes: &FlatVec2D<u8>) -> Option<u64> {
    let mut dist: FlatVec2D<u64> = FlatVec2D(vec![u64::MAX; nodes.0.len()], nodes.1, nodes.2);
    let mut heap = BinaryHeap::new();
    dist[start] = 0;

    heap.push(HeapState {
        node_idx: start, // top-left
        cost: 0,         // we ignore cost of starting in this node
    });

    while let Some(HeapState {
        node_idx: (x, y),
        cost,
    }) = heap.pop()
    {
        if (x, y) == end {
            return Some(cost);
        } else if cost > dist[(x, y)] {
            continue;
        }

        get_neigh(x, y, nodes.1, nodes.2).iter().for_each(|&neigh| {
            let height = nodes[(x, y)];
            let neigh_height = nodes[neigh];
            let hdiff = height.abs_diff(neigh_height);
            let new_cost = if hdiff < 2 || neigh_height < height {
                cost + 1
            } else {
                u64::MAX
            };
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

fn main() -> io::Result<()> {
    let input = read_input()?;
    let (start, end, parsed_input) = parse_input(input);
    let part1 =
        solve(start, end, &parsed_input).expect("Could not find a low cost route from S to E.");
    let part2 = parsed_input
        .0
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, node)| *node == 0)
        .map(|(start, _)| {
            let x = start % parsed_input.1;
            let y = start / parsed_input.1;
            (x, y)
        })
        .flat_map(|start| solve(start, end, &parsed_input))
        .min()
        .expect("Could not find ANY route from any S (a elevation) to E.");
    println!("Part1: {part1}, Part2: {part2}");
    Ok(())
}
