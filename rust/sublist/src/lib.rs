// Check out https://doc.rust-lang.org/std/slice/struct.Windows.html
// This I found through community solutions, after submitting the one here.

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {

    fn is_set<T: PartialEq>(large: &[T], small: &[T], set: Comparison) -> Comparison {
        for i in 0..=(large.len() - small.len()) {
            let large_slice: &[T] = &large[i..(i + small.len())];
            if small == large_slice {
                return set;
            }
        }
        Comparison::Unequal
    }

    match _first_list.len().cmp(&_second_list.len()) {
        Ordering::Greater => is_set(_first_list, _second_list, Comparison::Superlist),
        Ordering::Less => is_set(_second_list, _first_list, Comparison::Sublist),
        Ordering::Equal => {
            if _first_list == _second_list { Comparison::Equal } 
            else { Comparison::Unequal }
        }
    }
}