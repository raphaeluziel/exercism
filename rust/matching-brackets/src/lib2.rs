// This second try uses what the automated suggestions stated

pub fn brackets_are_balanced(string: &str) -> bool {
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
            let pp = match p.pop() {
                Some(val) => val,
                None => return false
            };
            let cc = c.last().unwrap_or(&0);
            let bb = b.last().unwrap_or(&0);
            if pp < *cc || pp < *bb { return false; } 
        }
        if ch.1 == '}' {
            let cc = match c.pop() {
                Some(val) => val,
                None => return false
            };
            let pp = p.last().unwrap_or(&0);
            let bb = b.last().unwrap_or(&0);
            if cc < *pp || cc < *bb { return false; }
        }
        if ch.1 == ']' { 
            let bb = match b.pop() {
                Some(val) => val,
                None => return false
            };
            let cc = c.last().unwrap_or(&0);
            let pp = p.last().unwrap_or(&0);
            if bb < *pp || bb < *cc { return false; } 
        }
    }

    if !p.is_empty() || !c.is_empty() || !b.is_empty() { return false; }

    true
}
