// Fourth attempt incorporating auto suggestion

fn check_passed(c: char, bracket: char, v0: &mut Vec<usize>, v1: &[usize], v2: &[usize]) -> bool {
    if c == bracket {
        let x = match v0.pop() {
            Some(val) => val,
            None => return false
        };
        let y = v1.last().unwrap_or(&0);
        let z = v2.last().unwrap_or(&0);
        if x < *y || x < *z { return false; }
    }
    true
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut p = Vec::new();
    let mut c = Vec::new();
    let mut b = Vec::new();

    for ch in string.char_indices() {
        match ch.1 {
            '(' => p.push(ch.0),
            '{' => c.push(ch.0),
            '[' => b.push(ch.0),
            _ => ()
        }
        if !check_passed(ch.1, ')', &mut p, &c, &b) { return false; }
        if !check_passed(ch.1, '}', &mut c, &p, &b) { return false; }
        if !check_passed(ch.1, ']', &mut b, &c, &p) { return false; }
    }

    if !p.is_empty() || !c.is_empty() || !b.is_empty() { return false; }

    true
}
