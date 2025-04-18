// This is my initial attempt with just the basics

use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 { return None; }
    if array.len() == 1 {
        if key == array[0] { return Some(0); }
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