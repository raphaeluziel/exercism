// Fifth attempt but not sure ir this is any better

pub fn brackets_are_balanced(string: &str) -> bool {
    let openings = ['(', '{', '['];
    let closings = [')', '}', ']'];
    let order = [(0, 1, 2), (1, 2, 0), (2, 0, 1)];
    let mut locations = vec![Vec::new(), Vec::new(), Vec::new()];

    for ch in string.char_indices() {
        for i in 0..3 {
            if ch.1 == openings[i] {
                locations[i].push(ch.0);
            }
            else if ch.1 == closings[i] {
                let x = match locations[order[i].0].pop() {
                    Some(val) => val,
                    None => return false
                };
                let y = locations[order[i].1].last().unwrap_or(&0);
                let z = locations[order[i].2].last().unwrap_or(&0);
                if x < *y || x < *z { return false; }
            }
        }
    } 

    for location in locations {
        if !location.is_empty() { return false; }
    }

    true
}
