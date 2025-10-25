use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut set = HashSet::new();

    for b in sentence.as_bytes() {
        if (*b >= b'a' && *b <= b'z') || (*b >= b'A' && *b <= b'Z') {
            let c = (*b as char).to_ascii_lowercase();
            set.insert(c);
        }
    }

    set.len() == 26
}
