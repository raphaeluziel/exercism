// Sixth, and I think my final attempt at this!

pub fn brackets_are_balanced(string: &str) -> bool {

    #[derive(Debug)]
    struct Bracket {
        opening: char,
        closing: char,
        locations: Vec<usize>,
        order: (usize, usize, usize)
    }

    let p = Bracket{ opening: '(', closing: ')', locations:vec![], order: (0, 1, 2) };
    let c = Bracket{ opening: '{', closing: '}', locations:vec![], order: (1, 2, 0) };
    let b = Bracket{ opening: '[', closing: ']', locations:vec![], order: (2, 0, 1) };

    let bracket_arr = [p, c, b];

    for ch in string.char_indices() {
        for mut bracket in &bracket_arr {
            if ch.1 == bracket.opening {
                bracket.locations.push(ch.0);
            }

        }
    }

    

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

    todo!("HHH")
}