#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    // Check for equality
    if first_list.len() == second_list.len() {
        let mut equal = true;
        for i in 0..first_list.len() {
            if first_list[i] != second_list[i] {
                equal = false;
                break;
            }
        }
        if equal {
            return Comparison::Equal;
        }
    }

    // Helper to check if `small` is a sublist of `large`
    fn is_sublist(small: &[i32], large: &[i32]) -> bool {
        if small.is_empty() {
            return true;
        }
        if small.len() > large.len() {
            return false;
        }
        for start in 0..=(large.len() - small.len()) {
            let mut match_all = true;
            for i in 0..small.len() {
                if large[start + i] != small[i] {
                    match_all = false;
                    break;
                }
            }
            if match_all {
                return true;
            }
        }
        false
    }

    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }
    if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}
