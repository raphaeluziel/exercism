use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let key = -1;
    let array = &[-9, -5, -1, -1, 0, 3, 5, 9, 21, 70, 88, 909];

    let mut start:usize = array.len() / 2 - 1;
    let mut end = 0;

    match key.cmp(&array[start]) {
        Ordering::Equal => return Some(start),
        Ordering::Less => { let temp = start; start = end; end = temp; },
        Ordering::Greater => end = array.len()
    };
    
    println!("{start} - {end} Section1 = {:?}", &array[start..end]);

    start = end / 2 - 1;

    match key.cmp(&array[start]) {
        Ordering::Equal => return Some(start),
        Ordering::Less => { let temp = start; start = end; end = temp; },
        Ordering::Greater => end = array.len()
    };

    println!("{start} - {end} Section2 = {:?}", &array[start..end]);


    todo!(
        "Using the binary search algorithm, find the element '{key}' in the array '{array:?}' and return its index."
    );
}
