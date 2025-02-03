pub fn is_valid(code: &str) -> bool {
    println!("\nOriginal code = {}", code);
    let mut checksum:u64 = 0;
    let mut digits:Vec<u8> = Vec::new();

    if code.len() < 2 || !code.is_ascii() { return false; }

    let bytes = code.as_bytes();

    for byte in bytes {
        match byte {
            32 => (),
            47..=57 => digits.push(*byte - 48),
            _ => return false,
        }
    }

    if bytes.len() < 2 { return false; }

    println!("\nDIGITS = {:?}", digits);

    for i in (0..digits.len()).rev() {
        println!("IIIIIIII {i}");
        //println!("digit[{}] = {}", i, digits[i]);
        if i % 2 == 0 {
            println!("HERE {}", digits[i]);
            checksum += if digits[i] < 5 { digits[i] as u64 * 2 } else { digits[i] as u64 * 2 - 9 };
            println!("{checksum}");
        }
        else {
            println!("NOT THIS ONE {}", digits[i]);
            checksum += digits[i] as u64;
            println!("{checksum}");
        }
    }

    //println!("DIGITS = {:?}\n", digits);
    println!("{checksum}");


    checksum % 10 == 0
}