#![feature(test)]

extern crate test;
// clear && cargo +nightly bench

use std::collections::HashSet;

const MAX_26_BITS: u32 = 67_108_863;

pub fn is_pangram(sentence: &str) -> bool {
    let hs:HashSet<_> = sentence
        .to_ascii_lowercase()
        .as_bytes()
        .iter()
        .filter(|&&x| x >= b'a' && x <= b'z')
        .map(|x| 2u32.pow((x - b'a') as u32))
        .collect();
    
    hs.iter().sum::<u32>() == MAX_26_BITS
}

////////////////////////////////////////////////////////////////////////////////

pub fn is_pangram_all_contains(sentence: &str) -> bool {
    let sentence_lowered = sentence.to_lowercase();
    ('a'..='z').all(|ltr| sentence_lowered.contains(ltr))
}

pub fn is_pangram_hash_is_subset(sentence: &str) -> bool {
    let all: HashSet<char> = HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
    let used: HashSet<char> = HashSet::from_iter(sentence.to_lowercase().chars());
    all.is_subset(&used)
}

pub fn is_pangram_hashset_len(sentence: &str) -> bool {
    sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<char>>()
        .len()
        == 26
}

const A_LCASE: u8 = 97;
const A_UCASE: u8 = 65;
const ALL_26_BITS_SET: u32 = 67108863;

pub fn is_pangram_bitfield(sentence: &str) -> bool {
    let mut letter_flags = 0;

    for letter in sentence.chars() {
        if letter >= 'a' && letter <= 'z' {
            letter_flags |= 1 << (letter as u8 - A_LCASE);
        } else if letter >= 'A' && letter <= 'Z' {
            letter_flags |= 1 << (letter as u8 - A_UCASE);
        }
    }
    letter_flags == ALL_26_BITS_SET
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_is_pangram(b: &mut Bencher) {
        b.iter(|| {
            is_pangram("Victor jagt zwölf_(12) Boxkämpfer quer über den großen Sylter Deich.")
        });
    }

    #[bench]
    fn bench_is_pangram_all_contains(b: &mut Bencher) {
        b.iter(|| {
            is_pangram_all_contains("Victor jagt zwölf_(12) Boxkämpfer quer über den großen Sylter Deich.")
        });
    }

    #[bench]
    fn bench_is_pangram_hash_is_subset(b: &mut Bencher) {
        b.iter(|| {
            is_pangram_hash_is_subset("Victor jagt zwölf_(12) Boxkämpfer quer über den großen Sylter Deich.")
        });
    }

    #[bench]
    fn bench_is_pangram_hashset_len(b: &mut Bencher) {
        b.iter(|| {
            is_pangram_hashset_len("Victor jagt zwölf_(12) Boxkämpfer quer über den großen Sylter Deich.")
        });
    }

    #[bench]
    fn bench_is_pangram_bitfield(b: &mut Bencher) {
        b.iter(|| {
            is_pangram_bitfield("Victor jagt zwölf_(12) Boxkämpfer quer über den großen Sylter Deich.")
        });
    }
}
