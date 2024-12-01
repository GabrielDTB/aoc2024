use std::collections::HashMap;
use std::hash::Hash;

pub mod template;

// We're never going to have arrays with > u32::MAX elements,
// let alone the same element, in AOC.
pub fn count_occurrences<T>(iter: impl IntoIterator<Item = T>) -> HashMap<T, u32>
where
    T: Eq + Hash,
{
    let mut counts = HashMap::new();
    for item in iter.into_iter() {
        counts
            .entry(item)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    counts
}
