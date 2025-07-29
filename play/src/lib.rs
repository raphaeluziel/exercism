pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = 7;

    let mut v = vec![vec![0u32; size as usize]; size as usize];

    if size == 0 {
        return v;
    }
    if size == 1 {
        return vec![vec![1]];
    }

    let last = size * size;
    let mut count = 1usize;
    let mut start = 0usize;
    let mut end = size as usize;
    let mut rev = false;

    while count < last {
        
    }

    todo!()
}
