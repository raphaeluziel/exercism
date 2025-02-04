pub fn is_valid(code: &str) -> bool {
    let mut checksum: u64 = 0;
    let mut digits: Vec<u8> = Vec::new();

    if code.len() < 2 || !code.is_ascii() { return false; }

    let bytes= code.as_bytes();

    if bytes.len() < 2 { return false; }

    for byte in bytes {
        match byte {
            32 => (),
            47..=57 => digits.push(*byte - 48),
            _ => return false,
        }
    }

    if digits.len() < 2 { return false; }

    digits.reverse();

    for i in 0..digits.len() {
        if i % 2 != 0 {
            checksum += if digits[i] < 5 {
                digits[i] as u64 * 2
            } else {
                digits[i] as u64 * 2 - 9
            };
        } else {
            checksum += digits[i] as u64;
        }
    }
    checksum % 10 == 0
}