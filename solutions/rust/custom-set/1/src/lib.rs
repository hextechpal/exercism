use std::{collections::HashMap, hash::Hash};

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: Eq + Hash + Clone> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    map: HashMap<T, bool>,
}

impl<T: Hash + Eq + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut map: HashMap<T, bool> = HashMap::new();
        for t in input {
            map.entry(t.clone()).insert_entry(true);
        }
        Self { map }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.map.contains_key(element)
    }

    pub fn add(&mut self, element: T) {
        self.map.entry(element).or_insert(true);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.map.iter().all(|(t, _)| other.contains(t))
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !other.map.iter().any(|(t, _)| self.contains(t))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let map = self
            .map
            .iter()
            .filter(|&(t, _)| other.contains(t))
            .map(|(t, b)| (t.clone(), b.clone()))
            .collect();
        Self { map }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let f: HashMap<T, bool> = self
            .map
            .iter()
            .filter(|&(t, _)| !other.contains(t))
            .map(|(t, b)| (t.clone(), b.clone()))
            .collect();

        Self { map: f }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut first = self.map.clone();
        first.extend(other.map.clone());
        Self { map: first }
    }
}
