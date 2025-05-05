// First try.  Works, but is way too slow!

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    palindrome: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.palindrome
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    let s = s.as_bytes();
    s.iter().zip(s.iter().rev()).all(|(fr, bk)| fr == bk)
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max { return None; }

    let mut smallest_pal = Palindrome { palindrome: 0, factors: HashSet::new() };
    let mut largest_pal = Palindrome { palindrome: 0, factors: HashSet::new() };

    let mut smallest = 0;
    let mut largest = 0;

    for i in min..=max {
        for j in min..=max {
            let prod = i * j;
            if is_palindrome(prod) {
                if prod < smallest || smallest == 0 { smallest = prod; }
                if prod > largest { largest = prod; }
            } 
        }
    }

    if smallest == 0 || largest == 0 { return None; }

    smallest_pal.palindrome = smallest;
    largest_pal.palindrome = largest;

    for i in min..=max {
        for j in i..=max {
            let prod = i * j;
            if prod == largest { largest_pal.factors.insert((i, j)); }
            if prod == smallest { smallest_pal.factors.insert((i, j)); }
        }
    }

    Some((smallest_pal, largest_pal))
}