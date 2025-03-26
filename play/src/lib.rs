use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // find(&[1, 3, 4, 6, 8, 9, 11], 1), Some(0)

    if array.len() == 0 { println!("HERE"); return None; }
    if array.len() == 1 {
        if key == array[0] { return Some(0); }
        else { return None; }
    }

    println!("Array len = {}", array.len());
    let mut start:usize = 0;
    let mut end:usize = array.len() - 1;
    let mut mid:usize = (end - start) / 2;
    println!("(A) Start = {start}, mid = {mid}, end = {end}");

    

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
        println!("(B) Start = {start}, mid = {mid}, end = {end}");
    }
    
    None
}
