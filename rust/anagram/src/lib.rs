use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result_set: HashSet<&str> = HashSet::new();
    
    let mut word_vec: Vec<char> = word.to_lowercase().chars().collect();
    word_vec.sort();

    for res in possible_anagrams {
        let mut poss_vec = Vec::new();

        for c in res.to_lowercase().chars() {
            poss_vec.push(c);
        }
        poss_vec.sort();

        if word.to_lowercase() != res.to_lowercase() && poss_vec == word_vec {
            result_set.insert(res);
        }
    }
    result_set
}