pub fn encode(source: &str) -> String {
    let mut s: String = String::new();
    if source.is_empty() { return s; }

    let source = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBBBBBBBBBB";

    let mut n = 0;

    for x in source.as_bytes().windows(2) {
        println!("{:?}", x);
        n += 1;
        if x[0] != x[1] {
            if n > 1 { s.push_str(&n.to_string()); }
            s.push(x[0] as char);
            n = 0;
        }
    }

    // for x in source.as_bytes().windows(2).peekable() {
    //     println!("{:?}", x);
    //     if x[0] == x[1] {
    //         n += 1;
    //     }
    //     else {
    //         if n > 1 { s.push_str(&(n + 1).to_string()); }
    //         s.push(x[0] as char);
    //         n = 0;
    //     }
    // }


    println!("s = {:?}", s);

    // let mut letters = source.chars();
    // let mut n: u64 = 0;
    // let mut curr = letters.next().unwrap_or_default();

    // for letter in letters {
    //     n += 1;
    //     if letter != curr {
    //         if n > 1 { s.push_str(&n.to_string()) }
    //         s.push(curr);
    //         curr = letter;
    //         n = 0;
    //     }
    // }
    // n += 1;
    // if n > 1 { s.push_str(&n.to_string()) }
    // s.push(curr);
    s
}

pub fn decode(source: &str) -> String {
    let mut s: String = String::new();
    if source.is_empty() { return s; }
    
    let letters = source.chars();
    let mut n = 0;
    let mut power = 1;

    for letter in letters {
        if letter.is_ascii_digit() {
            n = power * n + letter.to_digit(10).unwrap_or_default();
            power *= 10;   
        }
        else {
            if n == 0 {
                n = 1;
            }
            for _ in 0..n {
                s.push(letter);
            }
            power = 1;
            n = 0;
        } 
    }

    s
}
