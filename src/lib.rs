use std::collections::{hash_map, HashMap};
use std::hash::Hash;

pub mod template;

/// Counts how many times it has seen a certain element.
pub struct Counter<T> {
    // We're never going to have arrays with > u32::MAX elements,
    // let alone the same element, in AOC.
    state: HashMap<T, u32>,
}

impl<T> Counter<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        let state = HashMap::new();
        Self { state }
    }
    pub fn get(&self, item: &T) -> u32 {
        match self.state.get(item) {
            Some(&count) => count,
            None => 0,
        }
    }
    pub fn add(&mut self, item: T) {
        self.state
            .entry(item)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    pub fn add_all<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in items.into_iter() {
            self.add(item);
        }
    }
    pub fn iter(&self) -> hash_map::Iter<T, u32> {
        self.state.iter()
    }
    pub fn iter_mut(&mut self) -> hash_map::IterMut<T, u32> {
        self.state.iter_mut()
    }
}

impl<T> FromIterator<T> for Counter<T>
where
    T: Eq + Hash,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut counter = Counter::new();
        counter.add_all(iter);
        counter
    }
}

impl<T> IntoIterator for Counter<T>
where
    T: Eq + Hash,
{
    type Item = (T, u32);
    type IntoIter = hash_map::IntoIter<T, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.state.into_iter()
    }
}
