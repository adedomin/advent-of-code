use std::collections::{BinaryHeap, HashMap};

/// Minimum heap key state for BinaryHeap
/// Inverts Ord/PartialOrd for the cost field for Djikstra problems.
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
    distmap: HashMap<K, Option<C>>,
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
        let dent = self.distmap.entry(key.clone()).or_insert(None);
        match *dent {
            Some(val) if cost < val => {
                *dent = Some(cost);
                self.heap.push(HeapState { key, cost });
            }
            None => {
                *dent = Some(cost);
                self.heap.push(HeapState { key, cost });
            }
            _ => (),
        }
    }
}
