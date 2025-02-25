pub fn brackets_are_balanced(string: &str) -> bool {
    //let st = "())()";
    let mut p = Vec::new();
    let mut c = Vec::new();
    let mut b = Vec::new();

    for ch in string.char_indices() {
        //println!("{:?}", ch);
        match ch.1 {
            '(' => p.push(ch.0),
            '{' => c.push(ch.0),
            '[' => b.push(ch.0),
            _ => ()
        }
        if ch.1 == ')' {
            println!("PARENAAA = {:?}", p);
            let pp = p.pop();
            if pp.is_none() { return false; }
            println!("PP = {:?}", pp);
            println!("PARENBBB = {:?}", p);
            let cc = c.last().unwrap_or(&0);
            let bb = b.last().unwrap_or(&0);
            //if pp < *cc || pp < *bb { return false; } 
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

    // println!("PAREN = {:?}", p);
    // println!("CURLY = {:?}", c);
    // println!("BRACK = {:?}", b);

    todo!("HEY")
}
