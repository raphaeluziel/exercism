/// The idea here is to use last 26 bits of a u32 as placeholders for the 
/// presence (1) or absence (0) of each of the 26 letters.
/// 
/// The main part is to convert the character in the sentence to a binary 
/// number with a single 1 bit in a particular place in a 32-bit number
/// (though only 26 bits are actually used).
/// 
/// So for example, 
/// the char 'a' will be 0000_0000_0000_0000_0000_0000_0000_0001
/// the char 'b' will be 0000_0000_0000_0000_0000_0000_0000_0010
///                                 :
///                                 :
///                                 :
/// the char 'z' will be 0000_0010_0000_0000_0000_0000_0000_0000
/// 
/// When added together, each letter will place a 1 bit in a particular place
/// in the binary, so if all of the last 26 bits are 1's, then that will mean
/// that all the letters are present. 

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
