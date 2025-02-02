// fn is_a_number() -> bool {
//     let s = "§¥";
//     let s = "hello my friend";
//     //let s = "9846";
//     println!("\n\nYOYO {} NUMBER {}\n\n", s, s.is_ascii());

//     let x = s.as_bytes();
//     println!("X {:?}", x);

//     false
// }

pub fn is_valid(code: &str) -> bool {
    //let code = "95";
    let mut checksum:u64 = 0;

    if code.len() < 2 || !code.is_ascii() { return false; }

    println!("CODIE {}", code);

    let mut digits: Vec<u64> = code.as_bytes().iter().filter(|x| **x != 32).map(|x| (*x - 48) as u64).collect();

    println!("\nDIGITS\n{:?}", digits);

    for i in 0..digits.len() {
        //if !(48..58).contains(&digits[i]) { return false; }
        if i % 2 == 0 { 
            digits[i] *= 2;
            if digits[i] > 9 { digits[i] -= 9; }
        }
        checksum += digits[i];
    }

    println!("\nDIGITS\n{:?}\n", digits);
    println!("\nCHECKSUM {checksum}\n");

    println!("RESULT {}", checksum % 10 == 0);

    checksum % 10 == 0
}