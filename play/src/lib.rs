use std::arch::x86_64;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = "1234 5678   8765 3215";
    // let digits = code.split_ascii_whitespace();
    // for digit in digits {
    //     println!("{}", digit);
    // }
    let digits: Vec<&u8> = code.as_bytes().iter().filter(|x| x).collect();
    println!("HEY {:?}", digits);
    todo!("Is the Luhn checksum for {code} valid?");
}