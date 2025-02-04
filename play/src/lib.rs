pub fn is_valid(code: &str) -> bool {
    println!("\nOriginal code = {}", code);
    let mut checksum: u64 = 0;
    let mut digits: Vec<u8> = Vec::new();

    if code.len() < 2 || !code.is_ascii() {
        return false;
    }

    let bytes= code.as_bytes();

    if bytes.len() < 2 {
        return false;
    }

    for byte in bytes {
        match byte {
            32 => (),
            47..=57 => digits.push(*byte - 48),
            _ => return false,
        }
    }

    if digits.len() < 2 { return false; }

    println!("\nDIGITS = {:?}", digits);
    digits.reverse();
    println!("DIGITS = {:?}", digits);

    // for i in 0..digits.len() {
    //     if i % 2 != 0 {
    //         println!("HERE I = {} VAL = {}", i, digits[i]);
    //         checksum += if digits[i] < 5 {
    //             digits[i] as u64 * 2
    //         } else {
    //             digits[i] as u64 * 2 - 9
    //         };
    //         println!("{checksum}");
    //     } else {
    //         println!("NOT THIS ONE {}", digits[i]);
    //         checksum += digits[i] as u64;
    //         println!("{checksum}");
    //     }
    // }

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

    //println!("DIGITS = {:?}\n", digits);
    println!("CHECKSUM {checksum}");

    checksum % 10 == 0
}
