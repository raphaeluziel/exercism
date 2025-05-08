// This iteration removes the needless hashmap I used before, which I think
// caused the slowness of the program when benchmarked.
// The benchmark test went from about 1200 ns to about 200 ns!


// After seeing some other approaches, I learned that I can use bitwise OR
// which removes the need for checking duplicates (which was the reason for
// using the hashmap)

// Still twice as slow as the given bitfield approach exercism wrote.

const MAX_26_BITS: u32 = 67_108_863;

pub fn is_pangram(sentence: &str) -> bool {

    sentence.to_ascii_lowercase()
            .as_bytes()
            .iter()
            .filter(|&&x| x.is_ascii_lowercase())
            .map(|x| 2u32.pow((x - b'a') as u32))
            .fold(0, |acc, x| acc | x) == MAX_26_BITS
}