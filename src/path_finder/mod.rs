use std::hash::Hash;
use std::collections::{BinaryHeap, HashMap};

pub trait PathState : Ord + Sized {
    type HashKey : Hash + Eq;
    fn is_finished(&self) -> bool;
    fn get_next_states(&self) -> Vec<Self>;
    fn get_hash_key(&self) -> Self::HashKey;
    fn distance(&self) -> usize;
}

pub fn find_shortest_path<P : PathState>(start : P) -> Option<P> {
    let mut paths = BinaryHeap::new();
    paths.push(start);
    let mut found = BinaryHeap::new();
    let mut preferred_by_hash_key: HashMap<P::HashKey, usize> = HashMap::new();

    loop {
        let ep = &paths.pop().unwrap();
        for next in ep.get_next_states() {
            let hash_key = next.get_hash_key();
            let min_by_has_key = preferred_by_hash_key.entry(hash_key).or_insert(next.distance() + 1);
            if next.is_finished() {
                found.push(next);
            } else if next.distance() < *min_by_has_key {
                *min_by_has_key = next.distance();
                paths.push(next);
            }
        }
        if paths.len() == 0 {
            break;
        }
        if let (Some(sp), Some(np)) = (found.peek(), paths.peek()) {
            if sp.distance() < np.distance() {
                break;
            }
        }
    };

    found.pop()
}