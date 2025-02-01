/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut checksum:u64 = 0;

    if code.len() < 2 || !code.is_ascii() { return false; }

    println!("CODE {}", code);

    let mut digits: Vec<u64> = code.as_bytes().iter().rev().filter(|x| **x != 32).map(|x| (*x - 48) as u64).collect();

    println!("\nDIGITS\n{:?}", digits);

    for i in 0..digits.len() {
        if !(48..58).contains(&digits[i]) { return false; }
        if i % 2 != 0 { 
            digits[i] *= 2;
            if digits[i] > 9 { digits[i] -= 9; }
        }
        checksum += digits[i];
    }

    println!("\nDIGITS\n{:?}\n", digits);
    println!("\nCHECKSUM {checksum}\n");

    checksum % 10 == 0
}