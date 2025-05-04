use std::{collections::{BTreeSet, HashSet}, hash::Hash};

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

    let mut poss_prods: BTreeSet<u64> = BTreeSet::new();
    let mut smallest_pal = Palindrome { palindrome: 0, factors: HashSet::new() };
    let mut largest_pal = Palindrome { palindrome: 0, factors: HashSet::new() };
    let mut min_found = false;

    for i in min..=max {
        for j in i..=max {
            let prod = i * j;
            if prod == smallest_pal.palindrome { smallest_pal.factors.insert((i, j)); }
            else if prod == largest_pal.palindrome { largest_pal.factors.insert((i, j)); }
            let inserted = poss_prods.insert(prod);
            let a_palindrome = is_palindrome(prod);
            if inserted && a_palindrome {
                if !min_found {
                    smallest_pal.palindrome = prod;
                    smallest_pal.factors.insert((i, j));
                    min_found = true;
                }
                else {
                    largest_pal.palindrome = prod;
                    largest_pal.factors.insert((i, j));
                }
            }
            //println!("S = {:?}, \nL = {:?}\n\n", smallest_pal, largest_pal);
        }
    }

    if smallest_pal.palindrome == 0 || largest_pal.palindrome == 0 { return  None; }

    Some((smallest_pal, largest_pal))
}
