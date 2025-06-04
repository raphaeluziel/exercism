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
    
    let mut n = 0;
    let mut power = 1;

    for letter in source.chars() {
        if let Some(digit) = letter.to_digit(10)  {
            n = power * n + digit;
            power *= 10;   
        }
        else {
            if n == 0 { n = 1; }
            for _ in 0..n { s.push(letter); }
            power = 1;
            n = 0;
        } 
    }

    s
}