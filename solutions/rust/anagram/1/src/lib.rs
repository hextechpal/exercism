use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    fn counter(word: &str) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        // Handle lowercase conversion correctly across Unicode
        for c in word.chars().flat_map(|ch| ch.to_lowercase()) {
            *counts.entry(c).or_insert(0) += 1;
        }
        counts
    }

    let word_lower = word.to_lowercase();
    let word_counter = counter(&word_lower);

    possible_anagrams
        .iter()
        .copied()
        .filter(|candidate| {
            let candidate_lower: String =
                candidate.chars().flat_map(|c| c.to_lowercase()).collect();

            // Exclude identical words (case-insensitive)
            if candidate_lower == word_lower {
                return false;
            }

            counter(&candidate_lower) == word_counter
        })
        .collect()
}
