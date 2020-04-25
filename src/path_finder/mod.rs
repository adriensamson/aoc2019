use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

pub trait PathState: Sized {
    type HashKey: Hash + Eq;
    fn is_finished(&self) -> bool;
    fn get_next_states(&self) -> Vec<Self>;
    fn get_hash_key(&self) -> Self::HashKey;
    fn distance(&self) -> usize;
}

struct DistWrapper<T: PathState>(T);

impl<T: PathState> Ord for DistWrapper<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.distance().cmp(&other.0.distance()).reverse()
    }
}

impl<T: PathState> PartialOrd for DistWrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: PathState> PartialEq for DistWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.distance() == other.0.distance()
    }
}

impl<T: PathState> Eq for DistWrapper<T> {}

pub fn find_shortest_path<P: PathState>(start: P) -> Option<P> {
    let mut preferred_by_hash_key: HashMap<P::HashKey, usize> = HashMap::new();
    preferred_by_hash_key.insert(start.get_hash_key(), 0);
    let mut paths = BinaryHeap::new();
    paths.push(DistWrapper(start));
    let mut found = BinaryHeap::new();

    loop {
        let ep = &paths.pop().unwrap();
        for next in ep.0.get_next_states() {
            let hash_key = next.get_hash_key();
            let min_by_has_key = preferred_by_hash_key
                .entry(hash_key)
                .or_insert(next.distance() + 1);
            if next.is_finished() {
                found.push(DistWrapper(next));
            } else if next.distance() < *min_by_has_key {
                *min_by_has_key = next.distance();
                paths.push(DistWrapper(next));
            }
        }
        if paths.is_empty() {
            break;
        }
        if let (Some(sp), Some(np)) = (found.peek(), paths.peek()) {
            if sp.0.distance() < np.0.distance() {
                break;
            }
        }
    }

    found.pop().map(|w| w.0)
}
