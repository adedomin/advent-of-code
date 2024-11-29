/// Minimum heap key state for BinaryHeap
/// Inverts Ord/PartialOrd for the cost field for Djikstra problems.
#[derive(Clone, PartialEq, Eq)]
pub struct HeapState<K, C>
where
    K: Clone + PartialEq + Eq,
    C: PartialOrd + Ord,
{
    pub key: K,
    pub cost: C,
}

impl<K, C> Ord for HeapState<K, C>
where
    K: Clone + PartialEq + Eq,
    C: PartialOrd + Ord,
{
    /// Reverse of the Ord impl for min heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<K, C> PartialOrd for HeapState<K, C>
where
    K: Clone + PartialEq + Eq,
    C: PartialOrd + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
