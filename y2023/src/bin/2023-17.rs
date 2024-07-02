use aoc_shared::{parse_to_flat2d, read_input, FlatVec2D};
use std::{
    collections::{BinaryHeap, HashMap},
    io,
};

type Pxy = (usize, usize);
type Pixy = (isize, isize);
type Vxy = (i8, i8);
type Key = (Pxy, Pxy, Vxy);
type DistMap = HashMap<Key, u64>;

fn get_neigh(x: isize, y: isize, lx: isize, ly: isize, ix: i8, iy: i8) -> [(Pixy, Pixy, Vxy); 3] {
    match (ix, iy) {
        (xdir, 0) => [
            ((x, y - 1), (x, y - 1), (0, -1)),
            ((x + xdir as isize, y), (lx, ly), (ix, iy)),
            ((x, y + 1), (x, y + 1), (0, 1)),
        ],
        (0, ydir) => [
            ((x - 1, y), (x - 1, y), (-1, 0)),
            ((x, y + ydir as isize), (lx, ly), (ix, iy)),
            ((x + 1, y), (x + 1, y), (1, 0)),
        ],
        _ => unreachable!(),
    }
}

/// We need a custom Ord & PartialOrd to make BinaryHeap a minheap.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct HeapState {
    state: Key,
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

fn get_dist(dist: &DistMap, key: &Key) -> u64 {
    *dist.get(key).unwrap_or(&u64::MAX)
}

fn cmp_and_swap_dist(dist: &mut DistMap, key: Key, new_cost: u64) -> bool {
    if let Some(swapped) = dist.get_mut(&key).map(|old_cost| {
        if new_cost < *old_cost {
            *old_cost = new_cost;
            true
        } else {
            false
        }
    }) {
        swapped
    } else {
        dist.insert(key, new_cost);
        true
    }
}

fn dist_cmp(ld: usize, old: usize, turned: bool, is_ultra: bool) -> bool {
    if is_ultra {
        if turned {
            old > 2
        } else {
            old < 9
        }
    } else {
        ld < 3
    }
}

fn solve(
    nodes: &FlatVec2D<u8>,
    start: (usize, usize),
    end: (usize, usize),
    ultra_crucible: bool,
) -> Option<u64> {
    let mut dist: DistMap = HashMap::new();
    let mut heap = BinaryHeap::new();
    dist.insert((start, start, (1i8, 0i8)), 0u64);
    dist.insert((start, start, (0i8, 1i8)), 0u64);

    heap.push(HeapState {
        state: (start, start, (1, 0)),
        cost: 0, // we ignore cost of starting in this node
    });
    heap.push(HeapState {
        state: (start, start, (0, 1)),
        cost: 0, // we ignore cost of starting in this node
    });

    while let Some(HeapState {
        state: ((x, y), (lx, ly), (ix, iy)),
        cost,
    }) = heap.pop()
    {
        if (x, y) == end {
            return Some(cost);
        } else if cost > get_dist(&dist, &((x, y), (lx, ly), (ix, iy))) {
            continue;
        }

        get_neigh(x as isize, y as isize, lx as isize, ly as isize, ix, iy)
            .iter()
            .filter(|((x1, y1), _, _)| nodes.in_bounds(*x1, *y1))
            .for_each(|&((x1, y1), (lx1, ly1), (ix1, iy1))| {
                let x1 = x1 as usize;
                let y1 = y1 as usize;
                let lx1 = lx1 as usize;
                let ly1 = ly1 as usize;

                let line_dist = std::cmp::max(x1.abs_diff(lx1), y1.abs_diff(ly1));
                let oline_dist = std::cmp::max(x.abs_diff(lx), y.abs_diff(ly));
                let turned = !(lx == lx1 && ly == ly1);

                let neigh_cost = (nodes[(x1, y1)] - b'0') as u64;
                let new_cost = if dist_cmp(line_dist, oline_dist, turned, ultra_crucible) {
                    cost + neigh_cost
                } else {
                    u64::MAX
                };

                let key = ((x1, y1), (lx1, ly1), (ix1, iy1));
                if cmp_and_swap_dist(&mut dist, key, new_cost) {
                    heap.push(HeapState {
                        state: key,
                        cost: new_cost,
                    });
                }
            });
    }
    None
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let grid = parse_to_flat2d(&input);
    let part1 =
        solve(&grid, (0, 0), (grid.1 - 1, grid.2 - 1), false).expect("expected to find min path.");
    let part2 =
        solve(&grid, (0, 0), (grid.1 - 1, grid.2 - 1), true).expect("expected to find min path.");
    print!("Part1: {part1}, ");
    print!("Part2: {part2}");
    println!();
    Ok(())
}
