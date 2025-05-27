pub fn encode(source: &str) -> String {
    let mut s: String = String::new();
    if source.is_empty() { return s; }
    
    let mut letters = source.chars();
    let mut n: u64 = 0;
    let mut curr = letters.next().unwrap_or_default();

    for letter in letters {
        n += 1;
        if letter != curr {
            if n > 1 { s.push_str(&n.to_string()) }
            s.push(curr);
            curr = letter;
            n = 0;
        }
    }
    n += 1;
    if n > 1 { s.push_str(&n.to_string()) }
    s.push(curr);
    println!("input  = {source}");
    println!("output = {:?}", s);
    s
}

pub fn decode(source: &str) -> String {
    let mut s: String = String::new();
    if source.is_empty() { return s; }

    let mut letters = source.chars();
    let mut n: u64 = 0;
    let mut curr = letters.next().unwrap_or_default();
    let mut prev = false;
    let mut power = 1;

    for letter in letters {
        if letter.is_ascii_digit() {
            if prev {
                power *= 10;
                n += power * letter.to_digit(10).unwrap_or_default();
            }
            

        }
    }

    todo!("Return the run-length decoding of {source}.");
}
