use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    // Remove any '-' or whitespance
    let candidate = candidate.replace("-", "").replace(" ", "").to_lowercase();

    // The hashset will store all UNIQUE letters
    let mut letter: HashSet<char> = HashSet::new();

    // The .insert() method returns false if it can not insert the value, which
    // happens if it's already in there.  Otherwise, it returns true
    candidate.chars().all(|x| letter.insert(x))
}