/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() { return None; }

    Some(s1.chars().into_iter()
           .zip(s2.chars().into_iter())
           .filter(|(c1, c2)| c1 != c2)
           .count())
}
