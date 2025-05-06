#![feature(test)]
extern crate test;
use test::Bencher;

use std::collections::HashSet;

// This number in binary has twenty six 1's
const MAX_26_BITS: u32 = 67_108_863;

pub fn is_pangram(sentence: &str) -> bool {
    // Using a hashset to remove duplicate letters
    let hs:HashSet<_> = sentence
        // Convert to lowercase to make it case insensitive
        .to_ascii_lowercase()
        // Convert to a slice of bytes
        .as_bytes()
        // Iterate through all the bytes
        .iter()
        // Remove anything that is not one of the 26 lowercase ASCII letters
        .filter(|x| x.is_ascii_lowercase())
        // Convert the byte representing the char to a binary with only a 
        // single 1 in the appropriate location
        .map(|x| 2u32.pow((x - b'a') as u32))
        // Collect these into the hashset
        .collect();
    
    // If all letters were present, then when summed, all bits in a 26 bit 
    // binary number will be 1's, which would equal the MAX_26_BITS
    hs.iter().sum::<u32>() == MAX_26_BITS
}


#[bench]
fn is_pangramyyy(b: &mut Bencher) {
    b.iter(|| {
        is_pangram(
            "Victor jagt zwölf_(12) Boxkämpfer quer über den großen Sylter Deich.",
        )
    });
}
