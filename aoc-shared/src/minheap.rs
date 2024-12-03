use std::collections::{BinaryHeap, HashMap};

/// Minimum heap key state for BinaryHeap
/// Inverts Ord/PartialOrd for the cost field for Dijkstra problems.
#[derive(Clone, PartialEq, Eq)]
pub struct HeapState<K, C>
where
    K: Clone + PartialEq + Eq + std::hash::Hash,
    C: Copy + PartialOrd + Ord,
{
    pub key: K,
    pub cost: C,
}

impl<K, C> Ord for HeapState<K, C>
where
    K: Clone + PartialEq + Eq + std::hash::Hash,
    C: Copy + PartialOrd + Ord,
{
    /// Reverse of the Ord impl for min heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<K, C> PartialOrd for HeapState<K, C>
where
    K: Clone + PartialEq + Eq + std::hash::Hash,
    C: Copy + PartialOrd + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Dijkstra<K, C>
where
    K: Clone + PartialEq + Eq + std::hash::Hash,
    C: Copy + PartialOrd + Ord,
{
    heap: BinaryHeap<HeapState<K, C>>,
    distmap: HashMap<K, C>,
}

impl<K, C> Dijkstra<K, C>
where
    K: Clone + PartialEq + Eq + std::hash::Hash,
    C: Copy + PartialOrd + Ord,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Dijkstra {
            heap: BinaryHeap::new(),
            distmap: HashMap::new(),
        }
    }

    pub fn pop(&mut self) -> Option<HeapState<K, C>> {
        self.heap.pop()
    }

    pub fn push(&mut self, key: K, cost: C) {
        self.push_bool(key, cost);
    }

    pub fn push_bool(&mut self, key: K, cost: C) -> bool {
        let mut changed = true;
        let dent = self
            .distmap
            .entry(key.clone())
            .and_modify(|old| {
                if cost < *old {
                    *old = cost
                } else {
                    changed = false;
                }
            })
            .or_insert(cost);
        if *dent == cost && changed {
            self.heap.push(HeapState { key, cost });
            true
        } else {
            false
        }
    }
}

pub struct DijkstraPath<K, C>
where
    K: Clone + PartialEq + Eq + std::hash::Hash,
    C: Copy + PartialOrd + Ord,
{
    dij: Dijkstra<K, C>,
    pathmap: HashMap<K, K>,
}

impl<K, C> DijkstraPath<K, C>
where
    K: Clone + PartialEq + Eq + std::hash::Hash,
    C: Copy + PartialOrd + Ord,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        DijkstraPath {
            dij: Dijkstra::new(),
            pathmap: HashMap::new(),
        }
    }

    pub fn pop(&mut self) -> Option<HeapState<K, C>> {
        self.dij.pop()
    }

    pub fn push(&mut self, key: K, oldkey: K, cost: C) {
        if self.dij.push_bool(key.clone(), cost) {
            self.pathmap.insert(key, oldkey);
        }
    }

    pub fn push_init(&mut self, key: K, cost: C) {
        self.dij.push(key.clone(), cost);
    }

    pub fn path_from<'a>(&'a self, start: &'a K) -> DijkstraPathIter<'a, K> {
        DijkstraPathIter {
            first: true,
            start,
            pathmap: &self.pathmap,
        }
    }
}

pub struct DijkstraPathIter<'a, K>
where
    K: Eq + std::hash::Hash,
{
    first: bool,
    start: &'a K,
    pathmap: &'a HashMap<K, K>,
}

impl<'a, K> Iterator for DijkstraPathIter<'a, K>
where
    K: Eq + std::hash::Hash,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.start);
        }
        self.start = self.pathmap.get(self.start)?;
        Some(self.start)
    }
}

#[cfg(test)]
mod test {
    use super::{DijkstraPath, HeapState};

    type Key = (i32, i32);
    type Cost = u32;
    const TARGET: Key = (5, 5);

    #[test]
    fn dij_path_test() {
        let mut dij = DijkstraPath::<Key, Cost>::new();
        dij.push_init((0, 0), 0);
        while let Some(HeapState { key, cost }) = dij.pop() {
            if key == TARGET {
                let path = dij.path_from(&key).copied().collect::<Vec<Key>>();
                assert_eq!(path, vec![(5, 5), (4, 4), (3, 3), (2, 2), (1, 1), (0, 0)]);
                return;
            }
            let (x, y) = key;

            dij.push((x - 1, y - 1), key, cost + 1);
            dij.push((x, y - 1), key, cost + 1);
            dij.push((x + 1, y - 1), key, cost + 1);
            dij.push((x - 1, y), key, cost + 1);
            dij.push((x + 1, y), key, cost + 1);
            dij.push((x - 1, y + 1), key, cost + 1);
            dij.push((x, y + 1), key, cost + 1);
            dij.push((x + 1, y + 1), key, cost + 1);
        }
        unreachable!();
    }
}
