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

    for (i, digit) in digits.iter().enumerate() {
        if i % 2 != 0 {
            checksum += if *digit < 5 {
                *digit as u64 * 2
            } else {
                *digit as u64 * 2 - 9
            };
        } else {
            checksum += *digit as u64;
        }
    }

    checksum % 10 == 0
}