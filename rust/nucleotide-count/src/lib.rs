use std::collections::HashMap;

const AMINOS:[char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !AMINOS.contains(&nucleotide) { return Err(nucleotide); }

    match dna.chars().find(|ch| !AMINOS.contains(ch)) {
        Some(x) => Err(x),
        None => Ok(dna.chars().filter(|&ch| ch == nucleotide).count())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut h:HashMap<char, usize> = HashMap::new();
    
    for amino in AMINOS {
        h.insert(amino, 0);
    }

    for ch in dna.chars() {
        if !h.contains_key(&ch) { return Err(ch); }
        h.entry(ch).and_modify(|n| *n += 1);
    }

    Ok(h)
}