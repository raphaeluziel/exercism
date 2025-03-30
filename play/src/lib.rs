// Final iteration.  
// Passes all tests including the generic tests using:
// cargo test --features generic

use std::cmp::Ordering;

// The U: AsRef<[T]> allows the function to accept references, or owned items
// The T: PartialEq + Ord is needed to only allow keys that can be ordered / compared
pub fn find<U: AsRef<[T]>, T: PartialEq + Ord>(array: U, key: T) -> Option<usize>
{
    // This converts an owned to a reference, but does nothing if already
    // a reference.  At least this is what I think it does.
    let array = array.as_ref();
    
    // An empty array cannot contain the key
    if array.is_empty() { return None; }

    // These three indices will keep moving to narrow down
    // the part of the slice that must contain the key
    let mut start:usize = 0;
    let mut end:usize = array.len() - 1;
    let mut mid:usize = (end - start) / 2;

    // Check if the key can be found at the start or end of the slice
    if key == array[start] { return Some(start); }
    if key == array[end] { return  Some(end); }

    // This loop incorporates the binary search algorithm
    // as described in the instructions
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

    // If the loop above doesn't find the key, then it aint there!
    None
}
