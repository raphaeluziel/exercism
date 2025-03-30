// This is the generic version of the same
// Run the following to test out the optional generic function tests:
// cargo test --features generic

use std::{cmp::Ordering, ops::Index};

pub fn find<U: AsRef<[T]>, T>(array: U, key: T) -> Option<usize>
    where T: PartialEq + Ord,
{
    let array = array.as_ref();
    
    if array.is_empty() { return None; }
    if array.len() == 1 {
        if key == *array.as_ref().index(0) { return Some(0); }
        else { return None; }
    }

    let mut start:usize = 0;
    let mut end:usize = array.len() - 1;
    let mut mid:usize = (end - start) / 2;

    if key == array[start] { return Some(start); }
    if key == array[end] { return  Some(end); }

    while mid != start && mid != end {
        match key.cmp(&array[mid]) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => { 
                end = mid;
                mid -= (end - start) / 2;
            },
            Ordering::Greater => {
                start = mid;
                mid += (end - start) / 2;
            }
        };
    }
    
    None
}
