pub fn brackets_are_balanced(string: &str) -> bool {
    let st = ")()";
    let mut p = Vec::new();
    let mut c = Vec::new();
    let mut b = Vec::new();

    for ch in st.char_indices() {
        //println!("{:?}", ch);
        match ch.1 {
            '(' => p.push(ch.0),
            '{' => c.push(ch.0),
            '[' => b.push(ch.0),
            _ => ()
        }
        if ch.1 == ')' { 
            let pp = p.pop().unwrap_or(0);
            println!("PP = {:?}", pp);
            let cc = c.last().unwrap_or(&0);
            let bb = b.last().unwrap_or(&0);
            if pp < *cc || pp < *bb { return false; } 
        }
        if ch.1 == '}' {
            let cc = c.pop().unwrap_or(0);
            let pp = p.last().unwrap_or(&0);
            let bb = b.last().unwrap_or(&0);
            if cc < *pp || cc < *bb { return false; }
        }
        if ch.1 == ']' { 
            let bb = b.pop().unwrap_or(0);
            let cc = c.last().unwrap_or(&0);
            let pp = p.last().unwrap_or(&0);
            if bb < *pp || bb < *cc { return false; } 
        }
    }

    println!("PAREN = {:?}", p.iter().max().unwrap_or(&0));
    println!("CURLY = {:?}", c.iter().max().unwrap_or(&0));
    println!("BRACK = {:?}", b.iter().max().unwrap_or(&0));

    todo!("HEY")
}
