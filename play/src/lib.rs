#![feature(test)]

extern crate test;
// clear && cargo +nightly test && cargo +nightly bench

use std::collections::HashSet;

const MAX_26_BITS: u32 = 67_108_863;

pub fn is_pangram(sentence: &str) -> bool {
    let flags: u32 = 0;
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let alphabet_flags:Vec<_> = alphabet.char_indices().map(|(i, c)| (c, 2u32.pow(i as u32))).collect();

    println!("{:?}", alphabet_flags);

    // let hs:HashSet<_> = sentence
    //     .to_ascii_lowercase()
    //     .as_bytes()
    //     .iter()
    //     .filter(|&&x| x.is_ascii_lowercase())
    //     .map(|x| 2u32.pow((x - b'a') as u32))
    //     .collect();
    
    // hs.iter().sum::<u32>() == MAX_26_BITS
    todo!()
}


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
}