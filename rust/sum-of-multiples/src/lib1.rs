use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set: HashSet<u32> = HashSet::new();
    for n in factors {
        if *n == 0 { continue; }
        let mut i = 1;
        while n * i < limit {
            set.insert(n * i);
            i += 1;
        }
    }
    set.iter().sum()
}