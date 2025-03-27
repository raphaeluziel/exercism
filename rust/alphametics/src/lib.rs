use itertools::{Itertools, Permutations};
use std::collections::{BTreeMap, HashMap, HashSet};

// Function that turns words into numbers using the BTreeMap values
fn word_as_num(s: &str, bm: &BTreeMap<char, u8>) -> u64 {
    s.chars().rev().enumerate().fold(0, |acc, (pw, ch)| {
        acc + (*bm.get(&ch).unwrap_or(&0) as u64) * 10u64.pow(pw as u32)
    })
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Remove all extraneous characters from the input
    let input = input.replace(" ", "").replace("==", "=");

    // Collect first letters into a set to make sure these don't get a 0n
    let firsts: HashSet<char> = input
        .split(['+', '='])
        .map(|x| x.chars().next().unwrap_or_default())
        .collect();

    // Create a BTreeMap using the chars of the input
    let mut bm: BTreeMap<char, u8> = BTreeMap::from_iter(
        input
            .char_indices()
            .filter(|&x| x.1 != '+' && x.1 != '=')
            .map(|x| (x.1, 2)),
    );

    // Get the words, later to be converted to numbers
    let mut words: Vec<&str> = input.split(['+', '=']).collect();
    let last_word = words.pop().unwrap_or_default();

    // Create a permutation using all digits
    let perms: Permutations<std::ops::Range<u8>> = (0..10).permutations(bm.len());

	// Flag to see if after all the permutations, a combination was found
	let mut found = false;

    for perm in perms {
        // Fill the BTreeMap with values of the permutation
        bm = bm
            .iter_mut()
            .enumerate()
            .map(|x| (*x.1.0, perm[x.0]))
            .collect();

        // If any first letters in bm have 0 as a value, go to the next permutation
        if bm.iter().any(|(k, v)| firsts.contains(k) && *v == 0) {
            continue;
        }

		// Calculate the total of all the words to be summed
		// then check to see if the sum is valid
        let mut total = 0u64;
        for word in &words {
            total += word_as_num(word, &bm);
        }
        if total == word_as_num(last_word, &bm) {
			found = true;
            break;
        }
    }

	match found {
		true => Some(bm.clone().into_iter().collect()),
		false => None
	}
}