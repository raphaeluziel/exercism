pub fn encode(source: &str) -> String {
    let mut s: String = String::new();
    let mut letters = source.chars();
    //println!("letters = {:?}", letters);
    let mut n: u64 = 0;
    let mut curr = letters.next().unwrap_or_default();
    // println!("curr = {:?}", curr);
    // println!("letters = {:?}", letters);
    for letter in letters {
        //println!("letter = {:?}", letter);
        n += 1;
        if letter != curr {
            //println!("HHHHHHHHHHHHHHHHHHHHHHHHHHHHH {n}");
            if n > 1 { s.push_str(&n.to_string()) }
            println!("pushing {:?} with n = {n}", curr);
            s.push(curr);
            curr = letter;
            n = 0;
        }
    }
    println!("input  = {source}");
    println!("output = {:?}", s);
    s
    //todo!("Return the run-length encoding of {source}.");
}

pub fn decode(source: &str) -> String {
    todo!("Return the run-length decoding of {source}.");
}
