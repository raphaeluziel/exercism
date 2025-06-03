pub fn encode(source: &str) -> String {
    let mut s: String = String::new();
    if source.is_empty() { return s }

    let mut it = source.chars().peekable();
    let mut n = 0;

    while let Some(ch) = it.next() {
        n += 1;
        if Some(ch) != it.peek().copied() || it.peek().is_none() {
            if n > 1 { s.push_str(&n.to_string()); }
            s.push(ch);
            n = 0;
        }
    }

    s
}

pub fn decode(source: &str) -> String {
    let mut s: String = String::new();
    if source.is_empty() { return s; }

    let source = "12WB12W3B24WB";
    // WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB

    

    let ggg: Vec<_> = source.split(char::is_numeric).collect();
    let hhh: Vec<_> = source.split(char::is_alphabetic).collect();
    println!("ggg = {:?}\nhhh = {:?}", ggg, hhh);
    
    // let letters = source.chars();
    // let mut n = 0;
    // let mut power = 1;

    // for letter in letters {
    //     if letter.is_ascii_digit() {
    //         n = power * n + letter.to_digit(10).unwrap_or_default();
    //         power *= 10;   
    //     }
    //     else {
    //         if n == 0 {
    //             n = 1;
    //         }
    //         for _ in 0..n {
    //             s.push(letter);
    //         }
    //         power = 1;
    //         n = 0;
    //     } 
    // }

    s
}
