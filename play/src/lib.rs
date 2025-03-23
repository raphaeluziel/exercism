use itertools::{Itertools, Permutations};
use std::collections::{BTreeMap, HashMap, HashSet, btree_set::Range};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input = input.replace(" ", "").replace("==", "=");
    let input: Vec<(char, u8)> = input
        .char_indices()
        .filter(|&x| x.1 != '+' && x.1 != '=')
        .map(|x| (x.1, (x.0 as u8)))
        .collect();

    let mut bm: BTreeMap<char, u8> = BTreeMap::from_iter(input);

    let mut iterations = 0;
    let perms: Permutations<std::ops::Range<u8>> = (0..10).permutations(bm.len());
    for perm in perms {
        for b in bm.iter_mut().enumerate() {
            *b.1.1 = perm[b.0];
        }
        iterations += 1;
    }
    println!("Iterations = {iterations}");

    //Some(bm.clone().into_iter().map(|(k, v)| (k, v)).collect())
    todo!("HEY")
}
